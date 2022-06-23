/// A binary heap is a data structure to represent ordered data. It is commonly used to implement priority queues.
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
/// assert_eq!(priorityQueue.pop(), 1);
/// assert_eq!(priorityQueue.pop(), 3);
/// assert_eq!(priorityQueue.pop(), 7);
/// ```
#[derive(Debug)]
pub struct BinHeap<T: Ord> {
    heap: Vec<T>,
}

impl<T: Ord> BinHeap<T> {

    /// Creates a new empty heap with the given [Order].
    pub fn new() -> BinHeap<T> {
        BinHeap {
            heap: Vec::<T>::new()
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

    fn sift_up(&mut self, node: usize) {
        let parent = node / 2;
        while node > 0 && self.heap[node].cmp(&self.heap[parent]).is_le() {
            self.heap.swap(node, parent);
            self.sift_up(parent);
        }
    }
}