fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("input failed");
    let x: i32 = input.trim().parse().expect("input is not a valid integer");
    let ret = Solution::reverse(x);
    println!("{ret}");
    let ret2 = Solution2::reverse(x);
    println!("{ret2}");
}

struct Solution {}
// Solution for LeetCode problem 7: Reverse Integer
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut ret: i32 = 0;
        let mut x = x;
        while x != 0 {
            ret = match ret.checked_mul(10) {
                Some(value) => value,
                None => return 0, // overflow
            };
            ret = match ret.checked_add(x % 10) {
                Some(value) => value,
                None => return 0, // overflow
            };
            x /= 10;
        }
        ret
    }
}

struct Solution2 {}
// Another solution for LeetCode problem 7: Reverse Integer
impl Solution2 {
    pub fn reverse(x: i32) -> i32 {
        let mut ret: i32 = 0;
        let mut x = x;
        while x != 0 {
            if ret > i32::MAX / 10 || ret < i32::MIN / 10 {
                return 0; // overflow
            }
            ret = ret * 10 + x % 10;
            x /= 10;
        }
        ret
    }
}
