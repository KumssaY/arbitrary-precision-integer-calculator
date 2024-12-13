// Implemantation for handling fractions/// A module for representing and working with fractions in Rust.
/// It supports mixed fractions, proper/improper fractions, and handling of negative exponents.
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use num::integer::gcd;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fraction {
    pub numerator: i64,
    pub denominator: i64,
}

impl Fraction {
    /// Creates a new Fraction and normalizes it.
    pub fn new(numerator: i64, denominator: i64) -> Self {
        if denominator == 0 {
            panic!("Denominator cannot be zero!");
        }

        let sign = if denominator < 0 { -1 } else { 1 };
        let gcd = gcd(numerator.abs(), denominator.abs());
        
        Fraction {
            numerator: numerator / gcd * sign,
            denominator: denominator.abs() / gcd,
        }
    }

    /// Converts the fraction into a mixed fraction form (whole part and remaining fraction).
    pub fn to_mixed(&self) -> (i64, Fraction) {
        let whole_part = self.numerator / self.denominator;
        let remainder = self.numerator % self.denominator;
        
        (whole_part, Fraction::new(remainder, self.denominator))
    }

    /// Calculates the reciprocal of the fraction.
    pub fn reciprocal(&self) -> Self {
        if self.numerator == 0 {
            panic!("Cannot find reciprocal of zero!");
        }
        Fraction::new(self.denominator, self.numerator)
    }

    /// Exponentiates the fraction to the power of an integer (positive or negative).
    pub fn exponentiate(&self, exp: i64) -> Self {
        if exp == 0 {
            return Fraction::new(1, 1);
        }

        let base = if exp > 0 {
            Fraction::new(self.numerator.pow(exp as u32), self.denominator.pow(exp as u32))
        } else {
            self.reciprocal().exponentiate(-exp)
        };

        base
    }

    /// Checks if the fraction is proper.
    pub fn is_proper(&self) -> bool {
        self.numerator.abs() < self.denominator.abs()
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_proper() {
            write!(f, "{}/{}", self.numerator, self.denominator)
        } else {
            let (whole, remainder) = self.to_mixed();
            if remainder.numerator == 0 {
                write!(f, "{}", whole)
            } else {
                write!(f, "{} {}/{}", whole, remainder.numerator.abs(), remainder.denominator)
            }
        }
    }
}

impl Add for Fraction {
    type Output = Fraction;

    fn add(self, other: Fraction) -> Fraction {
        let numerator = self.numerator * other.denominator + other.numerator * self.denominator;
        let denominator = self.denominator * other.denominator;
        Fraction::new(numerator, denominator)
    }
}

impl Sub for Fraction {
    type Output = Fraction;

    fn sub(self, other: Fraction) -> Fraction {
        let numerator = self.numerator * other.denominator - other.numerator * self.denominator;
        let denominator = self.denominator * other.denominator;
        Fraction::new(numerator, denominator)
    }
}

impl Mul for Fraction {
    type Output = Fraction;

    fn mul(self, other: Fraction) -> Fraction {
        let numerator = self.numerator * other.numerator;
        let denominator = self.denominator * other.denominator;
        Fraction::new(numerator, denominator)
    }
}

impl Div for Fraction {
    type Output = Fraction;

    fn div(self, other: Fraction) -> Fraction {
        self * other.reciprocal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraction_creation() {
        let frac = Fraction::new(6, 8);
        assert_eq!(frac, Fraction::new(3, 4));
    }

    #[test]
    fn test_mixed_fraction() {
        let frac = Fraction::new(7, 3);
        assert_eq!(frac.to_mixed(), (2, Fraction::new(1, 3)));
    }

    #[test]
    fn test_exponentiation_positive() {
        let frac = Fraction::new(2, 3);
        assert_eq!(frac.exponentiate(2), Fraction::new(4, 9));
    }

    #[test]
    fn test_exponentiation_negative() {
        let frac = Fraction::new(2, 3);
        assert_eq!(frac.exponentiate(-2), Fraction::new(9, 4));
    }

    #[test]
    fn test_addition() {
        let a = Fraction::new(1, 2);
        let b = Fraction::new(1, 3);
        assert_eq!(a + b, Fraction::new(5, 6));
    }

    #[test]
    fn test_multiplication() {
        let a = Fraction::new(2, 3);
        let b = Fraction::new(3, 4);
        assert_eq!(a * b, Fraction::new(1, 2));
    }

    #[test]
    fn test_display_mixed() {
        let frac = Fraction::new(7, 3);
        assert_eq!(format!("{}", frac), "2 1/3");
    }
}
