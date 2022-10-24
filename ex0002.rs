// Project Euler problem 2 (https://projecteuler.net/problem=2)
// By considering the terms in the Fibonacci sequence whose values do not exceed four million,
// find the sum of the even-valued terms.

fn main() {
    let mut prev: u64 = 1;
    let mut next: u64 = 2;
    let mut sum: u64 = 0;

    while next < 4_000_000 {
        if next & 1 == 0 {
            sum += next
        }

        let next_temp = prev + next;
        prev = next;
        next = next_temp;
    }

    println!("Answer: {sum}")
}
