//! Global router for Steiner tree-based routing with keepout avoidance.
//!
//! Provides data structures and algorithms for constructing rectilinear
//! Steiner routing trees with support for simple routing, Steiner point
//! insertion, wirelength-constrained routing, and rectangular keepout
//! avoidance. Includes an SVG visualizer for result inspection.

use std::collections::HashMap;
use std::fmt;

use crate::generic::{Contain, MinDist};
use crate::interval::{Hull, Interval};
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
    /// Unique identifier for this node
    pub id: String,
    /// Type of this node (Source, Steiner, or Terminal)
    pub node_type: NodeType,
    /// Position of this node in the layout
    pub pt: Point<i32, i32>,
    /// Indices of child nodes in the tree
    pub children: Vec<usize>,
    /// Index of the parent node, if any
    pub parent: Option<usize>,
    /// Load capacitance at this node
    pub capacitance: f64,
    /// Signal delay at this node
    pub delay: f64,
    /// Path length from source to this node
    pub path_length: i32,
}

impl RoutingNode {
    /// Creates a new routing node with the given id, type, and position.
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

    /// Computes the Manhattan distance to another routing node.
    pub fn manhattan_distance(&self, other: &RoutingNode) -> i32 {
        self.pt.min_dist_with(&other.pt) as i32
    }
}

/// A rectilinear Steiner routing tree with support for keepout avoidance.
pub struct GlobalRoutingTree {
    nodes: Vec<RoutingNode>,
    node_map: HashMap<String, usize>,
    source_idx: usize,
    next_steiner_id: i32,
    next_terminal_id: i32,
    /// The worst-case (longest) wirelength among all source-to-terminal paths
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

    /// Returns a shared reference to the source node.
    pub fn get_source(&self) -> &RoutingNode {
        &self.nodes[self.source_idx]
    }

    /// Returns a mutable reference to the source node.
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

