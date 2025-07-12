fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let input = input.trim().trim_matches('\"');
    let res = Solution::letter_combinations(input.to_string());
    println!("{:?}", res);
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let nums_map = HashMap::from([
            (2 as u32, "abc"),
            (3, "def"),
            (4, "ghi"),
            (5, "jkl"),
            (6, "mno"),
            (7, "pqrs"),
            (8, "tuv"),
            (9, "wxyz"),
        ]);
        if digits.len() == 0 {
            return vec![];
        }
        let mut res = Vec::<String>::new();
        Self::track_back(&digits, &nums_map, &mut String::new(), &mut res);
        res
    }

    fn track_back(
        digits: &str,
        nums_map: &HashMap<u32, &str>,
        path: &mut String,
        res: &mut Vec<String>,
    ) {
        if digits.len() == 0 {
            res.push(path.clone());
            return;
        }
        let digit = digits.chars().next().unwrap().to_digit(10).unwrap();
        for ch in nums_map.get(&digit).unwrap().chars() {
            path.push(ch);
            Self::track_back(&digits[1..], nums_map, path, res);
            path.pop();
        }
    }
}



/*
input:
"23"
output:
["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
*/
