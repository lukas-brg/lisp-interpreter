use crate::errors::RuntimeError;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumBase {
    Dec,
    Hex,
    Oct,
    Bin,
}
impl NumBase {
    pub fn is_valid_digit(self, digit: char) -> bool {
        match self {
            NumBase::Dec => digit.is_numeric(),
            NumBase::Bin => digit == '0' || digit == '1',
            NumBase::Oct => ('0'..'7').contains(&digit),
            NumBase::Hex => digit.is_ascii_hexdigit(),
        }
    }

    pub fn parse_int(self, num_str: &str) -> Result<i64, std::num::ParseIntError> {
        match self {
            NumBase::Dec => num_str.parse(),
            NumBase::Bin => i64::from_str_radix(num_str.trim_start_matches("0b"), 2),
            NumBase::Oct => i64::from_str_radix(num_str.trim_start_matches("0o"), 8),
            NumBase::Hex => i64::from_str_radix(num_str.trim_start_matches("0x"), 16),
        }
    }

    pub fn parse_float(self, num_str: &str) -> Result<f64, std::num::ParseFloatError> {
        match self {
            NumBase::Dec => num_str.parse(),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    _List(Vec<Value>),
    None,
}

impl Value {
    #[allow(dead_code)]
    pub fn promote_to_float(self) -> Self {
        match self {
            Value::Int(i) => Value::Float(i as f64),
            other => other,
        }
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, Value::Int(_) | Value::Float(_))
    }

    pub fn get_numeric_value_as_float(&self) -> Option<f64> {
        match *self {
            Value::Int(v) => Some(v as f64),
            Value::Float(v) => Some(v),
            _ => None,
        }
    }

    pub fn int_div_assign(&mut self, rhs: Value) {
        let result: f64 = match (&*self, rhs) {
            (Value::Int(l), Value::Int(r)) => *l as f64 / r as f64,
            (Value::Float(l), Value::Float(r)) => *l / r,
            (Value::Int(l), Value::Float(r)) => (*l as f64) / r,
            (Value::Float(l), Value::Int(r)) => *l / (r as f64),
            _ => unreachable!(),
        };
        *self = Value::Int(result as i64);
    }

    pub fn pow_assign(&mut self, rhs: Self) {
        if !(self.is_numeric() && rhs.is_numeric()) {}

        let result: f64 = match (&mut *self, rhs) {
            (Value::Int(l), Value::Int(r)) => (*l as f64).powf(r as f64),
            (Value::Float(l), Value::Float(r)) => l.powf(r),
            (Value::Int(l), Value::Float(r)) => (*l as f64).powf(r),
            (Value::Float(l), Value::Int(r)) => l.powf(r as f64),
            _ => unreachable!(),
        };

        if result.fract() == 0.0 {
            *self = Value::Int(result as i64);
        } else {
            *self = Value::Float(result);
        }
    }

    pub fn negate(&mut self) {
        match self {
            Value::Boolean(v) => *v = !(*v),
            Value::Float(v) => *v *= -1.0,
            Value::Int(v) => *v *= -1,
            _ => unreachable!(),
        }
    }

    pub fn compare_to(&self, rhs: &Self) -> Result<i64, RuntimeError> {
        if !(self.is_numeric() && rhs.is_numeric()) {
            let msg = format!(
                "Incompatible Types for comparision: {:?} and {:?}",
                self, rhs
            );
            return Err(RuntimeError::new(msg));
        }

        let l = self.get_numeric_value_as_float().unwrap();
        let r = rhs.get_numeric_value_as_float().unwrap();
        let cmp = l - r;
        if cmp < 0.0 {
            return Ok(-1);
        } else if cmp > 0.0 {
            return Ok(1);
        }
        Ok(0)
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => Value::Int(l + r),
            (Value::Float(l), Value::Float(r)) => Value::Float(l + r),
            (Value::Int(l), Value::Float(r)) => Value::Float(l as f64 + r),
            (Value::Float(l), Value::Int(r)) => Value::Float(l + r as f64),
            (Value::String(l), Value::String(r)) => Value::String(l + &r),
            _ => Value::None,
        }
    }
}

