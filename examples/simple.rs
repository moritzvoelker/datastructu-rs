use binheap::BinHeap;

fn main() {
    let mut heap = BinHeap::new();
    heap.push(3);
    heap.push(5);
    heap.push(8);
    heap.push(10);


    println!("Root: {}", heap.min().unwrap());
    println!("Values:");

    while let Some(value) = heap.pop() {
        println!("{value}");
    }
}