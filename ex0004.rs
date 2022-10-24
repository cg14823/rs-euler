// Project Euler problem 4 https://projecteuler.net/problem=4
// Find the largest palindrome made from the product of two 3-digit numbers.

fn main() {
    // computers are pretty fast so exahusting all the options is easy
    let largest = largest_palindrome_brute_force();
    println!("Answer: {largest}")
}

fn largest_palindrome_brute_force() -> u64 {
    let mut largest: u64 = 0;
    let mut iterations: u64 = 0;
    for x in 100..1000 {
        for y in 100..1000 {
            let mult = x * y;
            if mult > largest
                && (mult.to_string() == mult.to_string().chars().rev().collect::<String>())
            {
                largest = mult;
            }
            iterations += 1;
        }
    }

    return largest;
}
