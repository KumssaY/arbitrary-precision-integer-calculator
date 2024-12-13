use std::cmp::Ordering;

/// Represents an arbitrarily large integer.
#[derive(Debug, Clone)]
pub struct LargeInt {
    pub sign: i8,          // 1 for positive, -1 for negative
    pub digits: Vec<u8>,   // Digits stored in reverse order
}

impl LargeInt {
    /// Creates a new LargeInt from a sign and a vector of digits.
    pub fn new(sign: i8, digits: Vec<u8>) -> Self {
        let mut digits = digits;
        // Normalize by removing leading zeros
        while digits.len() > 1 && digits.last() == Some(&0) {
            digits.pop();
        }
        // Zero is always positive
        if digits.len() == 1 && digits[0] == 0 {
            LargeInt { sign: 1, digits }
        } else {
            LargeInt { sign, digits }
        }
    }

    /// Constructs a LargeInt representing zero.
    pub fn zero() -> Self {
        LargeInt::new(1, vec![0])
    }

    /// Parses a string into a LargeInt.
    pub fn parse(input: &str) -> Self {
        let input = input.trim();
        let sign = if input.starts_with('-') { -1 } else { 1 };
        let digits = input
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap() as u8)
            .rev()
            .collect();
        LargeInt::new(sign, digits)
    }

    /// Converts the LargeInt back into a string.
    pub fn to_string(&self) -> String {
        let mut result = self
            .digits
            .iter()
            .rev()
            .map(|&d| char::from_digit(d as u32, 10).unwrap())
            .collect::<String>();
        if self.sign == -1 {
            result.insert(0, '-');
        }
        result
    }

    /// Compares the absolute values of two LargeInts.
    pub fn compare_abs(&self, other: &Self) -> Ordering {
        if self.digits.len() != other.digits.len() {
            return self.digits.len().cmp(&other.digits.len());
        }
        for (a, b) in self.digits.iter().rev().zip(other.digits.iter().rev()) {
            if a != b {
                return a.cmp(b);
            }
        }
        Ordering::Equal
    }

    /// Pads the digits of two LargeInts to equal lengths.
    pub fn pad_equal_lengths(a: &Self, b: &Self) -> (Vec<u8>, Vec<u8>) {
        let max_len = a.digits.len().max(b.digits.len());
        let a_padded = a.digits.iter().cloned().chain(std::iter::repeat(0).take(max_len - a.digits.len())).collect();
        let b_padded = b.digits.iter().cloned().chain(std::iter::repeat(0).take(max_len - b.digits.len())).collect();
        (a_padded, b_padded)
    }

    /// Adds two LargeInts.
    pub fn add(&self, other: &Self) -> Self {
        match (self.sign, other.sign) {
            (1, 1) => self.add_same_sign(other),
            (-1, -1) => {
                let mut result = self.add_same_sign(other);
                result.sign = -1;
                result
            }
            (1, -1) => self.subtract_abs(other),
            (-1, 1) => {
                let mut result = other.subtract_abs(self);
                result.sign = -1;
                result
            }
            _ => unreachable!(),
        }
    }

    /// Subtracts two LargeInts.
    pub fn subtract(&self, other: &Self) -> Self {
        match (self.sign, other.sign) {
            (1, 1) => self.subtract_abs(other),
            (-1, -1) => {
                let mut result = other.subtract_abs(self);
                result.sign = -result.sign;
                result
            }
            (1, -1) => self.add_same_sign(other),
            (-1, 1) => {
                let mut result = self.add_same_sign(other);
                result.sign = -1;
                result
            }
            _ => unreachable!(),
        }
    }

    /// Adds two LargeInts with the same sign.
    pub fn add_same_sign(&self, other: &Self) -> Self {
        let mut result_digits = Vec::new();
        let mut carry = 0;

        let (a_padded, b_padded) = LargeInt::pad_equal_lengths(self, other);

        for (a, b) in a_padded.into_iter().zip(b_padded.into_iter()) {
            let sum = a + b + carry;
            result_digits.push(sum % 10);
            carry = sum / 10;
        }

        if carry > 0 {
            result_digits.push(carry);
        }

        LargeInt::new(self.sign, result_digits)
    }

    /// Subtracts the absolute values of two LargeInts.
    pub fn subtract_abs(&self, other: &Self) -> Self {
        match self.compare_abs(other) {
            Ordering::Equal => LargeInt::zero(),
            Ordering::Greater => self.subtract_same_sign(other),
            Ordering::Less => {
                let mut result = other.subtract_same_sign(self);
                result.sign = -result.sign;
                result
            }
        }
    }

    /// Subtracts two LargeInts with the same sign.
    pub fn subtract_same_sign(&self, other: &Self) -> Self {
        let mut result_digits = Vec::new();
        let mut borrow = 0;

        let (a_padded, b_padded) = LargeInt::pad_equal_lengths(self, other);

        for (a, b) in a_padded.into_iter().zip(b_padded.into_iter()) {
            let mut diff = a as i16 - b as i16 - borrow;
            if diff < 0 {
                diff += 10;
                borrow = 1;
            } else {
                borrow = 0;
            }
            result_digits.push(diff as u8);
        }

        LargeInt::new(self.sign, result_digits)
    }
    pub fn is_zero(&self) -> bool {
        self.digits.len() == 1 && self.digits[0] == 0
    }
    pub fn one() -> Self {
        LargeInt::new(1, vec![1])
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let a = LargeInt::parse("123");
        let b = LargeInt::parse("456");
        assert_eq!(a.add(&b).to_string(), "579");

        let a = LargeInt::parse("-123");
        let b = LargeInt::parse("456");
        assert_eq!(a.add(&b).to_string(), "333");
    }

    #[test]
    fn test_subtraction() {
        let a = LargeInt::parse("456");
        let b = LargeInt::parse("123");
        assert_eq!(a.subtract(&b).to_string(), "333");

        let a = LargeInt::parse("123");
        let b = LargeInt::parse("456");
        assert_eq!(a.subtract(&b).to_string(), "-333");
    }

    #[test]
    fn test_large_numbers() {
        let a = LargeInt::parse("12345678901234567890");
        let b = LargeInt::parse("98765432109876543210");
        assert_eq!(a.add(&b).to_string(), "111111111011111111100");
    }
}
