// Project Euler problem 3 (https://projecteuler.net/problem=3)
// What is the largest prime factor of the number 600851475143 ?

fn main() {
    let prime_number = find_largest_prime_factor(600851475143);
    println!("Answer: {prime_number}")
}

// 0(n^0.5) complexity approach

fn find_largest_prime_factor(n: u64) -> u64 {
    let mut max_prime: u64 = 0;
    let mut current = n;

    // Devide by 2 until it is no longer divisible by two
    while current & 1 == 0 {
        max_prime = 2;
        current >>= 1
    }

    // Devide by 3 until it is no longer divisible by 3
    while current % 3 == 0 {
        max_prime = 3;
        current /= 3;
    }

    for i in (5..((current as f64).sqrt() as u64) + 1).step_by(6) {
        while current % i == 0 {
            max_prime = i;
            current /= i;
        }

        while current % (i + 2) == 0 {
            max_prime = i + 2;
            current /= i + 2;
        }
    }

    if current > 4 {
        return current;
    }

    return max_prime;
}
