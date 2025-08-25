
use std::result::Result;

fn even_odd(x: u32) -> &'static str {
    if x % 2 == 0 {
        "Even"
    } else {
        "Odd"
    }
}

fn count_words(str : &str) -> u32 {
    let mut num: u32 = 1;
    let mut count = 0;
    let len  = str.len();
    println!("length {}", len);
    for i in str.chars() {
        count += 1;
        if i == ' ' && count == 1 {
            continue;
        } 
        if i == ' '  && count < len{
            num +=1;
        }
    }
    num
}

fn sum_limit(limit: u32) -> u32 {
    let mut sum: u32 = 0;
    for i in 1..limit {
        sum += i;
    }
    sum
}

// fn take_ownership(s: String)  {
//     let strr: String = String::(s);
//     println!("borrowed string {}", strr);
//     // println!("original string {}",s);
// }

fn check_sign(n: i32) -> String {

    let mut word : String = String::from("");
    if n == 0 {
        word.push_str("Zero")
    } else if n > 0 {
        word.push_str("Positive")
    } else {
        word.push_str("Negative")
    }

    return word
}

fn day_type(day: &str) -> String {
    let mut word = String::new();

    match day {
        "Saturday" | "Sunday" => word.push_str("weekend"),
        _ => word.push_str("weekday"),
    }

    word
}

fn sum_limit_loop(mut limit: u32) -> u32 {
    let mut sum: u32 = 0;

    loop {
        sum += limit;

        limit -= 1;

        if limit == 0 {
            break;
        }
    }
    sum
}

fn borrowing_reference(s: &String) {
    println!("{}", s.len());
}

fn mutable_borrow(s: &mut String) {
    s.push_str(" world");
    println!("{}", s);
}

fn first_word(s: &str) -> &str {
    &s[..1]
}

fn safe_divide(a: u32, b: u32) -> Result<u32, String> {
    if b == 0 {
        Err("dividing by 0".to_string())
    } else {
        Ok(a/b)
    }
}


fn main() {
    mutable_borrow(&mut "Ishita".to_string());
    let answer: Result<u32, String> = safe_divide(1,2);
    println!("{}",answer);
}
