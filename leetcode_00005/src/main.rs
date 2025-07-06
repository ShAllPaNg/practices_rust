use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let res = Solution::longest_palindrome(input);
    println!("res = {res}");
}

struct Solution {}

impl Solution {
    pub fn update_length(
        char_vec: &Vec<char>,
        mut left: usize,
        mut right: usize,
        res_right: &mut usize,
        res_left: &mut usize,
    ) {
        let mut left_char = char_vec.get(left);
        let mut right_char = char_vec.get(right);
        while left_char.is_some()
            && right_char.is_some()
            && *(left_char.unwrap()) == *(right_char.unwrap())
        {
            if right - left > *res_right - *res_left {
                (*res_left, *res_right) = (left, right);
            }
            left = left.wrapping_sub(1);
            right = right.wrapping_add(1);
            left_char = char_vec.get(left);
            right_char = char_vec.get(right);
        }
    }

    pub fn longest_palindrome(s: String) -> String {
        let char_vec: Vec<char> = s.chars().collect();
        let (mut res_left, mut res_right) = (0, 0);
        for (index, ch) in char_vec.iter().enumerate() {
            let right_char = char_vec.get(index.wrapping_add(1));
            if right_char.is_some() {
                //检查偶数长度字符串
                if *(right_char.unwrap()) == *ch {
                    let (left, right) = (index, index.wrapping_add(1));
                    Solution::update_length(&char_vec, left, right, &mut res_right, &mut res_left);
                }
                let (left, right) = (index, index);
                Solution::update_length(&char_vec, left, right, &mut res_right, &mut res_left);
            }
        }

        let result = String::from_iter(&char_vec[res_left..=res_right]);
        result
    }
}
