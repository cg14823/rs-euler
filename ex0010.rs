// Project Euler problem 10 https://projecteuler.net/problem= 10
// Find the sum of all the primes below two million.

fn main() {
    println!(
        "Answer {} == {}",
        brute_force_prime_sum(2_000_000),
        sum_sieve_of_eratosthenes(2_000_000)
    );
}

// I am going to re-use some of the logic in the brute force approach of exercise 7
fn brute_force_prime_sum(n: u64) -> u64 {
    // only work if n is smaller than 2
    let mut prime_numbers: Vec<u64> = vec![2];
    let mut current_sum: u64 = 2;

    for i in 3..n {
        let mut is_prime = true;

        // We can save some time but only looking up to the square root of the number
        let square_root = (i as f64).sqrt() as u64;
        for prime in prime_numbers.iter() {
            if prime > &square_root {
                break;
            }

            if i % prime == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            prime_numbers.push(i);
            current_sum += i;
        }
    }

    return current_sum;
}

fn sum_sieve_of_eratosthenes(n: usize) -> usize {
    if n < 2 {
        return 0;
    }

    // ofset by two as we start at the smallest prime so ix = 0 represents 2
    let mut primes = vec![true; n-2];

    for i in 2..(n as f64).sqrt() as usize {
        if !primes[i - 2] {
            continue;
        }

        let mut not_prime = i * i;
        while not_prime < n {
            primes[not_prime - 2] = false;
            not_prime += i;
        }
    }

    let mut current_sum :usize = 0;
    for i in 0..primes.len() {
        if primes[i] {
            current_sum += i + 2;
        }
    }

    return current_sum;
}
