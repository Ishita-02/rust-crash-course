pub fn num_to_string(num: u32) -> String {
    let mut string: String = "".to_string();
    match num {
        0 => string = "zero".to_string(),
        1 => string = "one".to_string(),
        2 => string = "two".to_string(),
        3 => string = "three".to_string(),
        _ => string = "other".to_string(),
    }

    return string;
}

pub fn unwrap_or_default(x: Option<u32>, v: u32) -> u32 {
    match x {
        Some(x) => return x,
        None => return v
    }
}
