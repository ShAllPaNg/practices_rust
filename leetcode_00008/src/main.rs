use std::io;

fn main() {
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");
    let ret = Solution::my_atoi(str);
    println!("{}", ret);
}

struct Solution {}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut ret: i32 = 0;
        let mut sign = 1;
        let mut str = s.trim();
        match str.chars().next() {
            Some('+') => str = &str[1..],
            Some('-') => {
                str = &str[1..];
                sign = -1;
            }
            _ => {}
        };
        str = str.trim_start_matches('0');
        for c in str.chars() {
            if c.is_digit(10) {
                if let Some(new_ret) = ret.checked_mul(10) {
                    ret = new_ret;
                } else {
                    return if sign == 1 { i32::MAX } else { i32::MIN };
                }
                if let Some(new_ret) = ret.checked_add(c.to_digit(10).unwrap() as i32) {
                    ret = new_ret;
                } else {
                    return if sign == 1 { i32::MAX } else { i32::MIN };
                }
            } else {
                break;
            }
        }
        ret * sign
    }
}
