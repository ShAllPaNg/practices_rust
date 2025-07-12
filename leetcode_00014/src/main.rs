use std::str;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let input = input.trim();
    let input = input.strip_prefix('[').unwrap();
    let input = input.strip_suffix(']').unwrap();
    let input: Vec<String> = input
        .split(',')
        .map(|s| s.trim().trim_matches('\"').to_string())
        .collect();
    let output = Solution::longest_common_prefix(input);
    println!("{:?}", output);
}

struct Solution {}
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 1 {
            return strs[0].clone();
        }
        let mut prefix = Self::common_prefix(&strs[0], &strs[1]);
        for i in 2..strs.len() {
            prefix = Self::common_prefix(prefix, &strs[i]);
        }
        return prefix.to_string();
    }

    fn common_prefix<'a>(str1: &'a str, str2: &str) -> &'a str {
        let mut end_idx: usize = usize::MAX;
        for (c1, c2) in str1.chars().enumerate().zip(str2.chars().enumerate()) {
            if c1 == c2 {
                end_idx = c1.0;
            } else {
                break;
            }
        }
        if end_idx == usize::MAX {
            return &str1[0..0];
        } else {
            return &str1[0..=end_idx];
        }
    }
}

/*
input:
["flower","flow","flight"]
output:
"fl"
*/
