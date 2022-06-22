use std::cmp::Ordering;

/// A binary heap is a data structure to represent ordered data. It is commonly used to implement priority queues.
/// 
/// # Examples
/// ```
/// use binheap::{BinHeap, Asc};
/// 
/// let mut priorityQueue = BinHeap::new(Asc);
/// 
/// priorityQueue.push(3);
/// priorityQueue.push(1);
/// priorityQueue.push(7);
/// 
/// while let Some(value) = priorityQueue.pop() {
///     println!("{value}");
/// }
/// ```
#[derive(Debug)]
pub struct BinHeap<T: Ord, O: Order<T>> {
    heap: Vec<T>,
    order: O,
}

/// Defines the Order in which the elements are retrieved from the Queue. There are two implementations given, [Asc] and [Desc].
pub trait Order<T: Ord> {
    fn cmp(&self, first: &T, second: &T) -> Ordering;
    fn is_le(&self, first: &T, second: &T) -> bool {
        self.cmp(first, second).is_le()
    }
}

/// Defines an ascending order of elements.
pub struct Asc;
/// Defines a descending order of elements.
pub struct Desc;

impl<T: Ord> Order<T> for Asc {
    fn cmp(&self, first: &T, second: &T) -> Ordering {
        first.cmp(second)
    }
}
impl<T: Ord> Order<T> for Desc {
    fn cmp(&self, first: &T, second: &T) -> Ordering {
        second.cmp(first)
    }
}


impl<T: Ord, O: Order<T>> BinHeap<T, O> {

    /// Creates a new empty heap with the given [Order].
    pub fn new(order: O) -> BinHeap<T, O> {
        BinHeap {
            heap: Vec::<T>::new(),
            order
        }
    }

    /// Pushes a new element of type T onto the heap.
    pub fn push(&mut self, element: T) {
        self.heap.push(element);
        self.sift_up(self.heap.len() - 1);
    }

    /// Pops the lowest element (defined by the [Order]), aka. the root of the heap. Returns [None] if the heap is empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }

        let root = self.heap.swap_remove(0);
        self.sift_down(0);
        Some(root)
    }

    /// Returns an immutable reference to the root or [None] if the heap is empty. A mutable reference can't be obtained, since mutation of an element could destroy the heap invariant. To mutate the root, pop, mutate and push it back on the heap.
    pub fn root(&self) -> Option<&T> {
        self.heap.get(0)
    }

    fn sift_down(&mut self, node: usize) {
        let left = node * 2 + 1;
        let right = node * 2 + 2;
        if node * 2 + 1 < self.heap.len() {
            let no_right = right >= self.heap.len();
            let other = if no_right || self.order.is_le(&self.heap[left], &self.heap[right]) {
                left
            } else {
                right
            };
            if self.order.is_le(&self.heap[other], &self.heap[node]) {
                self.heap.swap(other, node);
                self.sift_down(other)
            }
        }
    }

    fn sift_up(&mut self, node: usize) {
        let parent = node / 2;
        while node > 0 && self.order.is_le(&self.heap[node], &self.heap[parent]) {
            self.heap.swap(node, parent);
            self.sift_up(parent);
        }
    }
}