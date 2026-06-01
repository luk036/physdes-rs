//! Circular doubly-linked list for polygon decomposition algorithms.
//!
//! Provides `RDllist`, a circular doubly-linked list specialized for
//! `Dllink<usize>` nodes. Used internally by rectilinear polygon cut
//! and hull operations.

use crate::dllink::Dllink;

/// Iterator over the nodes in an `RDllist`.
///
/// Traverses the circular list starting from a given node and
/// stops when it returns to the starting node.
pub struct RDllIterator {
    current: *mut Dllink<usize>,
    stop: *mut Dllink<usize>,
    started: bool,
}

impl RDllIterator {
    fn new(node: *mut Dllink<usize>) -> Self {
        RDllIterator {
            current: node,
            stop: node,
            started: false,
        }
    }
}

impl Iterator for RDllIterator {
    type Item = *mut Dllink<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }
        if !self.started {
            self.started = true;
            Some(self.current)
        } else {
            unsafe {
                self.current = (*self.current).next;
            }
            if self.current == self.stop {
                None
            } else {
                Some(self.current)
            }
        }
    }
}

/// A circular doubly-linked list of `Dllink<usize>` nodes.
///
/// Nodes are stored in a contiguous `Vec` for cache efficiency.
/// The list is constructed as a circular chain where each node's
/// `next` and `prev` point to adjacent nodes, forming a ring.
///
/// This structure is used internally by `rpolygon_cut` and
/// `rpolygon_hull` algorithms which need to dynamically insert
/// and remove nodes during recursive decomposition.
pub struct RDllist {
    /// Storage for all list nodes. The vector is pre-reserved with
    /// extra capacity (3x initial size) to allow expansion during
    /// recursive cut operations.
    pub cycle: Vec<Dllink<usize>>,
}

impl RDllist {
    /// Creates a new circular doubly-linked list with `num_nodes` nodes.
    ///
    /// The nodes are labeled `0..num_nodes` and linked in a circular chain.
    /// If `reverse` is `true`, the chain is built in reverse order.
    ///
    /// The internal vector is pre-reserved with `3 * num_nodes` capacity
    /// to accommodate node insertions during recursive decomposition.
    pub fn new(num_nodes: usize, reverse: bool) -> Self {
        let mut cycle: Vec<Dllink<usize>> = Vec::with_capacity(3 * num_nodes);
        for k in 0..num_nodes {
            cycle.push(Dllink::new_linked(k));
        }

        let len = cycle.len();
        if len == 0 {
            return RDllist { cycle };
        }

        // Link nodes in a circular chain
        if !reverse {
            // Forward order: 0 -> 1 -> 2 -> ... -> N-1 -> 0
            for i in 0..len {
                let prev_i = if i == 0 { len - 1 } else { i - 1 };
                let next_i = if i == len - 1 { 0 } else { i + 1 };
                let next_ptr: *mut Dllink<usize> = &mut cycle[next_i];
                let prev_ptr: *mut Dllink<usize> = &mut cycle[prev_i];
                cycle[i].next = next_ptr;
                cycle[i].prev = prev_ptr;
            }
        } else {
            // Reverse order: 0 -> N-1 -> N-2 -> ... -> 1 -> 0
            for i in 0..len {
                let prev_i = if i == 0 { len - 1 } else { i - 1 };
                let next_i = if i == len - 1 { 0 } else { i + 1 };
                // In reverse ordering, prev and next are swapped relative to forward
                let rev_prev_ptr: *mut Dllink<usize> = &mut cycle[next_i];
                let rev_next_ptr: *mut Dllink<usize> = &mut cycle[prev_i];
                cycle[i].next = rev_next_ptr;
                cycle[i].prev = rev_prev_ptr;
            }
        }

        RDllist { cycle }
    }

    /// Returns a reference to the node at index `k`.
    #[inline]
    pub fn get(&self, k: usize) -> &Dllink<usize> {
        &self.cycle[k]
    }

    /// Returns a mutable reference to the node at index `k`.
    #[inline]
    pub fn get_mut(&mut self, k: usize) -> &mut Dllink<usize> {
        &mut self.cycle[k]
    }

