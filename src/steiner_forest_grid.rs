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
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
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

/// A pair of grid coordinates `((sx, sy), (tx, ty))`.
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
pub fn steiner_forest_grid(
    height: usize,
    width: usize,
    pairs: &[Pair],
) -> SteinerForestResult {
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

    let mut paid: HashMap<(usize, usize), f64> = HashMap::new();
    let mut f: Vec<(usize, usize, f64)> = Vec::new();

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
        let mut candidate_edges: Vec<(usize, usize, f64, (usize, usize))> = Vec::new();

        for &(u, v, cost) in &edges {
            if uf.find(u) == uf.find(v) {
                continue;
            }
            let root_u = uf.find(u);
            let root_v = uf.find(v);
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
            let key = (u.min(v), u.max(v));
            let paid_val = *paid.get(&key).unwrap_or(&0.0);
            if paid_val > cost {
                continue;
            }
            let delta_e = (cost - paid_val) / num as f64;
            if delta_e < min_delta {
                min_delta = delta_e;
                candidate_edges.clear();
                candidate_edges.push((u, v, cost, key));
            } else if (delta_e - min_delta).abs() < 1e-12 {
                candidate_edges.push((u, v, cost, key));
            }
        }

        if min_delta == f64::INFINITY {
            panic!("Graph is not connected or cannot connect pairs");
        }

        let (chosen_u, chosen_v, chosen_c, chosen_key) = candidate_edges[0];

        for &(u2, v2, c2) in &edges {
            if uf.find(u2) == uf.find(v2) {
                continue;
            }
            let ru2 = uf.find(u2);
            let rv2 = uf.find(v2);
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
            let key2 = (u2.min(v2), u2.max(v2));
            let entry = paid.entry(key2).or_insert(0.0);
            *entry += min_delta * num2 as f64;
            if *entry > c2 + 1e-6 {
                *entry = c2;
            }
        }

        if *paid.get(&chosen_key).unwrap_or(&0.0) >= chosen_c - 1e-6 {
            f.push((chosen_u, chosen_v, chosen_c));
            uf.union(chosen_u, chosen_v);
        }
    }

    let mut f_pruned = f.clone();
    let mut idx = f.len();
    while idx > 0 {
        idx -= 1;
        let mut temp_uf = UnionFind::new(n);
        for (j, &(ej_u, ej_v, _)) in f.iter().enumerate() {
            if j != idx {
                temp_uf.union(ej_u, ej_v);
            }
        }
        let mut connected = true;
        for &src in &sources {
            if let Some(partners) = pair_dict.get(&src) {
                for &tgt in partners {
                    if temp_uf.find(src) != temp_uf.find(tgt) {
                        connected = false;
                        break;
                    }
                }
            }
            if !connected {
                break;
            }
        }
        if connected {
            f_pruned.remove(idx);
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
