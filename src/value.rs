use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    None,
}

impl Value {
    pub fn promote_to_float(self) -> Self {
        match self {
            Value::Int(i) => Value::Float(i as f64),
            other => other,
        }
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, Value::Int(_) | Value::Float(_))
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

        *self = Value::Float(result);
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

        *self = Value::Float(result);
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
        }
    }
}