    /// Creates an iterator starting from node `k`.
    #[inline]
    pub fn from_node(&self, k: usize) -> RDllIterator {
        let ptr: *mut Dllink<usize> =
            &self.cycle[k] as *const Dllink<usize> as *mut Dllink<usize>;
        RDllIterator::new(ptr)
    }

    /// Returns an iterator starting from node 0.
    #[inline]
    pub fn iter(&self) -> RDllIterator {
        self.from_node(0)
    }

    /// Adds a new linked node with the given data, returning its index.
    ///
    /// The node is in locked (self-pointing) state and must be
    /// spliced into the list by the caller.
    pub fn push_linked(&mut self, data: usize) -> usize {
        let idx = self.cycle.len();
        self.cycle.push(Dllink::new_linked(data));
        idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let list = RDllist::new(0, false);
        assert_eq!(list.cycle.len(), 0);
    }

    #[test]
    fn test_forward_list() {
        let list = RDllist::new(3, false);
        let n0 = list.get(0);
        let n1 = list.get(1);
        let n2 = list.get(2);

        // Check forward links: 0 -> 1 -> 2 -> 0
        assert_eq!(unsafe { (*n0.next).data }, 1);
        assert_eq!(unsafe { (*n1.next).data }, 2);
        assert_eq!(unsafe { (*n2.next).data }, 0);

        // Check backward links: 0 -> 2 -> 1 -> 0
        assert_eq!(unsafe { (*n0.prev).data }, 2);
        assert_eq!(unsafe { (*n1.prev).data }, 0);
        assert_eq!(unsafe { (*n2.prev).data }, 1);
    }

    #[test]
    fn test_reverse_list() {
        let list = RDllist::new(3, true);
        let n0 = list.get(0);
        let n1 = list.get(1);
        let n2 = list.get(2);

        // Reverse order: 0 -> 2 -> 1 -> 0
        assert_eq!(unsafe { (*n0.next).data }, 2);
        assert_eq!(unsafe { (*n1.next).data }, 0);
        assert_eq!(unsafe { (*n2.next).data }, 1);
    }

    #[test]
    fn test_push_linked() {
        let mut list = RDllist::new(2, false);
        let idx = list.push_linked(99);
        assert_eq!(idx, 2);
        assert!(list.get(2).is_locked());
        assert_eq!(list.get(2).data, 99);
    }

    #[test]
    fn test_capacity_reserved() {
        let list = RDllist::new(5, false);
        assert!(list.cycle.capacity() >= 15); // 3 * 5
    }

    #[test]
    fn test_iterator_yields_all_nodes() {
        let list = RDllist::new(5, false);
        let count: usize = list.from_node(0).count();
        assert_eq!(count, 5, "Iterator should yield 5 nodes for a 5-element list");
    }

    #[test]
    fn test_iterator_yields_all_nodes_from_middle() {
        let list = RDllist::new(5, false);
        let count: usize = list.from_node(2).count();
        assert_eq!(count, 5, "Iterator from middle should still yield all 5 nodes");
    }

    #[test]
    fn test_detach_and_iterate() {
        let mut list = RDllist::new(5, false);

        // Detach node at index 2
        list.cycle[2].detach();

        // Iterate should now yield 4 nodes (index 2 is detached)
        let count: usize = list.from_node(0).count();
        assert_eq!(count, 4, "After detaching one node, should yield 4");
    }

    #[test]
    fn test_detach_middle_and_check_links() {
        let mut list = RDllist::new(5, false);

        // Verify links before detach
        assert_eq!(unsafe { (*list.cycle[2].next).data }, 3);
        assert_eq!(unsafe { (*list.cycle[2].prev).data }, 1);

        // Detach node at index 2
        list.cycle[2].detach();
        assert!(list.cycle[2].is_locked());

        // Verify node 1 now points to node 3
        assert_eq!(unsafe { (*list.cycle[1].next).data }, 3);
        // Verify node 3 now points to node 1
        assert_eq!(unsafe { (*list.cycle[3].prev).data }, 1);
    }
}
