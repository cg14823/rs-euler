// Project Euler problem 5 https://projecteuler.net/problem=5
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
fn main() {
    let min_divisible = euclids_algorithm();
    println!("Answer: {min_divisible}");
}

#[allow(dead_code)]
fn simple_brute_force_approach() -> u64 {
    let mut min_number: u64 = 20;
    loop {
        let mut divisible = true;
        for i in 1..21 {
            if min_number % i != 0 {
                divisible = false;
                break;
            }
        }

        if divisible {
            return min_number;
        }

        // it has to be divisble by 20 so we can go up by factors of 20.
        min_number += 20;
    }
}

// The least common multiple (lcm) of a and b is their product divided by their greatest
// common divisor (gcd) ( i.e. lcm(a, b) = ab/gcd(a,b)).
fn euclids_algorithm() -> u64 {
    let mut lcm = 1;
    for i in 1..21 {
        lcm *= i / gcd(i, lcm);
    }

    return lcm;
}

// Euclidean algorithm to get the greatest common denominator
fn gcd(a: u64, b: u64) -> u64 {
    let mut val1;
    let mut val2;
    if a > b {
        val1 = a;
        val2 = b
    } else {
        val1 = b;
        val2 = a;
    }

    while val1 > 0 && val2 > 0 {
        let remainder = val1 % val2;

        val1 = val2;
        val2 = remainder;
    }

    if val1 == 0 {
        return val2;
    }

    return val1;
}
