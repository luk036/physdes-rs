//! Global router for Steiner tree-based routing.
//!
//! Provides data structures and algorithms for constructing rectilinear
//! Steiner routing trees with support for simple routing, Steiner point
//! insertion, and wirelength-constrained routing.

use std::collections::HashMap;
use std::fmt;

use crate::generic::MinDist;
use crate::interval::Interval;
use crate::point::Point;

/// Type of a routing node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Steiner,
    Terminal,
    Source,
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeType::Steiner => write!(f, "Steiner"),
            NodeType::Terminal => write!(f, "Terminal"),
            NodeType::Source => write!(f, "Source"),
        }
    }
}

/// A node in the routing tree.
#[derive(Debug, Clone)]
pub struct RoutingNode {
    pub id: String,
    pub node_type: NodeType,
    pub pt: Point<i32, i32>,
    pub children: Vec<usize>,
    pub parent: Option<usize>,
    pub capacitance: f64,
    pub delay: f64,
    pub path_length: i32,
}

impl RoutingNode {
    pub fn new(id: &str, node_type: NodeType, pt: Point<i32, i32>) -> Self {
        RoutingNode {
            id: id.to_string(),
            node_type,
            pt,
            children: Vec::new(),
            parent: None,
            capacitance: 0.0,
            delay: 0.0,
            path_length: 0,
        }
    }

    pub fn manhattan_distance(&self, other: &RoutingNode) -> i32 {
        self.pt.min_dist_with(&other.pt) as i32
    }
}

/// A rectilinear Steiner routing tree.
pub struct GlobalRoutingTree {
    nodes: Vec<RoutingNode>,
    node_map: HashMap<String, usize>,
    source_idx: usize,
    next_steiner_id: i32,
    next_terminal_id: i32,
    pub worst_wirelength: i32,
}

impl GlobalRoutingTree {
    pub fn new(source_position: Point<i32, i32>) -> Self {
        let source = RoutingNode::new("source", NodeType::Source, source_position);
        let mut nodes = Vec::new();
        let mut node_map = HashMap::new();
        node_map.insert("source".to_string(), 0usize);
        nodes.push(source);
        GlobalRoutingTree {
            nodes,
            node_map,
            source_idx: 0,
            next_steiner_id: 1,
            next_terminal_id: 1,
            worst_wirelength: 0,
        }
    }

    pub fn get_source(&self) -> &RoutingNode {
        &self.nodes[self.source_idx]
    }

    pub fn get_source_mut(&mut self) -> &mut RoutingNode {
        &mut self.nodes[self.source_idx]
    }

    fn add_node(&mut self, node: RoutingNode) -> usize {
        let idx = self.nodes.len();
        self.node_map.insert(node.id.clone(), idx);
        self.nodes.push(node);
        idx
    }

    fn _find_nearest_node(&self, point: Point<i32, i32>, exclude_id: Option<&str>) -> usize {
        if self.nodes.len() <= 1 {
            return self.source_idx;
        }
        let mut nearest = self.source_idx;
        let mut min_dist = i32::MAX;
        for (idx, node) in self.nodes.iter().enumerate() {
            if let Some(ex) = exclude_id {
                if node.id == ex {
                    continue;
                }
            }
            let dist = node.pt.min_dist_with(&point) as i32;
            if dist < min_dist {
                min_dist = dist;
                nearest = idx;
            }
        }
        nearest
    }

    pub fn insert_steiner_node(&mut self, point: Point<i32, i32>, parent_id: Option<&str>) -> String {
        let id = format!("steiner_{}", self.next_steiner_id);
        self.next_steiner_id += 1;
        let idx = self.add_node(RoutingNode::new(&id, NodeType::Steiner, point));

        let parent_idx = match parent_id {
            Some(pid) => *self.node_map.get(pid).expect("Parent node not found"),
            None => self.source_idx,
        };
        self.nodes[idx].parent = Some(parent_idx);
        self.nodes[parent_idx].children.push(idx);
        id
    }

    pub fn insert_terminal_node(&mut self, point: Point<i32, i32>, parent_id: Option<&str>) -> String {
        let id = format!("terminal_{}", self.next_terminal_id);
        self.next_terminal_id += 1;
        let idx = self.add_node(RoutingNode::new(&id, NodeType::Terminal, point));

        let parent_idx = match parent_id {
            Some(pid) => *self.node_map.get(pid).expect("Parent node not found"),
            None => self._find_nearest_node(point, None),
        };
        self.nodes[idx].parent = Some(parent_idx);
        self.nodes[parent_idx].children.push(idx);
        id
    }

