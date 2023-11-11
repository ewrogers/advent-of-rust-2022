use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

pub enum BigIntParseError {
    InvalidCharacter,
}

#[derive(Debug, Clone)]
pub struct BigInt {
    value: String,
}

impl BigInt {
    pub fn from_value<T>(value: &T) -> Self
    where
        T: Add + Sub + Mul + Div + Display,
    {
        Self {
            value: value.to_string(),
        }
    }

    #[must_use]
    pub fn add(&self, other: &Self) -> Self {
        let sum = add_string_numbers(&self.value, &other.value);
        BigInt { value: sum }
    }

    #[must_use]
    pub fn multiply_by(&self, other: &Self) -> Self {
        let product = multiply_string_numbers(&self.value, &other.value);
        BigInt { value: product }
    }

    #[must_use]
    pub fn squared(&self) -> Self {
        let square = multiply_string_numbers(&self.value, &self.value);
        BigInt { value: square }
    }

    #[must_use]
    pub fn divide_by(&self, other: &BigInt) -> Self {
        let quotient = divide_string_numbers(&self.value, &other.value);
        BigInt {
            value: quotient.unwrap(),
        }
    }

    pub fn divisible_by(&self, divisor: u32) -> Result<bool, String> {
        if divisor == 0 {
            return Ok(false);
        }

        let mut remainder = 0;
        for c in self.value.chars() {
            if let Some(digit) = c.to_digit(10) {
                remainder = (remainder * 10 + digit) % divisor;
            } else {
                return Err("Invalid digit".into());
            }
        }

        Ok(remainder == 0)
    }
}

impl Default for BigInt {
    fn default() -> Self {
        Self {
            value: "0".to_string(),
        }
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FromStr for BigInt {
    type Err = BigIntParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|c| c.is_ascii_digit()) {
            Ok(Self {
                value: s.to_string(),
            })
        } else {
            Err(BigIntParseError::InvalidCharacter)
        }
    }
}

fn add_string_numbers(a: &str, b: &str) -> String {
    let mut a: Vec<char> = a.chars().collect();
    let mut b: Vec<char> = b.chars().collect();

    let mut carry = 0;
    let mut result = Vec::with_capacity(a.len() + b.len());

    while !a.is_empty() || !b.is_empty() || carry > 0 {
        let mut sum = carry;

        if let Some(&digit) = a.last() {
            sum += digit.to_digit(10).unwrap();
            a.pop();
        }

        if let Some(&digit) = b.last() {
            sum += digit.to_digit(10).unwrap();
            b.pop();
        }

        carry = sum / 10;
        result.push(char::from_digit(sum % 10, 10).unwrap());
    }

    result.iter().rev().collect()
}

fn multiply_string_numbers(a: &str, b: &str) -> String {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();

    let mut result = vec![0; a.len() + b.len()];

    for (a_idx, &a_char) in a.iter().rev().enumerate() {
        for (b_idx, &b_char) in b.iter().rev().enumerate() {
            let prod = a_char.to_digit(10).unwrap() * b_char.to_digit(10).unwrap();
            let sum = prod + result[a_idx + b_idx];

            result[a_idx + b_idx] = sum % 10;
            result[a_idx + b_idx + 1] += sum / 10;
        }
    }

    while let Some(&0) = result.last() {
        result.pop();
    }

    result
        .into_iter()
        .rev()
        .map(|d| char::from_digit(d, 10).unwrap())
        .collect()
}

fn divide_string_numbers(dividend: &str, divisor: &str) -> Result<String, String> {
    if divisor == "0" {
        return Err("Division by zero".into());
    }

    let mut result: Vec<char> = Vec::new();
    let mut remainder: i128 = 0;
    let divisor = divisor.parse::<i128>().unwrap();

    for digit_char in dividend.chars() {
        let digit = i128::from(digit_char.to_digit(10).unwrap());
        remainder = remainder * 10 + digit;

        let quotient_digit = remainder / divisor;
        remainder %= divisor;

        result.push(char::from_digit(u32::try_from(quotient_digit).unwrap(), 10).unwrap());
    }

    // Remove leading zeros
    while result.len() > 1 && result[0] == '0' {
        result.remove(0);
    }

    Ok(result.iter().collect())
}
