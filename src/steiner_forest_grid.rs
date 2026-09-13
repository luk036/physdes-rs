use std::collections::{HashMap, HashSet};

/// Union-find (disjoint-set) with path compression and union by rank.
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let mut root = x;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        let mut node = x;
        while self.parent[node] != root {
            let next = self.parent[node];
            self.parent[node] = root;
            node = next;
        }
        root
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return false;
        }
        match self.rank[ra].cmp(&self.rank[rb]) {
            std::cmp::Ordering::Less => self.parent[ra] = rb,
            std::cmp::Ordering::Greater => self.parent[rb] = ra,
            std::cmp::Ordering::Equal => {
                self.parent[rb] = ra;
                self.rank[ra] += 1;
            }
        }
        true
    }
}

/// Result of a Steiner forest computation.
#[derive(Debug, Clone)]
pub struct SteinerForestResult {
    /// Edges in the forest as `(u, v, cost)` tuples.
    pub edges: Vec<(usize, usize, f64)>,
    /// Total cost of all edges.
    pub total_cost: f64,
    /// Source node indices.
    pub sources: HashSet<usize>,
    /// Terminal node indices.
    pub terminals: HashSet<usize>,
    /// Steiner node indices.
    pub steiner_nodes: HashSet<usize>,
}

type Pair = ((usize, usize), (usize, usize));