    pub fn insert_steiner_node(
        &mut self,
        point: Point<i32, i32>,
        parent_id: Option<&str>,
    ) -> String {
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

    pub fn insert_terminal_node(
        &mut self,
        point: Point<i32, i32>,
        parent_id: Option<&str>,
    ) -> String {
        let id = format!("terminal_{}", self.next_terminal_id);
        self.next_terminal_id += 1;

        let parent_idx = match parent_id {
            Some(pid) => *self.node_map.get(pid).expect("Parent node not found"),
            None => self._find_nearest_node(point, None),
        };

        let idx = self.add_node(RoutingNode::new(&id, NodeType::Terminal, point));
        self.nodes[idx].parent = Some(parent_idx);
        self.nodes[parent_idx].children.push(idx);
        id
    }

    /// Insert a new node on an existing branch between two nodes (Python `insert_node_on_branch`).
    #[allow(clippy::manual_contains)]
    pub fn insert_node_on_branch(
        &mut self,
        node_type: NodeType,
        point: Point<i32, i32>,
        branch_start_id: &str,
        branch_end_id: &str,
    ) -> String {
        let start_idx = *self
            .node_map
            .get(branch_start_id)
            .expect("Branch start node not found");
        let end_idx = *self
            .node_map
            .get(branch_end_id)
            .expect("Branch end node not found");

        let is_child = self.nodes[start_idx].children.iter().any(|&c| c == end_idx);
        assert!(
            self.nodes[end_idx].parent == Some(start_idx) || is_child,
            "branch_end is not a direct child of branch_start"
        );

        let id = match node_type {
            NodeType::Steiner => {
                let s = format!("steiner_{}", self.next_steiner_id);
                self.next_steiner_id += 1;
                s
            }
            NodeType::Terminal => {
                let s = format!("terminal_{}", self.next_terminal_id);
                self.next_terminal_id += 1;
                s
            }
            _ => panic!("Node type must be Steiner or Terminal"),
        };
        let new_idx = self.add_node(RoutingNode::new(&id, node_type, point));

        // Rewire: start -> new -> end
        self.nodes[start_idx].children.retain(|c| *c != end_idx);
        self.nodes[end_idx].parent = None;

        self.nodes[new_idx].parent = Some(start_idx);
        self.nodes[start_idx].children.push(new_idx);

        self.nodes[end_idx].parent = Some(new_idx);
        self.nodes[new_idx].children.push(end_idx);

        id
    }

    /// Find the nearest insertion point for a terminal, avoiding keepouts.
    /// Returns `(parent_node_idx, nearest_node_idx)` where parent_node is `Some` when a Steiner
    /// point needs to be inserted on the branch between parent and nearest.
    fn _find_insertion_point(
        &self,
        point: Point<i32, i32>,
        allowed_wirelength: i32,
        keepouts: &Option<Vec<Point<Interval<i32>, Interval<i32>>>>,
    ) -> (Option<usize>, usize) {
        let mut nearest_node = self.source_idx;
        let mut parent_node: Option<usize> = None;
        let mut min_distance = self.worst_wirelength.max(1);
        let mut valid_found = false;

        // Stack-based DFS
        let mut stack = vec![self.source_idx];

        while let Some(node_idx) = stack.pop() {
            let child_count = self.nodes[node_idx].children.len();
            // Push children in order (reverse for DFS order)
            for ci in (0..child_count).rev() {
                let child_idx = self.nodes[node_idx].children[ci];
                let possible_path = self.nodes[node_idx].pt.hull_with(&self.nodes[child_idx].pt);
                let distance = possible_path.min_dist_with(&point) as i32;
                let nearest_pt = possible_path.nearest_to(&point);

                // Check keepouts
                if let Some(ref kos) = *keepouts {
                    let mut blocked = false;
                    let path1 = nearest_pt.hull_with(&point);
                    let path2 = nearest_pt.hull_with(&self.nodes[node_idx].pt);
                    let path3 = nearest_pt.hull_with(&self.nodes[child_idx].pt);
                    for ko in kos {
                        if ko.contains(&nearest_pt)
                            || ko.blocks(&path1)
                            || ko.blocks(&path2)
                            || ko.blocks(&path3)
                        {
                            blocked = true;
                            break;
                        }
                    }
                    if blocked {
                        continue;
                    }
                }

                let path_length = self.nodes[node_idx].path_length
                    + self.nodes[node_idx].pt.min_dist_with(&nearest_pt) as i32
                    + distance;

                let mut update = false;
                if path_length <= allowed_wirelength {
                    if valid_found {
                        if distance < min_distance {
                            update = true;
                        }
                    } else {
                        valid_found = true;
                        update = true;
                    }
                } else if !valid_found
                    && path_length <= self.worst_wirelength
                    && distance < min_distance
                {
                    update = true;
                }

                if update {
                    min_distance = distance;
                    if nearest_pt == self.nodes[node_idx].pt {
                        nearest_node = node_idx;
                        parent_node = None;
                    } else if nearest_pt == self.nodes[child_idx].pt {
                        nearest_node = child_idx;
                        parent_node = None;
                    } else {
                        nearest_node = child_idx;
                        parent_node = Some(node_idx);
                    }
                }

                stack.push(child_idx);
            }
        }

        (parent_node, nearest_node)
    }

    fn _insert_terminal_impl(
        &mut self,
        point: Point<i32, i32>,
        allowed_wirelength: i32,
        keepouts: Option<Vec<Point<Interval<i32>, Interval<i32>>>>,
    ) {
        let terminal_id = format!("terminal_{}", self.next_terminal_id);
        self.next_terminal_id += 1;
        let terminal_idx = self.add_node(RoutingNode::new(&terminal_id, NodeType::Terminal, point));

        let (parent_node, nearest_node) =
            self._find_insertion_point(point, allowed_wirelength, &keepouts);

        let nearest_idx = nearest_node;
        match parent_node {
            None => {
                self.nodes[terminal_idx].parent = Some(nearest_idx);
                self.nodes[nearest_idx].children.push(terminal_idx);
                let dist = self.nodes[nearest_idx].pt.min_dist_with(&point) as i32;
                self.nodes[terminal_idx].path_length = self.nodes[nearest_idx].path_length + dist;
            }
            Some(parent_idx) => {
                let steiner_id = format!("steiner_{}", self.next_steiner_id);
                self.next_steiner_id += 1;

                let possible_path = self.nodes[parent_idx]
                    .pt
                    .hull_with(&self.nodes[nearest_idx].pt);
                let nearest_pt = possible_path.nearest_to(&point);
                let steiner_idx =
                    self.add_node(RoutingNode::new(&steiner_id, NodeType::Steiner, nearest_pt));

                // Rewire: parent -> nearest  becomes  parent -> steiner -> nearest
                self.nodes[parent_idx]
                    .children
                    .retain(|c| *c != nearest_idx);
                self.nodes[nearest_idx].parent = None;

                self.nodes[steiner_idx].parent = Some(parent_idx);
                self.nodes[parent_idx].children.push(steiner_idx);

                let dist_ps = self.nodes[parent_idx].pt.min_dist_with(&nearest_pt) as i32;
                self.nodes[steiner_idx].path_length = self.nodes[parent_idx].path_length + dist_ps;

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
        keepouts: Option<Vec<Point<Interval<i32>, Interval<i32>>>>,
    ) {
        self._insert_terminal_impl(point, i32::MAX, keepouts);
    }

    pub fn insert_terminal_with_constraints(
        &mut self,
        point: Point<i32, i32>,
        allowed_wirelength: i32,
        keepouts: Option<Vec<Point<Interval<i32>, Interval<i32>>>>,
    ) {
        self._insert_terminal_impl(point, allowed_wirelength, keepouts);
    }

    /// Calculates the total wirelength of the entire routing tree.
    pub fn calculate_total_wirelength(&self) -> i32 {
        let mut total = 0;
        for node in &self.nodes {
            if let Some(parent_idx) = node.parent {
                total += self.nodes[parent_idx].manhattan_distance(node);
            }
        }
        total
    }

    /// Calculates the worst-case (maximum) source-to-terminal wirelength.
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

    /// Finds the path from a node back to the source.
    ///
    /// Returns the nodes along the path in order from source to the target node.
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

    /// Returns all terminal nodes in the routing tree.
    pub fn get_all_terminals(&self) -> Vec<&RoutingNode> {
        self.nodes
            .iter()
            .filter(|n| n.node_type == NodeType::Terminal)
            .collect()
    }

    /// Returns all Steiner nodes in the routing tree.
    pub fn get_all_steiner_nodes(&self) -> Vec<&RoutingNode> {
        self.nodes
            .iter()
            .filter(|n| n.node_type == NodeType::Steiner)
            .collect()
    }

    /// Returns a formatted string representation of the tree structure.
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

    /// Removes redundant Steiner points that have only one child.
    ///
    /// After optimization, the remaining Steiner points have at least two
    /// children and are topologically significant.
    pub fn optimize_steiner_points(&mut self) {
        let to_remove: Vec<usize> = self
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| {
                n.node_type == NodeType::Steiner && n.children.len() == 1 && n.parent.is_some()
            })
            .map(|(i, _)| i)
            .collect();

        for &idx in to_remove.iter().rev() {
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

    /// Generate an SVG visualization of the routing tree.
    pub fn to_svg(
        &self,
        keepouts: Option<&Vec<Point<Interval<i32>, Interval<i32>>>>,
        width: u32,
        height: u32,
        margin: u32,
    ) -> String {
        if self.nodes.is_empty() {
            return "<svg></svg>".to_string();
        }

        let min_x = self.nodes.iter().map(|n| n.pt.xcoord).min().unwrap();
        let max_x = self.nodes.iter().map(|n| n.pt.xcoord).max().unwrap();
        let min_y = self.nodes.iter().map(|n| n.pt.ycoord).min().unwrap();
        let max_y = self.nodes.iter().map(|n| n.pt.ycoord).max().unwrap();

        let range_x = (max_x - min_x).max(1) as f64;
        let range_y = (max_y - min_y).max(1) as f64;

        let w = (width as f64) - 2.0 * (margin as f64);
        let h = (height as f64) - 2.0 * (margin as f64);
        let scale = (w / range_x).min(h / range_y);

        let sx = |x: i32| margin as f64 + (x - min_x) as f64 * scale;
        let sy = |y: i32| margin as f64 + (y - min_y) as f64 * scale;

        let mut svg = String::new();
        svg.push_str(&format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
            width, height
        ));
        svg.push_str(r#"<rect width="100%" height="100%" fill="white"/>"#);

        // Arrowhead marker
        svg.push_str(
            r#"<defs><marker id="ah" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">"#,
        );
        svg.push_str(r#"<polygon points="0 0, 10 3.5, 0 7" fill="black"/>"#);
        svg.push_str("</marker></defs>");

        // Draw connections
        fn draw_conn(
            svg: &mut String,
            tree: &GlobalRoutingTree,
            idx: usize,
            sx: &dyn Fn(i32) -> f64,
            sy: &dyn Fn(i32) -> f64,
        ) {
            let node = &tree.nodes[idx];
            for &child in &node.children {
                let cnode = &tree.nodes[child];
                let (x1, y1) = (sx(node.pt.xcoord), sy(node.pt.ycoord));
                let (x2, y2) = (sx(cnode.pt.xcoord), sy(cnode.pt.ycoord));
                svg.push_str(&format!(
                    r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="2" marker-end="url(#ah)"/>"#,
                    x1, y1, x2, y2
                ));
            }
            for &child in &node.children {
                draw_conn(svg, tree, child, sx, sy);
            }
        }
        draw_conn(&mut svg, self, self.source_idx, &sx, &sy);

        // Draw keepouts
        if let Some(kos) = keepouts {
            for ko in kos {
                let x1 = sx(ko.xcoord.lb);
                let y1 = sy(ko.ycoord.lb);
                let x2 = sx(ko.xcoord.ub);
                let y2 = sy(ko.ycoord.ub);
                let rw = (x2 - x1).abs();
                let rh = (y2 - y1).abs();
                svg.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="orange" stroke="black" stroke-width="1"/>"#,
                    x1.min(x2), y1.min(y2), rw, rh
                ));
            }
        }

        // Draw nodes
        for node in &self.nodes {
            let (x_pos, y_pos) = (sx(node.pt.xcoord), sy(node.pt.ycoord));
            let label = match node.node_type {
                NodeType::Source => "S".to_string(),
                NodeType::Steiner => {
                    format!("S{}", node.id.strip_prefix("steiner_").unwrap_or("t"))
                }
                NodeType::Terminal => {
                    format!("T{}", node.id.strip_prefix("terminal_").unwrap_or(""))
                }
            };
            let (color, radius) = match node.node_type {
                NodeType::Source => ("red", 8u32),
                NodeType::Steiner => ("blue", 6u32),
                NodeType::Terminal => ("green", 6u32),
            };
            svg.push_str(&format!(
                r#"<circle cx="{}" cy="{}" r="{}" fill="{}" stroke="black" stroke-width="1"/>"#,
                x_pos, y_pos, radius, color
            ));
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" font-family="Arial" font-size="10" fill="black">{}</text>"#,
                x_pos + radius as f64 + 2.0,
                y_pos + 4.0,
                label
            ));
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" font-family="Arial" font-size="8" fill="gray" text-anchor="middle">({},{})</text>"#,
                x_pos,
                y_pos - radius as f64 - 5.0,
                node.pt.xcoord,
                node.pt.ycoord
            ));
        }