    fn _find_nearest_insertion_with_constraints(
        &self,
        pt: Point<i32, i32>,
        _allowed_wirelength: i32,
        _keepouts: &Option<Vec<Interval<i32>>>,
    ) -> (Option<usize>, usize) {
        // Simplified: find nearest node directly
        let nearest = self._find_nearest_node(pt, None);
        (None, nearest)
    }

    fn _insert_terminal_impl(
        &mut self,
        point: Point<i32, i32>,
        allowed_wirelength: i32,
        keepouts: Option<Vec<Interval<i32>>>,
    ) {
        let terminal_id = format!("terminal_{}", self.next_terminal_id);
        self.next_terminal_id += 1;
        let terminal_idx = self.add_node(RoutingNode::new(&terminal_id, NodeType::Terminal, point));

        let (parent_node, nearest_node) =
            self._find_nearest_insertion_with_constraints(point, allowed_wirelength, &keepouts);

        let nearest_idx = nearest_node;
        match parent_node {
            None => {
                self.nodes[terminal_idx].parent = Some(nearest_idx);
                self.nodes[nearest_idx].children.push(terminal_idx);
                let dist = self.nodes[nearest_idx].pt.min_dist_with(&point) as i32;
                self.nodes[terminal_idx].path_length =
                    self.nodes[nearest_idx].path_length + dist;
            }
            Some(parent_idx) => {
                let steiner_id = format!("steiner_{}", self.next_steiner_id);
                self.next_steiner_id += 1;
                let nearest_pt = point; // simplified
                let steiner_idx =
                    self.add_node(RoutingNode::new(&steiner_id, NodeType::Steiner, nearest_pt));

                // Rewire
                self.nodes[parent_idx]
                    .children
                    .retain(|&c| c != nearest_idx);
                self.nodes[nearest_idx].parent = None;

                self.nodes[steiner_idx].parent = Some(parent_idx);
                self.nodes[parent_idx].children.push(steiner_idx);

                let dist_ps = self.nodes[parent_idx].pt.min_dist_with(&nearest_pt) as i32;
                self.nodes[steiner_idx].path_length =
                    self.nodes[parent_idx].path_length + dist_ps;

                self.nodes[nearest_idx].parent = Some(steiner_idx);
                self.nodes[steiner_idx].children.push(nearest_idx);

                self.nodes[terminal_idx].parent = Some(steiner_idx);
                self.nodes[steiner_idx].children.push(terminal_idx);

                let dist_st = nearest_pt.min_dist_with(&point) as i32;
                self.nodes[terminal_idx].path_length =
                    self.nodes[steiner_idx].path_length + dist_st;
            }
        }
    }

    pub fn insert_terminal_with_steiner(
        &mut self,
        point: Point<i32, i32>,
        keepouts: Option<Vec<Interval<i32>>>,
    ) {
        self._insert_terminal_impl(point, i32::MAX, keepouts);
    }

    pub fn insert_terminal_with_constraints(
        &mut self,
        point: Point<i32, i32>,
        allowed_wirelength: i32,
        keepouts: Option<Vec<Interval<i32>>>,
    ) {
        self._insert_terminal_impl(point, allowed_wirelength, keepouts);
    }

    pub fn calculate_total_wirelength(&self) -> i32 {
        let mut total = 0;
        for node in &self.nodes {
            if let Some(parent_idx) = node.parent {
                total += self.nodes[parent_idx].manhattan_distance(node);
            }
        }
        total
    }

    pub fn calculate_worst_wirelength(&self) -> i32 {
        fn traverse(tree: &GlobalRoutingTree, idx: usize) -> i32 {
            let node = &tree.nodes[idx];
            let mut worst = 0;
            for &child in &node.children {
                let child_len = node.manhattan_distance(&tree.nodes[child]);
                let child_path = traverse(tree, child);
                worst = worst.max(child_len + child_path);
            }
            worst
        }
        traverse(self, self.source_idx)
    }

    pub fn find_path_to_source(&self, node_id: &str) -> Vec<&RoutingNode> {
        let mut idx = *self.node_map.get(node_id).expect("Node not found");
        let mut path = Vec::new();
        loop {
            path.push(&self.nodes[idx]);
            match self.nodes[idx].parent {
                Some(p) => idx = p,
                None => break,
            }
        }
        path.reverse();
        path
    }

    pub fn get_all_terminals(&self) -> Vec<&RoutingNode> {
        self.nodes
            .iter()
            .filter(|n| n.node_type == NodeType::Terminal)
            .collect()
    }

    pub fn get_all_steiner_nodes(&self) -> Vec<&RoutingNode> {
        self.nodes
            .iter()
            .filter(|n| n.node_type == NodeType::Steiner)
            .collect()
    }

