/// Converts a number from one base to another.
/// Supports bases from 2 to 36.
pub fn convert_base(number: &str, from_base: u32, to_base: u32) -> Result<String, String> {
  if from_base < 2 || from_base > 36 || to_base < 2 || to_base > 36 {
      return Err("Base must be between 2 and 36".to_string());
  }

  let decimal_value = match u128::from_str_radix(number, from_base) {
      Ok(value) => value,
      Err(_) => return Err("Invalid number for the given base".to_string()),
  };

  Ok(decimal_to_base(decimal_value, to_base))
}

/// Converts a decimal number to a string representation in the specified base.
fn decimal_to_base(mut number: u128, base: u32) -> String {
  if number == 0 {
      return "0".to_string();
  }

  let mut result = String::new();
  while number > 0 {
      let remainder = (number % base as u128) as u8;
      let digit = if remainder < 10 {
          (b'0' + remainder) as char
      } else {
          (b'a' + (remainder - 10)) as char
      };
      result.push(digit);
      number /= base as u128;
  }

  result.chars().rev().collect()
}

/// Converts a number from a given base to decimal.
pub fn to_decimal(number: &str, from_base: u32) -> Result<u128, String> {
  if from_base < 2 || from_base > 36 {
      return Err("Base must be between 2 and 36".to_string());
  }

  match u128::from_str_radix(number, from_base) {
      Ok(value) => Ok(value),
      Err(_) => Err("Invalid number for the given base".to_string()),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_convert_base() {
      assert_eq!(convert_base("1010", 2, 10).unwrap(), "10");
      assert_eq!(convert_base("10", 10, 2).unwrap(), "1010");
      assert_eq!(convert_base("ff", 16, 2).unwrap(), "11111111");
      assert_eq!(convert_base("z", 36, 10).unwrap(), "35");
      assert!(convert_base("1010", 2, 37).is_err());
  }

  #[test]
  fn test_to_decimal() {
      assert_eq!(to_decimal("1010", 2).unwrap(), 10);
      assert_eq!(to_decimal("a", 16).unwrap(), 10);
      assert!(to_decimal("1010", 37).is_err());
  }
}
