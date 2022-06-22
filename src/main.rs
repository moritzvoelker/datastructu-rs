mod binheap;

use crate::binheap::{BinHeap, Asc, Desc};

fn main() {
    println!("=== Example 1 ===");
    example1();
    println!("\n\n=== Example 2 ===");
    example2();
    println!("\n\n=== Example 3 ===");
    example3();
}


struct Task (i32, Box<dyn Fn() -> ()>);
impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for Task { }
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn example1() {
    let mut heap = BinHeap::new(Asc);
    heap.push(3);
    heap.push(5);
    heap.push(8);
    heap.push(10);


    println!("Root: {}", heap.root().unwrap());
    println!("Values:");

    while let Some(value) = heap.pop() {
        println!("{value}");
    }
}

fn example2() {
    let mut heap = BinHeap::new(Desc);
    heap.push((3, 'b'));
    heap.push((3, 'a'));
    heap.push((8, 'x'));
    heap.push((10, 'z'));

    println!("Root: {:?}", heap.root().unwrap());
    println!("Values:");

    while let Some(value) = heap.pop() {
        println!("{value:?}");
    }
}

fn example3() {
    let mut heap = BinHeap::new(Asc);

    heap.push(Task (3, Box::new(|| println!("Should be done eventually"))));
    heap.push(Task (1, Box::new(|| println!("Top priority!"))));
    heap.push(Task (2, Box::new(|| println!("It's important."))));

    while let Some(Task(_, task)) = heap.pop() {
        (task)();
    }
}
