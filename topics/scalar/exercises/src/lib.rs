pub fn eq(a: char, b: char) -> bool {
    return a == b;
}

pub fn add(a: f32, b: f32, c: f32) -> f32 {
    return a+b+c;
}

pub fn cast(x: u8, y: i8, z: f32) -> f32 {
    let u: f32 = x as f32;
    let v: f32 = y as f32;
    return u + v + z;
}
