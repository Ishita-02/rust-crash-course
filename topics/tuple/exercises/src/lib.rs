pub fn first(t: (bool, u32, char)) -> bool {
    return t.0;
}

pub fn last(t: (bool, u32, char)) -> char {
    return t.2;
}

pub fn swap(mut t: (u32, u32)) -> (u32, u32) {
    let a : u32 = t.1;
    let b : u32 = t.0;
    t.0 = a;
    t.1 = b;
    return t;
}