        // Legend
        let ly = 20u32;
        svg.push_str(&format!(
            r#"<text x="20" y="{}" font-family="Arial" font-size="12" font-weight="bold">Legend:</text>"#,
            ly
        ));
        let items = [
            ("Source", "red", 20, ly + 20),
            ("Steiner", "blue", 20, ly + 40),
            ("Terminal", "green", 20, ly + 60),
        ];
        for (text, color, lx, ly) in &items {
            svg.push_str(&format!(
                r#"<circle cx="{}" cy="{}" r="4" fill="{}" stroke="black"/>"#,
                lx,
                ly - 4,
                color
            ));
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" font-family="Arial" font-size="10">{}</text>"#,
                lx + 10,
                ly,
                text
            ));
        }

        // Statistics
        let sy2 = ly + 90;
        svg.push_str(&format!(
            r#"<text x="20" y="{}" font-family="Arial" font-size="10" font-weight="bold">Statistics:</text>"#,
            sy2
        ));
        svg.push_str(&format!(
            r#"<text x="20" y="{}" font-family="Arial" font-size="9">Total Nodes: {}</text>"#,
            sy2 + 15,
            self.nodes.len()
        ));
        svg.push_str(&format!(
            r#"<text x="20" y="{}" font-family="Arial" font-size="9">Terminals: {}</text>"#,
            sy2 + 30,
            self.get_all_terminals().len()
        ));
        svg.push_str(&format!(
            r#"<text x="20" y="{}" font-family="Arial" font-size="9">Steiner: {}</text>"#,
            sy2 + 45,
            self.get_all_steiner_nodes().len()
        ));
        svg.push_str(&format!(
            r#"<text x="20" y="{}" font-family="Arial" font-size="9">Wirelength: {}</text>"#,
            sy2 + 60,
            self.calculate_total_wirelength()
        ));

        svg.push_str("</svg>");
        svg
    }

    /// Save SVG to a file.
    pub fn save_svg(
        &self,
        keepouts: Option<&Vec<Point<Interval<i32>, Interval<i32>>>>,
        filename: &str,
        width: u32,
        height: u32,
    ) {
        let svg = self.to_svg(keepouts, width, height, 50);
        std::fs::write(filename, svg).expect("Failed to write SVG file");
        println!("Saved SVG to {}", filename);
    }
}

