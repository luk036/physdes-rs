//! Doubly-linked link node for polygon decomposition algorithms.
//!
//! Provides a low-level intrusive doubly-linked list node (`Dllink<T>`)
//! used internally by rectilinear polygon cut and hull algorithms.

use std::marker::PhantomData;

/// A doubly-linked list node.
///
/// Each node holds a value of type `T` and pointers to the next and previous
/// nodes in the list. A freshly created node is "locked" (points to itself),
/// acting as a sentinel. Call `lock()` / `is_locked()` to manage this state.
///
/// # Safety
///
/// This type uses raw pointers internally and is neither `Send` nor `Sync`.
/// The caller must ensure that all nodes outlive any pointers referencing them.
#[repr(C)]
pub struct Dllink<T> {
    /// Pointer to the next node.
    pub next: *mut Dllink<T>,
    /// Pointer to the previous node.
    pub prev: *mut Dllink<T>,
    /// Stored data value.
    pub data: T,
    /// Marker to opt out of auto-Send/Sync.
    _marker: PhantomData<*mut T>,
}

// SAFETY: Dllink is designed for single-threaded internal use.
// We explicitly opt out of Send/Sync because of raw pointer fields.
// However, T itself should still be Send/Sync if possible.
// The auto-derived impls are correct: raw pointers make it !Send/!Sync by default.

impl<T> Dllink<T> {
    /// Creates a new locked (detached) `Dllink` with the given data.
    ///
    /// A locked node has null pointers and is not part of any list.
    #[inline]
    pub const fn new(data: T) -> Self {
        Dllink {
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
            data,
            _marker: PhantomData,
        }
    }

    /// Creates a new locked (detached) `Dllink`.
    #[inline]
    pub fn new_linked(data: T) -> Self {
        Dllink {
            next: std::ptr::null_mut(),
            prev: std::ptr::null_mut(),
            data,
            _marker: PhantomData,
        }
    }

    /// Locks the node (nulls pointers). After locking, the node is not
    /// part of any list.
    #[inline]
    pub fn lock(&mut self) {
        self.next = std::ptr::null_mut();
        self.prev = std::ptr::null_mut();
    }

    /// Returns `true` if the node is locked (null pointers, not in a list).
    #[inline]
    pub fn is_locked(&self) -> bool {
        self.next.is_null() && self.prev.is_null()
    }

    /// Detaches this node from its containing list.
    ///
    /// Panics if the node is locked (not in a list).
    #[inline]
    pub fn detach(&mut self) {
        assert!(!self.is_locked(), "cannot detach a locked node");
        let n = self.next;
        let p = self.prev;
        unsafe {
            (*p).next = n;
            (*n).prev = p;
        }
        self.lock();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_locked() {
        let node = Dllink::new_linked(42);
        assert!(node.is_locked());
        assert_eq!(node.data, 42);
    }

    #[test]
    fn test_lock_unlock() {
        let mut n1 = Dllink::new_linked(1);
        let mut n2 = Dllink::new_linked(2);

        // Link n1 -> n2 circularly
        n1.next = &mut n2;
        n2.prev = &mut n1;

        assert!(!n1.is_locked());
        assert!(!n2.is_locked());

        n1.lock();
        assert!(n1.is_locked());
    }

    #[test]
    fn test_detach() {
        let mut n1 = Dllink::new_linked(1);
        let mut n2 = Dllink::new_linked(2);
        let mut n3 = Dllink::new_linked(3);

        // Set up circular list: n1 -> n2 -> n3 -> n1
        let p1: *mut Dllink<i32> = &mut n1;
        let p2: *mut Dllink<i32> = &mut n2;
        let p3: *mut Dllink<i32> = &mut n3;

        n1.next = p2;
        n1.prev = p3;
        n2.next = p3;
        n2.prev = p1;
        n3.next = p1;
        n3.prev = p2;

        // Detach n2
        n2.detach();
        assert!(n2.is_locked());

        // Now n1.next should be n3
        assert_eq!(unsafe { (*n1.next).data }, 3);
        // And n3.prev should be n1
        assert_eq!(unsafe { (*n3.prev).data }, 1);
    }

    #[test]
    #[should_panic(expected = "cannot detach a locked node")]
    fn test_detach_locked_panics() {
        let mut node = Dllink::new_linked(99);
        node.detach();
    }
}
