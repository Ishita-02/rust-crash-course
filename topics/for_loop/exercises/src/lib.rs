pub fn sum(nums: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for n in nums {
        sum += n;
    }

    return sum;
}

pub fn fill(i: u32, n: usize) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();
    let mut num: usize = 0;

    while num < n {
        vec.push(i);
        num += 1;
    }

    return vec;
}
