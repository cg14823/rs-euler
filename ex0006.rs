// Project Euler problem 6 https://projecteuler.net/problem=6
// Find the difference between the sum of the squares of the
// first one hundred natural numbers and the square of the sum.

fn main() {
    let math_answer = math_approach(100);
    println!("Answer: {answer} == {math_answer}");
}

#[allow(dead_code)]
fn brute_force_approach(n: u64) -> u64 {
    let mut squared_sums = 0;
    let mut sum = 0;
    for i in 1..(n + 1) {
        sum += i;
        squared_sums += i * i;
    }

    return sum * sum - squared_sums;
}

#[allow(dead_code)]
fn gauss_sumation(n: u64) -> u64 {
    return (n * (n + 1)) / 2;
}

#[allow(dead_code)]
fn sum_of_squares(n: u64) -> u64 {
    return (n * (n + 1) * (2 * n + 1)) / 6;
}

fn math_approach(n: u64) -> u64 {
    let sum_of_first_n_numbers = gauss_sumation(n);
    return sum_of_first_n_numbers * sum_of_first_n_numbers - sum_of_squares(n);
}
