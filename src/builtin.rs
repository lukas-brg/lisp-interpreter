use crate::errors::RuntimeError;
use crate::value::Value;

#[allow(dead_code)]
pub struct BuiltinFunction {
    pub name: &'static str,
    pub func: fn(&[Value]) -> Result<Value, RuntimeError>,
    pub min_args: usize,
    pub max_args: Option<usize>,
}

#[allow(dead_code)]
trait BuiltinFunctionT {
    fn eval(args: &[Value]) -> Result<Value, RuntimeError>;
}