impl std::ops::AddAssign for Value {
    fn add_assign(&mut self, rhs: Value) {
        if let Value::Int(l) = self {
            if let Value::Int(r) = rhs {
                *l += r;
                return;
            }
        }

        let result: f64 = match (&*self, rhs) {
            (Value::Float(l), Value::Float(r)) => *l + r,
            (Value::Int(l), Value::Float(r)) => (*l as f64) + r,
            (Value::Float(l), Value::Int(r)) => *l + (r as f64),
            _ => unreachable!(),
        };

        if result.fract() == 0.0 {
            *self = Value::Int(result as i64);
        } else {
            *self = Value::Float(result);
        }
    }
}

impl std::ops::SubAssign for Value {
    fn sub_assign(&mut self, rhs: Value) {
        if let Value::Int(l) = self {
            if let Value::Int(r) = rhs {
                *l -= r;
                return;
            }
        }

        let result: f64 = match (&*self, rhs) {
            (Value::Float(l), Value::Float(r)) => *l - r,
            (Value::Int(l), Value::Float(r)) => (*l as f64) - r,
            (Value::Float(l), Value::Int(r)) => *l - (r as f64),
            _ => unreachable!(),
        };

        if result.fract() == 0.0 {
            *self = Value::Int(result as i64);
        } else {
            *self = Value::Float(result);
        }
    }
}

impl std::ops::MulAssign for Value {
    fn mul_assign(&mut self, rhs: Value) {
        if let Value::Int(l) = self {
            if let Value::Int(r) = rhs {
                *l *= r;
                return;
            }
        }

        let result: f64 = match (&*self, rhs) {
            (Value::Float(l), Value::Float(r)) => *l * r,
            (Value::Int(l), Value::Float(r)) => (*l as f64) * r,
            (Value::Float(l), Value::Int(r)) => *l * (r as f64),
            _ => unreachable!(),
        };

        if result.fract() == 0.0 {
            *self = Value::Int(result as i64);
        } else {
            *self = Value::Float(result);
        }
    }
}

impl std::ops::RemAssign for Value {
    fn rem_assign(&mut self, rhs: Self) {
        let result: f64 = match (&mut *self, rhs) {
            (Value::Int(l), Value::Int(r)) => {
                *l = ((*l % r) + r) % r;
                return;
            }
            (Value::Float(l), Value::Float(r)) => ((*l % r) + r) % r,
            (Value::Int(l), Value::Float(r)) => ((*l as f64 % r) + r) % r,
            (Value::Float(l), Value::Int(r)) => {
                let r = r as f64;
                ((*l % r) + r) % r
            }
            _ => unreachable!(),
        };

        *self = Value::Float(result);
    }
}

impl std::ops::DivAssign for Value {
    fn div_assign(&mut self, rhs: Value) {
        let result: f64 = match (&*self, rhs) {
            (Value::Int(l), Value::Int(r)) => *l as f64 / r as f64,
            (Value::Float(l), Value::Float(r)) => *l / r,
            (Value::Int(l), Value::Float(r)) => (*l as f64) / r,
            (Value::Float(l), Value::Int(r)) => *l / (r as f64),
            _ => unreachable!(),
        };

        if result.fract() == 0.0 {
            *self = Value::Int(result as i64);
        } else {
            *self = Value::Float(result);
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => Value::Int(l * r),
            (Value::Float(l), Value::Float(r)) => Value::Float(l * r),
            (Value::Int(l), Value::Float(r)) => Value::Float(l as f64 * r),
            (Value::Float(l), Value::Int(r)) => Value::Float(l * r as f64),
            _ => Value::None,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(l), Value::Int(r)) => l == r,
            (Value::Float(l), Value::Float(r)) => (l - r).abs() < f64::EPSILON,
            (Value::Boolean(l), Value::Boolean(r)) => l == r,
            (Value::String(l), Value::String(r)) => l == r,
            _ => false,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::Boolean(v) => write!(f, "{}", v),
            Value::String(v) => write!(f, "\"{}\"", v),
            Value::None => write!(f, "null"),
            other => write!(f, "{:?}", other),
        }
    }
}
