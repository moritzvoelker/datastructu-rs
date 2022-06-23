#![deny(missing_docs)]

//! This crate provides a simple (and hopefully efficient) implementation of a binary heap. Heaps are often used to implement priority queues, for example for task scheduling, an example is provided below.
//! 
//! A binary heap provides several operations with good runtime behaviour.
//! - push O(log n)
//! - pop  O(log n)
//! - min  O(1)
//! 
//! # Example
//! ```
//! use binheap::BinHeap;
//! 
//! struct Task (i32, Box<dyn Fn() -> ()>);
//! impl PartialEq for Task {
//!     fn eq(&self, other: &Self) -> bool {
//!         self.0 == other.0
//!     }
//! }
//! impl Eq for Task { }
//! impl PartialOrd for Task {
//!     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//!         self.0.partial_cmp(&other.0)
//!     }
//! }
//! impl Ord for Task {
//!     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//!         self.partial_cmp(other).unwrap()
//!     }
//! }
//! 
//! let mut heap = BinHeap::new();
//! 
//! heap.push(Task (3, Box::new(|| println!("Should be done eventually"))));
//! heap.push(Task (1, Box::new(|| println!("Top priority!"))));
//! heap.push(Task (2, Box::new(|| println!("It's important."))));
//! 
//! while let Some(Task(_, task)) = heap.pop() {
//!     (task)();
//! }
//! ```

mod binheap;

pub use crate::binheap::BinHeap;
