// Project Euler problem 7 https://projecteuler.net/problem=7
// What is the 10 001st prime number?

fn main() {
    println!("Answer: {}", brute_force_prime_search(10_001));
}

// The brute force approach is simple start with a number bigger than 1 and try to devided by
// all the prime numbers before it if it is divisible by any then it is not prime, if it is not
// divisible then it is a prime number.
fn brute_force_prime_search(nth: i64) -> u64 {
    // Initialize with some of the first prime numbers
    // nth has to be bigger than 6 in this case as we pre-populate with 6 primes.
    let mut prime_numbers: Vec<u64> = vec![2, 3, 5, 7, 11, 13];
    let mut current_val: u64 = 14;

    while prime_numbers.len() < (nth as usize) {
        let mut is_prime = true;
        // We can save some time but only looking up to the square root of the number
        let square_root = (current_val as f64).sqrt() as u64;
        for prime in prime_numbers.iter() {
            if prime > &square_root {
                break;
            }

            if current_val % prime == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            prime_numbers.push(current_val);
        }

        current_val += 1;
    }

    return *prime_numbers.last().unwrap();
}
