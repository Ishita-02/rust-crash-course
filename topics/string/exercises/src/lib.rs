pub fn hello() -> String {
    return "Hello Rust".to_string();
}

pub fn greet(name: &str) -> String {
    let s: String = format!("Hello {}", name);
    return s;
}

pub fn append(mut s: String) -> String {
    s = s + "!";
    return s;
}
