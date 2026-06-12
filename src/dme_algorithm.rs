//! Deferred Merge Embedding (DME) algorithm for clock tree synthesis.
//!
//! Implements the DME algorithm for constructing zero-skew clock trees
//! with Manhattan geometry. Supports both linear and Elmore delay models.
//!
//! Nodes are stored in an arena (`Tree`) and referenced by `usize` index,
//! avoiding `Rc<RefCell<>>` overhead.

use std::collections::HashMap;

use crate::generic::MinDist;
use crate::interval::Interval;
use crate::manhattan_arc::ManhattanArc;
use crate::point::Point;

/// A clock sink with name, position, and capacitance.
#[derive(Debug, Clone)]
pub struct Sink {
    pub name: String,
    pub position: Point<i32, i32>,
    pub capacitance: f64,
}

impl Sink {
    pub fn new(name: &str, position: Point<i32, i32>, capacitance: f64) -> Self {
        Sink { name: name.to_string(), position, capacitance }
    }
}

/// Node index used throughout the DME algorithm.
pub type NodeIdx = usize;

/// A node in the clock tree, stored in a `Tree` arena.
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub name: String,
    pub position: Point<i32, i32>,
    pub left: Option<NodeIdx>,
    pub right: Option<NodeIdx>,
    pub parent: Option<NodeIdx>,
    pub wire_length: i32,
    pub delay: f64,
    pub capacitance: f64,
    pub need_elongation: bool,
}

