use crate::errors::RuntimeError;
use crate::value::Value;

pub struct BuiltinFunction {
    pub name: &'static str,
    pub func: fn(&[Value]) -> Result<Value, RuntimeError>,
    pub min_args: usize,
    pub max_args: Option<usize>,
}