/// Computes an approximate Steiner forest on a grid graph using a primal-dual
/// approach with reverse-delete pruning.
///
/// Matches the Python `steiner_forest_grid` and C++ `SteinerForestGrid::compute`
/// implementations for identical results.
///
/// # Arguments
///
/// * `height` - Grid height (rows).
/// * `width` - Grid width (columns).
/// * `pairs` - Terminal pairs `((sx, sy), (tx, ty))` to connect.
///
/// # Returns
///
/// A `SteinerForestResult` with edges, cost, and node classification.
///
/// # Panics
///
/// Panics if the graph is disconnected and pairs cannot be connected.
pub fn steiner_forest_grid(height: usize, width: usize, pairs: &[Pair]) -> SteinerForestResult {
    let n = height * width;
    let mut uf = UnionFind::new(n);
    let mut sources = HashSet::new();
    let mut terminals = HashSet::new();
    let mut pair_dict: HashMap<usize, Vec<usize>> = HashMap::new();

    for &((sx, sy), (tx, ty)) in pairs {
        let source_idx = sx * width + sy;
        let target_idx = tx * width + ty;
        sources.insert(source_idx);
        terminals.insert(target_idx);
        pair_dict.entry(source_idx).or_default().push(target_idx);
        pair_dict.entry(target_idx).or_default().push(source_idx);
    }

    let all_term: HashSet<usize> = sources.union(&terminals).copied().collect();

    let mut edges: Vec<(usize, usize, f64)> = Vec::new();
    for row in 0..height {
        for col in 0..width {
            let node = row * width + col;
            if col + 1 < width {
                edges.push((node, node + 1, 1.0));
            }
            if row + 1 < height {
                edges.push((node, node + width, 1.0));
            }
        }
    }

    let mut paid = vec![0.0; edges.len()];
    let mut f = Vec::new();

    loop {
        let term_root: HashMap<usize, usize> = all_term.iter().map(|&t| (t, uf.find(t))).collect();

        let mut feasible = true;
        for (&src, partners) in &pair_dict {
            let root_src = term_root[&src];
            for &tgt in partners {
                if term_root[&tgt] != root_src {
                    feasible = false;
                    break;
                }
            }
            if !feasible {
                break;
            }
        }
        if feasible {
            break;
        }

        let mut comp_terms: HashMap<usize, HashSet<usize>> = HashMap::new();
        for &t in &all_term {
            comp_terms.entry(term_root[&t]).or_default().insert(t);
        }

        let mut active_comps = HashSet::new();
        for (&root, terms) in &comp_terms {
            let mut is_active = false;
            for &t in terms {
                if let Some(partners) = pair_dict.get(&t) {
                    for &partner in partners {
                        if term_root[&partner] != root {
                            is_active = true;
                            break;
                        }
                    }
                }
                if is_active {
                    break;
                }
            }
            if is_active {
                active_comps.insert(root);
            }
        }

        let mut min_delta = f64::INFINITY;
        let mut chosen_idx = 0usize;
        let mut chosen_u = 0usize;
        let mut chosen_v = 0usize;
        let mut chosen_c = 0.0f64;

        for (edge_idx, &(u, v, cost)) in edges.iter().enumerate() {
            let root_u = uf.find(u);
            let root_v = uf.find(v);
            if root_u == root_v {
                continue;
            }
            let mut num = 0;
            if active_comps.contains(&root_u) {
                num += 1;
            }
            if active_comps.contains(&root_v) {
                num += 1;
            }
            if num == 0 {
                continue;
            }
            let paid_val = paid[edge_idx];
            if paid_val > cost {
                continue;
            }
            let delta_e = (cost - paid_val) / num as f64;
            if delta_e < min_delta {
                min_delta = delta_e;
                chosen_idx = edge_idx;
                chosen_u = u;
                chosen_v = v;
                chosen_c = cost;
            }
        }

        if min_delta == f64::INFINITY {
            panic!("Graph is not connected or cannot connect pairs");
        }

        for (edge_idx, &(u2, v2, c2)) in edges.iter().enumerate() {
            let ru2 = uf.find(u2);
            let rv2 = uf.find(v2);
            if ru2 == rv2 {
                continue;
            }
            let mut num2 = 0;
            if active_comps.contains(&ru2) {
                num2 += 1;
            }
            if active_comps.contains(&rv2) {
                num2 += 1;
            }
            if num2 == 0 {
                continue;
            }
            let entry = &mut paid[edge_idx];
            *entry += min_delta * num2 as f64;
            if *entry > c2 + 1e-6 {
                *entry = c2;
            }
        }

        if paid[chosen_idx] >= chosen_c - 1e-6 {
            f.push((chosen_u, chosen_v, chosen_c));
            uf.union(chosen_u, chosen_v);
        }
    }

    // `f` is a forest: every added edge merges two distinct components, so the
    // minimal sub-forest preserving all required pair connections is the union
    // of the unique paths between each pair. Mark those paths directly instead
    // of rebuilding a UnionFind for every candidate edge.
    let mut adjacency: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for (edge_idx, &(u, v, _)) in f.iter().enumerate() {
        adjacency[u].push((v, edge_idx));
        adjacency[v].push((u, edge_idx));
    }

    let mut visited = vec![false; n];
    let mut parent_edge = vec![usize::MAX; n];
    let mut parent_node = vec![usize::MAX; n];
    let mut needed = vec![false; f.len()];
    let mut stack: Vec<usize> = Vec::new();
    let mut touched: Vec<usize> = Vec::new();

    for &src in &sources {
        stack.clear();
        touched.clear();
        stack.push(src);
        visited[src] = true;
        touched.push(src);
        while let Some(node) = stack.pop() {
            for &(neighbor, edge_idx) in &adjacency[node] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    parent_edge[neighbor] = edge_idx;
                    parent_node[neighbor] = node;
                    stack.push(neighbor);
                    touched.push(neighbor);
                }
            }
        }
        if let Some(partners) = pair_dict.get(&src) {
            for &tgt in partners {
                let mut current = tgt;
                while current != src {
                    let edge_idx = parent_edge[current];
                    if edge_idx == usize::MAX {
                        break;
                    }
                    needed[edge_idx] = true;
                    current = parent_node[current];
                }
            }
        }
        for &node in &touched {
            visited[node] = false;
        }
    }

    let mut f_pruned: Vec<(usize, usize, f64)> = Vec::with_capacity(f.len());
    for (edge_idx, &edge) in f.iter().enumerate() {
        if needed[edge_idx] {
            f_pruned.push(edge);
        }
    }

    let total_cost: f64 = f_pruned.iter().map(|&(_, _, c)| c).sum();

    let mut used_nodes = HashSet::new();
    for &(u, v, _) in &f_pruned {
        used_nodes.insert(u);
        used_nodes.insert(v);
    }
    let steiner_nodes: HashSet<usize> = used_nodes.difference(&all_term).copied().collect();

    SteinerForestResult {
        edges: f_pruned,
        total_cost,
        sources,
        terminals,
        steiner_nodes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steiner_forest_basic() {
        let h = 2;
        let w = 2;
        let pairs = [((0, 0), (1, 1))];
        let result = steiner_forest_grid(h, w, &pairs);
        assert!((result.total_cost - 2.0).abs() < 1e-9);
        assert!(result.sources.contains(&0));
        assert!(result.terminals.contains(&3));
        assert!(!result.steiner_nodes.is_empty());
    }

    #[test]
    fn test_steiner_forest_single_pair() {
        let h = 3;
        let w = 3;
        let pairs = [((0, 0), (2, 2))];
        let result = steiner_forest_grid(h, w, &pairs);
        assert!(result.total_cost > 0.0);
    }

    #[test]
    fn test_steiner_forest_adjacent_terminals() {
        let h = 2;
        let w = 2;
        let pairs = [((0, 0), (0, 1))];
        let result = steiner_forest_grid(h, w, &pairs);
        assert!((result.total_cost - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_steiner_forest_large_grid() {
        let h = 8;
        let w = 8;
        let pairs = [
            ((0, 0), (3, 2)),
            ((0, 0), (0, 5)),
            ((4, 4), (7, 5)),
            ((4, 4), (5, 7)),
            ((0, 1), (4, 1)),
        ];
        let result = steiner_forest_grid(h, w, &pairs);
        assert!((result.total_cost - 17.0).abs() < 1e-9);
        assert_eq!(result.edges.len(), 17);
    }

    #[test]
    fn test_steiner_forest_empty_pairs() {
        let h = 3;
        let w = 3;
        let pairs: [((usize, usize), (usize, usize)); 0] = [];
        let result = steiner_forest_grid(h, w, &pairs);
        assert!((result.total_cost - 0.0).abs() < 1e-9);
        assert!(result.edges.is_empty());
        assert!(result.steiner_nodes.is_empty());
    }

    #[test]
    fn test_steiner_forest_multi_pair() {
        let h = 4;
        let w = 4;
        let pairs = [((0, 0), (1, 1)), ((2, 2), (3, 3))];
        let result = steiner_forest_grid(h, w, &pairs);
        assert!(result.total_cost > 0.0);
        assert_eq!(result.sources.len(), 2);
        assert_eq!(result.terminals.len(), 2);
    }

    #[test]
    fn test_steiner_forest_consistency() {
        let h = 5;
        let w = 5;
        let pairs = [((0, 0), (4, 4))];
        let result1 = steiner_forest_grid(h, w, &pairs);
        let result2 = steiner_forest_grid(h, w, &pairs);
        assert!((result1.total_cost - result2.total_cost).abs() < 1e-9);
        assert_eq!(result1.edges.len(), result2.edges.len());
    }
}
