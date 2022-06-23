/// BinHeap is an implementation of a binary heap.
/// 
/// # Examples
/// ```
/// use binheap::BinHeap;
/// 
/// let mut priorityQueue = BinHeap::new();
/// 
/// priorityQueue.push(3);
/// priorityQueue.push(1);
/// priorityQueue.push(7);
/// 
/// assert_eq!(priorityQueue.pop(), Some(1));
/// assert_eq!(priorityQueue.pop(), Some(3));
/// assert_eq!(priorityQueue.pop(), Some(7));
/// assert_eq!(priorityQueue.pop(), None);
/// ```
#[derive(Debug)]
pub struct BinHeap<T: Ord> {
    heap: Vec<T>,
}

impl<T: Ord> BinHeap<T> {

    /// Creates a new empty heap.
    pub fn new() -> BinHeap<T> {
        BinHeap {
            heap: Vec::<T>::new()
        }
    }

    /// Pushes a new element of type [T][Ord] onto the heap.
    /// Max runtime of O(log n).
    /// 
    /// # Panic
    /// [push()](BinHeap::push) doesn't panic by itself, but the via [T][Ord] provided [cmp()](Ord::cmp) can panic. If it does, no memory is leaked and nothing unsafe happens, but the heap state may be disrupted, so it can't be used reliably afterwards.
    pub fn push(&mut self, element: T) {
        self.heap.push(element);
        self.sift_up(self.heap.len() - 1);
    }

    /// Pops the lowest element, aka. the root of the heap. Returns [None] if the heap is empty.
    /// Max runtime of O(log n).
    /// 
    /// # Panic
    /// [pop()](BinHeap::pop) doesn't panic by itself, but the via [T][Ord] provided [cmp()](Ord::cmp) can panic. If it does, no memory is leaked and nothing unsafe happens, but the heap state may be disrupted, so it can't be used reliably afterwards.
    pub fn pop(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }

        let root = self.heap.swap_remove(0);
        self.sift_down(0);
        Some(root)
    }

    /// Returns an immutable reference to the root or [None] if the heap is empty. A mutable reference can't be obtained, since mutation of an element could destroy the heap invariant. To mutate the root, pop, mutate and push it back on the heap.
    /// Max runtime of O(1).
    pub fn min(&self) -> Option<&T> {
        self.heap.get(0)
    }

    /// Takes the node at the index 'node' and "sift's it down", which means, it takes the node and swappes it with the smallest of its childred, until the heap invariant is restored.
    /// Max runtime of O(log n)
    fn sift_down(&mut self, node: usize) {
        let left = node * 2 + 1;
        let right = node * 2 + 2;
        if node * 2 + 1 < self.heap.len() {
            let no_right = right >= self.heap.len();
            let other = if no_right || self.heap[left].cmp(&self.heap[right]).is_le() {
                left
            } else {
                right
            };
            if self.heap[other].cmp(&self.heap[node]).is_le() {
                self.heap.swap(other, node);
                self.sift_down(other)
            }
        }
    }

    /// Takes the node at the index 'node' and "sift's it up", which means, it takes the node and swappes it with its parent, until the heap invariant is restored.
    /// Max runtime of O(log n)
    fn sift_up(&mut self, node: usize) {
        let parent = node / 2;
        while node > 0 && self.heap[node].cmp(&self.heap[parent]).is_le() {
            self.heap.swap(node, parent);
            self.sift_up(parent);
        }
    }
}