impl TreeNode {
    pub fn new(name: &str, position: Point<i32, i32>) -> Self {
        TreeNode {
            name: name.to_string(),
            position,
            left: None,
            right: None,
            parent: None,
            wire_length: 0,
            delay: 0.0,
            capacitance: 0.0,
            need_elongation: false,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

/// Arena-allocated tree of `TreeNode`s.
///
/// Nodes are stored in a `Vec` and referenced by their index.
/// This avoids `Rc<RefCell<>>` while still allowing safe mutation
/// during bottom-up merging and top-down embedding phases.
#[derive(Debug, Clone, Default)]
pub struct Tree {
    nodes: Vec<TreeNode>,
    pub root: Option<NodeIdx>,
}

impl Tree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, node: TreeNode) -> NodeIdx {
        let idx = self.nodes.len();
        self.nodes.push(node);
        idx
    }

    pub fn get(&self, idx: NodeIdx) -> &TreeNode {
        &self.nodes[idx]
    }

    pub fn get_mut(&mut self, idx: NodeIdx) -> &mut TreeNode {
        &mut self.nodes[idx]
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &TreeNode> {
        self.nodes.iter()
    }

    /// Simultaneously access two distinct nodes by index (safe, checked).
    pub fn get_pair_mut(&mut self, a: NodeIdx, b: NodeIdx) -> (&mut TreeNode, &mut TreeNode) {
        assert_ne!(a, b, "get_pair_mut called with identical indices");
        if a < b {
            let (left, right) = self.nodes.split_at_mut(b);
            (&mut left[a], &mut right[0])
        } else {
            let (left, right) = self.nodes.split_at_mut(a);
            (&mut right[0], &mut left[b])
        }
    }
}

/// Abstract delay model for wire delay calculation.
pub trait DelayCalculator {
    fn calculate_wire_delay(&self, length: i32, load_capacitance: f64) -> f64;
    fn calculate_wire_delay_per_unit(&self, load_capacitance: f64) -> f64;
    fn calculate_wire_capacitance(&self, length: i32) -> f64;
    /// Compute tapping point from delay/capacitance values (no &mut TreeNode needed).
    fn calculate_tapping_point(
        &self,
        distance: i32,
        left_delay: f64,
        right_delay: f64,
        left_capacitance: f64,
        right_capacitance: f64,
    ) -> (i32, f64);
}

/// Linear delay model: delay = k * length.
pub struct LinearDelayCalculator {
    pub delay_per_unit: f64,
    pub capacitance_per_unit: f64,
}

impl LinearDelayCalculator {
    pub fn new(delay_per_unit: f64, capacitance_per_unit: f64) -> Self {
        LinearDelayCalculator { delay_per_unit, capacitance_per_unit }
    }
}

impl DelayCalculator for LinearDelayCalculator {
    fn calculate_wire_delay(&self, length: i32, _load_capacitance: f64) -> f64 {
        self.delay_per_unit * length as f64
    }
    fn calculate_wire_delay_per_unit(&self, _load_capacitance: f64) -> f64 {
        self.delay_per_unit
    }
    fn calculate_wire_capacitance(&self, length: i32) -> f64 {
        self.capacitance_per_unit * length as f64
    }
    fn calculate_tapping_point(
        &self,
        distance: i32,
        left_delay: f64,
        right_delay: f64,
        _left_capacitance: f64,
        _right_capacitance: f64,
    ) -> (i32, f64) {
        if distance == 0 {
            return (0, left_delay.max(right_delay));
        }
        let skew = right_delay - left_delay;
        let extend_left = ((skew / self.delay_per_unit + distance as f64) / 2.0).round() as i32;
        let delay_left = left_delay + extend_left as f64 * self.delay_per_unit;
        let extend_left = extend_left.clamp(0, distance);
        let delay_left = if extend_left == 0 {
            left_delay
        } else if extend_left == distance {
            right_delay
        } else {
            delay_left
        };
        (extend_left, delay_left)
    }
}

/// Elmore delay model: considers wire resistance and capacitance.
pub struct ElmoreDelayCalculator {
    pub unit_resistance: f64,
    pub unit_capacitance: f64,
}

impl ElmoreDelayCalculator {
    pub fn new(unit_resistance: f64, unit_capacitance: f64) -> Self {
        ElmoreDelayCalculator { unit_resistance, unit_capacitance }
    }
}

impl DelayCalculator for ElmoreDelayCalculator {
    fn calculate_wire_delay(&self, length: i32, load_capacitance: f64) -> f64 {
        let r = self.unit_resistance * length as f64;
        let c = self.unit_capacitance * length as f64;
        r * (c / 2.0 + load_capacitance)
    }
    fn calculate_wire_delay_per_unit(&self, load_capacitance: f64) -> f64 {
        self.unit_resistance * (self.unit_capacitance / 2.0 + load_capacitance)
    }
    fn calculate_wire_capacitance(&self, length: i32) -> f64 {
        self.unit_capacitance * length as f64
    }
    fn calculate_tapping_point(
        &self,
        distance: i32,
        left_delay: f64,
        right_delay: f64,
        left_capacitance: f64,
        right_capacitance: f64,
    ) -> (i32, f64) {
        if distance == 0 {
            return (0, left_delay.max(right_delay));
        }
        let skew = right_delay - left_delay;
        let r = distance as f64 * self.unit_resistance;
        let c = distance as f64 * self.unit_capacitance;
        let z = (skew + r * (right_capacitance + c / 2.0))
            / (r * (c + right_capacitance + left_capacitance));
        let extend_left = (z * distance as f64).round() as i32;
        let r_left = extend_left as f64 * self.unit_resistance;
        let c_left = extend_left as f64 * self.unit_capacitance;
        let delay_left = left_delay + r_left * (c_left / 2.0 + left_capacitance);
        let extend_left = extend_left.clamp(0, distance);
        let delay_left = if extend_left == 0 {
            left_delay
        } else if extend_left == distance {
            right_delay
        } else {
            delay_left
        };
        (extend_left, delay_left)
    }
}

/// Results of clock skew analysis.
#[derive(Debug, Clone)]
pub struct SkewAnalysis {
    pub max_delay: f64,
    pub min_delay: f64,
    pub skew: f64,
    pub sink_delays: Vec<f64>,
    pub total_wirelength: i32,
    pub delay_model: String,
}

/// Detailed tree statistics.
#[derive(Debug, Clone)]
pub struct TreeStatistics {
    pub nodes: Vec<NodeInfo>,
    pub wires: Vec<WireInfo>,
    pub sinks: Vec<String>,
    pub total_nodes: i32,
    pub total_sinks: i32,
    pub total_wires: i32,
}

#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub name: String,
    pub position: (i32, i32),
    pub node_type: String,
    pub delay: f64,
    pub capacitance: f64,
}

#[derive(Debug, Clone)]
pub struct WireInfo {
    pub from_node: String,
    pub to_node: String,
    pub length: i32,
    pub from_pos: (i32, i32),
    pub to_pos: (i32, i32),
}

// ---------------------------------------------------------------------------
// DME algorithm
// ---------------------------------------------------------------------------

/// The DME algorithm for clock tree synthesis.
///
/// Builds a prescribed-skew clock tree using an arena-allocated node
/// representation (`Tree`). The constructed tree is retained and can be
/// queried via `get_tree()`.
pub struct DMEAlgorithm {
    sinks: Vec<Sink>,
    delay_calculator: Box<dyn DelayCalculator>,
    node_id: i32,
    source: Option<Point<i32, i32>>,
    tree: Tree,
}

impl DMEAlgorithm {
    pub fn new(sinks: Vec<Sink>, calculator: Box<dyn DelayCalculator>) -> Self {
        assert!(!sinks.is_empty(), "No sinks provided");
        DMEAlgorithm { sinks, delay_calculator: calculator, node_id: 0, source: None, tree: Tree::new() }
    }

