use std::cmp::Reverse;

use binheap::BinHeap;

fn main() {
    let mut heap = BinHeap::new();

    let data = [(3, 'b'), (3, 'a'), (8, 'x'), (10, 'z')];

    for datum in data {
        heap.push(Reverse(datum));
    }

    println!("Root: {:?}", heap.min().unwrap().0);
    println!("Values:");

    while let Some(Reverse(value)) = heap.pop() {
        println!("{value:?}");
    }
}