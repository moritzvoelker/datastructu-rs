use std::cmp::Reverse;

use binheap::BinHeap;

fn main() {
    let mut heap = BinHeap::new();
    heap.push(Reverse((3, 'b')));
    heap.push(Reverse((3, 'a')));
    heap.push(Reverse((8, 'x')));
    heap.push(Reverse((10, 'z')));

    println!("Root: {:?}", heap.root().unwrap());
    println!("Values:");

    while let Some(value) = heap.pop() {
        println!("{value:?}");
    }
}