    pub fn with_source(
        sinks: Vec<Sink>,
        calculator: Box<dyn DelayCalculator>,
        source: Point<i32, i32>,
    ) -> Self {
        assert!(!sinks.is_empty(), "No sinks provided");
        DMEAlgorithm { sinks, delay_calculator: calculator, node_id: 0, source: Some(source), tree: Tree::new() }
    }

    /// Returns a reference to the constructed tree.
    pub fn get_tree(&self) -> &Tree {
        &self.tree
    }

    pub fn get_tree_mut(&mut self) -> &mut Tree {
        &mut self.tree
    }

    /// Builds the clock tree and returns the root index.
    pub fn build_clock_tree(&mut self) -> NodeIdx {
        self.node_id = 0;
        self.tree = Tree::new();

        for s in &self.sinks {
            let mut node = TreeNode::new(&s.name, s.position);
            node.capacitance = s.capacitance;
            self.tree.add(node);
        }

        let leaf_indices: Vec<NodeIdx> = (0..self.tree.len()).collect();
        let root = self.build_merging_tree(&leaf_indices, false);

        let mut merging_segments: HashMap<NodeIdx, ManhattanArc<Interval<i32>>> = HashMap::new();
        self.compute_merging_segment(root, &mut merging_segments);
        self.embed_node(root, None, &merging_segments);
        self.compute_delays(root, 0.0);

        self.tree.root = Some(root);
        root
    }

    /// Build a balanced merging tree by recursive bipartition.
    fn build_merging_tree(&mut self, node_ids: &[NodeIdx], vertical: bool) -> NodeIdx {
        if node_ids.len() == 1 {
            return node_ids[0];
        }

        let mut sorted: Vec<NodeIdx> = node_ids.to_vec();
        if vertical {
            sorted.sort_by(|&a, &b| self.tree.get(a).position.xcoord.cmp(&self.tree.get(b).position.xcoord));
        } else {
            sorted.sort_by(|&a, &b| self.tree.get(a).position.ycoord.cmp(&self.tree.get(b).position.ycoord));
        }

        let mid = sorted.len() / 2;
        let left_child = self.build_merging_tree(&sorted[..mid], !vertical);
        let right_child = self.build_merging_tree(&sorted[mid..], !vertical);

        let id = format!("n{}", self.node_id);
        self.node_id += 1;
        let pos = self.tree.get(left_child).position;
        let parent_idx = self.tree.add(TreeNode::new(&id, pos));

        self.tree.get_mut(parent_idx).left = Some(left_child);
        self.tree.get_mut(parent_idx).right = Some(right_child);
        self.tree.get_mut(left_child).parent = Some(parent_idx);
        self.tree.get_mut(right_child).parent = Some(parent_idx);

        parent_idx
    }