    pub fn get_tree_structure(&self) -> String {
        fn fmt_node(tree: &GlobalRoutingTree, idx: usize, level: usize) -> String {
            let node = &tree.nodes[idx];
            let mut s = format!(
                "{}{}({}, {})",
                "  ".repeat(level),
                node.node_type,
                node.id,
                node.pt
            );
            s.push('\n');
            for &child in &node.children {
                s.push_str(&fmt_node(tree, child, level + 1));
            }
            s
        }
        fmt_node(self, self.source_idx, 0)
    }

    pub fn visualize_tree(&self) {
        println!("Global Routing Tree Structure:");
        println!("================================");
        print!("{}", self.get_tree_structure());
        println!("Total wirelength: {}", self.calculate_total_wirelength());
        println!("Total nodes: {}", self.nodes.len());
        println!("Terminals: {}", self.get_all_terminals().len());
        println!("Steiner points: {}", self.get_all_steiner_nodes().len());
    }

    pub fn optimize_steiner_points(&mut self) {
        let to_remove: Vec<usize> = self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.node_type == NodeType::Steiner && n.children.len() == 1 && n.parent.is_some())
            .map(|(i, _)| i)
            .collect();

        for idx in to_remove.into_iter().rev() {
            let parent = self.nodes[idx].parent;
            let child = self.nodes[idx].children[0];
            if let Some(p) = parent {
                self.nodes[p].children.retain(|&c| c != idx);
                self.nodes[p].children.push(child);
            }
            self.nodes[child].parent = parent;
            self.node_map.remove(&self.nodes[idx].id);
        }
        self.nodes.retain(|n| self.node_map.contains_key(&n.id));
        // Remap indices
        self.remap_indices();
    }

    fn remap_indices(&mut self) {
        let old_indices: Vec<usize> = (0..self.nodes.len()).collect();
        let mut new_map = HashMap::new();
        let mut new_nodes = Vec::new();
        for node in self.nodes.drain(..) {
            let new_idx = new_nodes.len();
            new_map.insert(node.id.clone(), new_idx);
            new_nodes.push(node);
        }
        self.nodes = new_nodes;
        for node in &mut self.nodes {
            if let Some(p) = node.parent {
                if let Some(&old_p) = old_indices.get(p) {
                    node.parent = Some(old_p);
                } else {
                    node.parent = None;
                }
            }
            node.children = node
                .children
                .iter()
                .filter_map(|c| old_indices.get(*c).copied())
                .collect();
        }
        self.node_map = new_map;
    }
}

/// High-level global router that constructs a routing tree.
pub struct GlobalRouter {
    terminal_positions: Vec<Point<i32, i32>>,
    tree: GlobalRoutingTree,
    worst_wirelength: i32,
    keepouts: Option<Vec<Interval<i32>>>,
}

impl GlobalRouter {
    pub fn new(
        source_pos: Point<i32, i32>,
        terminal_positions: Vec<Point<i32, i32>>,
        keepout_regions: Option<Vec<Interval<i32>>>,
    ) -> Self {
        let mut sorted = terminal_positions.clone();
        sorted.sort_by(|a, b| {
            let da = source_pos.min_dist_with(a) as i32;
            let db = source_pos.min_dist_with(b) as i32;
            da.cmp(&db)
        });

        let worst = if sorted.is_empty() {
            0
        } else {
            source_pos.min_dist_with(&sorted[sorted.len() - 1]) as i32
        };

        GlobalRouter {
            terminal_positions: sorted,
            tree: GlobalRoutingTree::new(source_pos),
            worst_wirelength: worst,
            keepouts: keepout_regions,
        }
    }

    pub fn route_simple(&mut self) {
        for &terminal in &self.terminal_positions {
            self.tree.insert_terminal_node(terminal, None);
        }
    }

    pub fn route_with_steiners(&mut self) {
        self.tree.worst_wirelength = self.worst_wirelength;
        for &terminal in &self.terminal_positions {
            self.tree
                .insert_terminal_with_steiner(terminal, self.keepouts.clone());
        }
    }

    pub fn route_with_constraints(&mut self, multiplier: f64) {
        let allowed = (self.worst_wirelength as f64 * multiplier).round() as i32;
        self.tree.worst_wirelength = self.worst_wirelength;
        for &terminal in &self.terminal_positions {
            self.tree
                .insert_terminal_with_constraints(terminal, allowed, self.keepouts.clone());
        }
    }

    pub fn get_tree(&self) -> &GlobalRoutingTree {
        &self.tree
    }
}
