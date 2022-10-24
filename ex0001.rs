// Project Eurler problem 1 (https://projecteuler.net/problem=1):
// Find the sum of all the multiples of 3 or 5 below 1000.
fn main() {
    let mut count: u64 = 0;
    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            count += i
        }
    }

    println!("Answer: {count}")
}