    fn compute_merging_segment(
        &mut self,
        node: NodeIdx,
        segments: &mut HashMap<NodeIdx, ManhattanArc<Interval<i32>>>,
    ) -> ManhattanArc<Interval<i32>> {
        if self.tree.get(node).is_leaf() {
            let pos = self.tree.get(node).position;
            let ms1 = ManhattanArc::from_point(pos);
            let ms = ManhattanArc::new(
                Interval::new(ms1.xcoord(), ms1.xcoord()),
                Interval::new(ms1.ycoord(), ms1.ycoord()),
            );
            segments.insert(node, ms);
            return ms;
        }

        let left = self.tree.get(node).left.expect("Internal node missing left child");
        let right = self.tree.get(node).right.expect("Internal node missing right child");

        let left_ms = self.compute_merging_segment(left, segments);
        let right_ms = self.compute_merging_segment(right, segments);

        let distance = left_ms.min_dist_with(&right_ms) as i32;

        let (left_delay, right_delay) = {
            let ln = self.tree.get(left);
            let rn = self.tree.get(right);
            (ln.delay, rn.delay)
        };
        let (left_cap, right_cap) = {
            let ln = self.tree.get(left);
            let rn = self.tree.get(right);
            (ln.capacitance, rn.capacitance)
        };

        let (extend_left, delay_left) = self.delay_calculator.calculate_tapping_point(
            distance, left_delay, right_delay, left_cap, right_cap,
        );

        {
            let (l_node, r_node) = self.tree.get_pair_mut(left, right);
            l_node.wire_length = extend_left;
            r_node.wire_length = distance - extend_left;
            if extend_left == 0 {
                r_node.need_elongation = true;
            } else if extend_left == distance {
                l_node.need_elongation = true;
            }
        }

        self.tree.get_mut(node).delay = delay_left;

        let merged_segment = left_ms.merge_with(&right_ms, extend_left);
        segments.insert(node, merged_segment);

        let wire_cap = self.delay_calculator.calculate_wire_capacitance(distance);
        self.tree.get_mut(node).capacitance = {
            let lc = self.tree.get(left).capacitance;
            let rc = self.tree.get(right).capacitance;
            lc + rc + wire_cap
        };

        merged_segment
    }

    fn embed_node(
        &mut self,
        node: NodeIdx,
        parent_segment: Option<&ManhattanArc<Interval<i32>>>,
        segments: &HashMap<NodeIdx, ManhattanArc<Interval<i32>>>,
    ) {
        let node_segment = segments.get(&node).expect("Merging segment not found for node");

        if parent_segment.is_none() {
            if let Some(src) = self.source {
                let nearest = node_segment.nearest_point_to(&src);
                self.tree.get_mut(node).position = nearest;
            } else {
                let upper = node_segment.get_upper_corner();
                self.tree.get_mut(node).position = upper;
            }
        } else {
            let parent_pos = self.tree.get(node).parent.map(|p| self.tree.get(p).position);
            if let Some(pp) = parent_pos {
                let nearest = node_segment.nearest_point_to(&pp);
                self.tree.get_mut(node).position = nearest;
                let dist = self.tree.get(node).position.min_dist_with(&pp) as i32;
                self.tree.get_mut(node).wire_length = dist;
            }
        }

        let left = self.tree.get(node).left;
        let right = self.tree.get(node).right;

        if let Some(l) = left {
            self.embed_node(l, Some(node_segment), segments);
        }
        if let Some(r) = right {
            self.embed_node(r, Some(node_segment), segments);
        }
    }

    fn compute_delays(&mut self, node: NodeIdx, parent_delay: f64) {
        let has_parent = self.tree.get(node).parent.is_some();
        if has_parent {
            let wl = self.tree.get(node).wire_length;
            let cap = self.tree.get(node).capacitance;
            let wire_delay = self.delay_calculator.calculate_wire_delay(wl, cap);
            self.tree.get_mut(node).delay = parent_delay + wire_delay;
        } else {
            self.tree.get_mut(node).delay = 0.0;
        }

        let current_delay = self.tree.get(node).delay;
        let left = self.tree.get(node).left;
        let right = self.tree.get(node).right;

        if let Some(l) = left {
            self.compute_delays(l, current_delay);
        }
        if let Some(r) = right {
            self.compute_delays(r, current_delay);
        }
    }

