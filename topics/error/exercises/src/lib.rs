#[derive(Debug, PartialEq)]
pub enum MathError {
    DivByZero,
}

pub fn div(x: u32, y: u32) -> Result<u32, MathError> {
    if y != 0 {
        return Ok(x/y);
    } else {
        return Err(MathError::DivByZero);
    }
}

pub fn get(v: &[u32], i: usize, default_val: u32) -> u32 {
    if 1 > 0 && i < v.len() {
        return v[i];
    } else {
        return default_val;
    }
}