/// High-level global router that constructs a routing tree from a source
/// and a set of terminal points, with optional keepout avoidance.
pub struct GlobalRouter {
    terminal_positions: Vec<Point<i32, i32>>,
    tree: GlobalRoutingTree,
    worst_wirelength: i32,
    keepouts: Option<Vec<Point<Interval<i32>, Interval<i32>>>>,
}

impl GlobalRouter {
    pub fn new(
        source_pos: Point<i32, i32>,
        terminal_positions: Vec<Point<i32, i32>>,
        keepout_regions: Option<Vec<Point<Interval<i32>, Interval<i32>>>>,
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

    /// Routes terminals by directly connecting each terminal to the nearest
    /// node in the existing tree (simple nearest-neighbor heuristic).
    pub fn route_simple(&mut self) {
        for &terminal in &self.terminal_positions {
            self.tree.insert_terminal_node(terminal, None);
        }
    }

    /// Routes terminals with Steiner point insertion to reduce total
    /// wirelength while avoiding keepout regions.
    pub fn route_with_steiners(&mut self) {
        self.tree.worst_wirelength = self.worst_wirelength;
        for &terminal in &self.terminal_positions {
            self.tree
                .insert_terminal_with_steiner(terminal, self.keepouts.clone());
        }
    }

    /// Routes terminals with Steiner points and wirelength constraints.
    /// The `multiplier` scales the worst-case wirelength to determine the
    /// allowed path length for each terminal.
    pub fn route_with_constraints(&mut self, multiplier: f64) {
        let allowed = (self.worst_wirelength as f64 * multiplier).round() as i32;
        self.tree.worst_wirelength = self.worst_wirelength;
        for &terminal in &self.terminal_positions {
            self.tree
                .insert_terminal_with_constraints(terminal, allowed, self.keepouts.clone());
        }
    }

    /// Returns a reference to the constructed routing tree.
    pub fn get_tree(&self) -> &GlobalRoutingTree {
        &self.tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_keepout(x1: i32, x2: i32, y1: i32, y2: i32) -> Point<Interval<i32>, Interval<i32>> {
        let lo_x = x1.min(x2);
        let hi_x = x1.max(x2);
        let lo_y = y1.min(y2);
        let hi_y = y1.max(y2);
        Point::new(Interval::new(lo_x, hi_x), Interval::new(lo_y, hi_y))
    }

    #[test]
    fn test_route_simple() {
        let src = Point::new(0, 0);
        let terminals = vec![Point::new(1, 1), Point::new(2, 2)];
        let mut router = GlobalRouter::new(src, terminals, None);
        router.route_simple();
        assert_eq!(router.get_tree().calculate_total_wirelength(), 4);
    }

    #[test]
    fn test_route_with_steiners() {
        let src = Point::new(0, 0);
        let terminals = vec![Point::new(1, 1), Point::new(2, 2)];
        let mut router = GlobalRouter::new(src, terminals, None);
        router.route_with_steiners();
        assert_eq!(router.get_tree().calculate_total_wirelength(), 4);
    }

    #[test]
    fn test_route_with_constraints() {
        let src = Point::new(0, 0);
        let terminals = vec![Point::new(1, 1), Point::new(2, 2)];
        let mut router = GlobalRouter::new(src, terminals, None);
        router.route_with_constraints(2.0);
        assert_eq!(router.get_tree().calculate_total_wirelength(), 4);
    }

    #[test]
    fn test_route_three_sinks_simple() {
        let src = Point::new(0, 0);
        let terminals = vec![Point::new(10, 0), Point::new(5, 10)];
        let mut router = GlobalRouter::new(src, terminals, None);
        router.route_simple();
        let wl = router.get_tree().calculate_total_wirelength();
        assert_eq!(wl, 25);
    }

    #[test]
    fn test_route_with_keepout() {
        let src = Point::new(0, 0);
        let terminals = vec![Point::new(10, 0)];
        let keepout = make_keepout(4, 6, -1, 1);
        let mut router = GlobalRouter::new(src, terminals, Some(vec![keepout]));
        router.route_with_steiners();
        let wl = router.get_tree().calculate_total_wirelength();
        // With keepout, the route should still complete
        assert!(wl > 0);
    }

    #[test]
    fn test_insert_steiner_and_terminal() {
        let mut tree = GlobalRoutingTree::new(Point::new(0, 0));
        let s1 = tree.insert_steiner_node(Point::new(1, 1), None);
        let t1 = tree.insert_terminal_node(Point::new(2, 2), Some(&s1));
        assert_eq!(tree.calculate_total_wirelength(), 4);
        assert_eq!(tree.get_all_terminals().len(), 1);
        assert_eq!(tree.get_all_steiner_nodes().len(), 1);
        let path = tree.find_path_to_source(&t1);
        assert_eq!(path.len(), 3);
        assert_eq!(path[0].id, "source");
        assert_eq!(path[1].id, s1);
        assert_eq!(path[2].id, t1);
    }

    #[test]
    fn test_insert_node_on_branch() {
        let mut tree = GlobalRoutingTree::new(Point::new(0, 0));
        let s1 = tree.insert_steiner_node(Point::new(1, 1), None);
        let t1 = tree.insert_terminal_node(Point::new(2, 2), Some(&s1));
        let new_id = tree.insert_node_on_branch(NodeType::Steiner, Point::new(1, 2), &s1, &t1);
        // new_id should be steiner_2
        assert_eq!(new_id, "steiner_2");
        // Path length should still work
        let path = tree.find_path_to_source(&t1);
        assert_eq!(path.len(), 4);
    }

    #[test]
    fn test_optimize_steiner_points() {
        let mut tree = GlobalRoutingTree::new(Point::new(0, 0));
        let s1 = tree.insert_steiner_node(Point::new(1, 1), None);
        let _t1 = tree.insert_terminal_node(Point::new(2, 2), Some(&s1));
        tree.optimize_steiner_points();
        assert_eq!(tree.get_all_steiner_nodes().len(), 0);
    }

    #[test]
    fn test_calculate_worst_wirelength() {
        let mut tree = GlobalRoutingTree::new(Point::new(0, 0));
        let s1 = tree.insert_steiner_node(Point::new(1, 1), None);
        let _t1 = tree.insert_terminal_node(Point::new(2, 2), Some(&s1));
        assert_eq!(tree.calculate_worst_wirelength(), 4);
    }

    #[test]
    fn test_to_svg_contains_elements() {
        let mut tree = GlobalRoutingTree::new(Point::new(0, 0));
        let _s1 = tree.insert_steiner_node(Point::new(1, 1), None);
        let _t1 = tree.insert_terminal_node(Point::new(2, 2), Some("steiner_1"));
        let svg = tree.to_svg(None, 200, 200, 50);
        assert!(svg.find("<svg").is_some());
        assert!(svg.find("</svg>").is_some());
        assert!(svg.find("Source").is_some());
        assert!(svg.find("Wirelength").is_some());
    }

    #[test]
    fn test_terminal_sorting_by_distance() {
        let src = Point::new(0, 0);
        let terminals = vec![Point::new(10, 0), Point::new(1, 0), Point::new(5, 0)];
        let router = GlobalRouter::new(src, terminals, None);
        // Should be sorted by distance: (1,0), (5,0), (10,0)
        assert_eq!(router.terminal_positions[0], Point::new(1, 0));
        assert_eq!(router.terminal_positions[1], Point::new(5, 0));
        assert_eq!(router.terminal_positions[2], Point::new(10, 0));
    }
}
