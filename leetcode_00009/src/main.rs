use core::num;

fn main() {
    let mut str = String::new();
    std::io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");
    let x = str.trim().parse::<i32>().unwrap();
    let ret = Solution::is_palindrome(x);
    println!("{}", ret);
}

struct Solution {}
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }
        let mut num = x;
        let mut rev_num: i32 = 0;
        while num > rev_num {
            rev_num = rev_num * 10 + num % 10;
            num /= 10;
        }
        num == rev_num || num == rev_num / 10 //123321, 12321
    }
}
