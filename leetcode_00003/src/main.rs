use std::{collections::HashMap, io};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");

    input = String::from(input.trim());

    let result = Solution::length_of_longest_substring(input);

    println!("{result}");
}

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_num: HashMap<char, usize> = HashMap::new();
        let mut cur_len = 0;
        let mut result = cur_len;
        let mut left: usize = 0;
        let vec_char: Vec<char> = s.chars().collect();
        for (index, cur_char) in vec_char.iter().enumerate() {
            if char_num.get(cur_char) == None || *(char_num.get(cur_char).unwrap()) == 0 {
                cur_len = index - left + 1;
                result = if cur_len > result { cur_len } else { result };
                char_num.insert(*cur_char, 1);
                continue;
            }
            while vec_char[left] != *cur_char {
                let value = char_num.get_mut(&vec_char[left]).unwrap();
                *value = *value - 1;
                left += 1;
            }
            left += 1;
        }
        result as i32
    }
}
