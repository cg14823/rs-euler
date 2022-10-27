// Project Euler problem 9 https://projecteuler.net/problem= 9
// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which
// a^2 + b^2 = c^2
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn main() {
    let (a, b, c) = brute_force_pythagoras_triplet();
    println!("Answer: {}", a * b * c);
}

// The are not many possible numbers as the values must be smaller than 1000 so this is easy
// to do via brute force
// This solution is horrific but as there is only 999 posible values to check it is blazing fast for
// a computer to do
fn brute_force_pythagoras_triplet() -> (i32, i32, i32) {
    for a in 1..1000 {
        for b in a + 1..1000 {
            let c = 1000 - a - b;
            if a * a + b * b == c * c {
                return (a, b, c);
            }
        }
    }

    panic!("No solution found");
}