    /// Analyze clock skew from the constructed tree.
    pub fn analyze_skew(&self, root: NodeIdx) -> SkewAnalysis {
        let mut sink_delays = Vec::new();
        collect_sink_delays(&self.tree, root, &mut sink_delays);

        if sink_delays.is_empty() {
            panic!("No sink delays collected");
        }

        let max_delay = sink_delays.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_delay = sink_delays.iter().cloned().fold(f64::INFINITY, f64::min);
        let skew = max_delay - min_delay;
        let total_wl = total_wirelength(&self.tree, root);
        #[allow(clippy::incompatible_msrv)]
        let delay_model = std::any::type_name_of_val(&*self.delay_calculator).to_string();

        SkewAnalysis { max_delay, min_delay, skew, sink_delays, total_wirelength: total_wl, delay_model }
    }
}

// ---------------------------------------------------------------------------
// Free helper functions (work with &Tree + NodeIdx)
// ---------------------------------------------------------------------------

fn collect_sink_delays(tree: &Tree, node: NodeIdx, sink_delays: &mut Vec<f64>) {
    if tree.get(node).is_leaf() {
        sink_delays.push(tree.get(node).delay);
    }
    if let Some(l) = tree.get(node).left {
        collect_sink_delays(tree, l, sink_delays);
    }
    if let Some(r) = tree.get(node).right {
        collect_sink_delays(tree, r, sink_delays);
    }
}

fn total_wirelength(tree: &Tree, node: NodeIdx) -> i32 {
    let mut total = tree.get(node).wire_length;
    if let Some(l) = tree.get(node).left {
        total += total_wirelength(tree, l);
    }
    if let Some(r) = tree.get(node).right {
        total += total_wirelength(tree, r);
    }
    total
}

/// Extracts detailed statistics from a clock tree.
pub fn get_tree_statistics(tree: &Tree, root: NodeIdx) -> TreeStatistics {
    let mut stats = TreeStatistics {
        nodes: Vec::new(),
        wires: Vec::new(),
        sinks: Vec::new(),
        total_nodes: 0,
        total_sinks: 0,
        total_wires: 0,
    };
    traverse_tree(tree, root, None, &mut stats);
    stats.total_nodes = stats.nodes.len() as i32;
    stats.total_sinks = stats.sinks.len() as i32;
    stats.total_wires = stats.wires.len() as i32;
    stats
}

fn traverse_tree(
    tree: &Tree,
    node: NodeIdx,
    parent: Option<NodeIdx>,
    stats: &mut TreeStatistics,
) {
    let n = tree.get(node);
    stats.nodes.push(NodeInfo {
        name: n.name.clone(),
        position: (n.position.xcoord, n.position.ycoord),
        node_type: if n.is_leaf() { "sink".to_string() } else { "internal".to_string() },
        delay: n.delay,
        capacitance: n.capacitance,
    });

    if n.is_leaf() {
        stats.sinks.push(n.name.clone());
    }

    if let Some(p) = parent {
        let pb = tree.get(p);
        stats.wires.push(WireInfo {
            from_node: pb.name.clone(),
            to_node: n.name.clone(),
            length: n.wire_length,
            from_pos: (pb.position.xcoord, pb.position.ycoord),
            to_pos: (n.position.xcoord, n.position.ycoord),
        });
    }

    let left = n.left;
    let right = n.right;
    let _ = n;

    if let Some(l) = left {
        traverse_tree(tree, l, Some(node), stats);
    }
    if let Some(r) = right {
        traverse_tree(tree, r, Some(node), stats);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_sinks(count: i32) -> Vec<Sink> {
        (0..count)
            .map(|i| {
                let x = (i * 37) % 100;
                let y = (i * 53) % 100;
                Sink::new(&format!("s{}", i), Point::new(x, y), 1.0 + (i % 5) as f64 * 0.2)
            })
            .collect()
    }

    fn run_tree(sinks: Vec<Sink>, calc: Box<dyn DelayCalculator>) -> (DMEAlgorithm, SkewAnalysis) {
        let mut dme = DMEAlgorithm::new(sinks, calc);
        let root = dme.build_clock_tree();
        let analysis = dme.analyze_skew(root);
        (dme, analysis)
    }

    #[test]
    fn test_dme_skew_within_two_percent_linear() {
        let sinks = make_sinks(8);
        let calc = Box::new(LinearDelayCalculator::new(0.5, 0.1));
        let (_, analysis) = run_tree(sinks, calc);
        assert!(analysis.max_delay > 0.0);
        let pct = analysis.skew / analysis.max_delay * 100.0;
        assert!(pct < 2.0, "Skew {:.4} ({:.2}%) exceeds 2%", analysis.skew, pct);
    }

    #[test]
    fn test_dme_skew_within_two_percent_elmore() {
        let sinks = make_sinks(8);
        let calc = Box::new(ElmoreDelayCalculator::new(0.1, 0.1));
        let (_, analysis) = run_tree(sinks, calc);
        assert!(analysis.max_delay > 0.0);
        let pct = analysis.skew / analysis.max_delay * 100.0;
        assert!(pct < 2.0, "Skew {:.4} ({:.2}%) exceeds 2%", analysis.skew, pct);
    }

    #[test]
    fn test_dme_two_sinks_zero_skew() {
        let sinks = vec![
            Sink::new("s1", Point::new(0, 0), 1.0),
            Sink::new("s2", Point::new(10, 0), 1.0),
        ];
        let (_, analysis) = run_tree(sinks, Box::new(LinearDelayCalculator::new(1.0, 0.1)));
        assert_eq!(analysis.skew, 0.0, "Two symmetric sinks should have zero skew");
        assert_eq!(analysis.total_wirelength, 10);
    }

    #[test]
    fn test_dme_single_sink() {
        let sinks = vec![Sink::new("s1", Point::new(5, 5), 1.0)];
        let (dme, analysis) = run_tree(sinks, Box::new(LinearDelayCalculator::new(0.5, 0.1)));
        assert!(dme.get_tree().get(dme.get_tree().root.unwrap()).is_leaf());
        assert_eq!(analysis.skew, 0.0);
        assert_eq!(analysis.sink_delays.len(), 1);
    }

    #[test]
    fn test_dme_with_source() {
        let sinks = vec![
            Sink::new("s1", Point::new(-10, -10), 1.0),
            Sink::new("s2", Point::new(10, -10), 1.0),
            Sink::new("s3", Point::new(-10, 10), 1.0),
            Sink::new("s4", Point::new(10, 10), 1.0),
        ];
        let calc = Box::new(LinearDelayCalculator::new(0.5, 0.1));
        let mut dme = DMEAlgorithm::with_source(sinks, calc, Point::new(0, 0));
        let root = dme.build_clock_tree();
        let analysis = dme.analyze_skew(root);
        assert!(analysis.max_delay > 0.0);
        let pct = analysis.skew / analysis.max_delay * 100.0;
        assert!(pct < 2.0, "Skew {:.4} ({:.2}%) exceeds 2%", analysis.skew, pct);
    }

    #[test]
    fn test_get_tree_statistics() {
        let sinks = vec![
            Sink::new("s1", Point::new(0, 0), 1.0),
            Sink::new("s2", Point::new(10, 0), 1.0),
        ];
        let mut dme = DMEAlgorithm::new(sinks, Box::new(LinearDelayCalculator::new(1.0, 0.1)));
        let root = dme.build_clock_tree();
        let stats = get_tree_statistics(dme.get_tree(), root);
        assert_eq!(stats.total_nodes, 3);
        assert_eq!(stats.total_sinks, 2);
        assert_eq!(stats.total_wires, 2);
    }
}
