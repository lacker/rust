use std::collections::BinaryHeap;

// Prints out numbers whose only prime factors are 2, 3, and 5.
fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(-1);

    let mut last = 0;
    loop {
        let i = heap.pop();
        let x = match i {
            Some(n) => -n,
            None => 0,
        };
        if x > last {
            println!("{}", x);
            last = x;
            heap.push(-2 * x);
            heap.push(-3 * x);
            heap.push(-5 * x);
        }
    }
}
