// All Arithmetic operations
mod bigint;

pub use crate::bigint::LargeInt;
use rayon::prelude::*; // Parallel processing using Rayon
use std::cmp::Ordering;

/// Multiplies two LargeInt numbers.
/// Uses parallel processing for efficient grid multiplication.
pub fn multiply(a: &LargeInt, b: &LargeInt) -> LargeInt {
    let mut result = vec![0; a.digits.len() + b.digits.len()];

    a.digits.par_iter().enumerate().for_each(|(i, &da)| {
        let mut carry = 0;
        for (j, &db) in b.digits.iter().enumerate() {
            let temp = result[i + j] + da * db + carry;
            result[i + j] = temp % 10;
            carry = temp / 10;
        }
        if carry > 0 {
            result[i + b.digits.len()] += carry;
        }
    });

    let mut product = LargeInt::new(a.sign * b.sign, result);
    product.normalize();
    product
}

/// Performs division and modulo operations simultaneously.
/// Returns a tuple (quotient, remainder).
/// Panics if division by zero is attempted.
pub fn divide_and_modulo(a: &LargeInt, b: &LargeInt) -> (LargeInt, LargeInt) {
    if b.is_zero() {
        panic!("Division by zero is not allowed!");
    }

    let mut quotient = Vec::new();
    let mut remainder = LargeInt::zero();

    for &digit in a.digits.iter().rev() {
        remainder.digits.insert(0, digit);
        remainder.normalize();

        let mut count = 0;
        while remainder.compare_abs(b) != Ordering::Less {
            remainder = remainder.subtract_same_sign(b);
            count += 1;
        }

        quotient.push(count as u8);
    }

    quotient.reverse();
    (
        LargeInt::new(a.sign * b.sign, quotient),
        remainder,
    )
}

/// Exponentiates a LargeInt to the power of another LargeInt.
/// Uses the binary exponentiation method for efficiency.
pub fn exponentiate(base: &LargeInt, exp: &LargeInt) -> LargeInt {
    if exp.is_zero() {
        return LargeInt::one();
    }

    let mut result = LargeInt::one();
    let mut base = base.clone();
    let mut exp = exp.clone();

    while !exp.is_zero() {
        if exp.digits[0] % 2 == 1 {
            result = multiply(&result, &base);
        }
        base = multiply(&base, &base);
        exp = divide_and_modulo(&exp, &LargeInt::new(1, vec![2])).0;
    }

    result
}

/// Computes the factorial of a LargeInt.
/// Uses parallel reduction for efficient computation.
pub fn factorial(n: &LargeInt) -> LargeInt {
    if n.is_zero() {
        return LargeInt::one();
    }

    if n.sign == -1 {
        panic!("Factorial is not defined for negative numbers!");
    }

    let one = LargeInt::one();
    let range: Vec<LargeInt> = (1..=n.to_string().parse::<usize>().unwrap())
        .map(|x| LargeInt::parse(&x.to_string()))
        .collect();

    range
        .par_iter()
        .cloned()
        .reduce(|| one.clone(), |acc, x| multiply(&acc, &x))
}
