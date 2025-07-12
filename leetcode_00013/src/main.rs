fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    input = input.trim().trim_matches('\"').to_string();

    let res = Solution::roman_to_int(input);
    println!("{res}");
}

struct Solution {}
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        use std::collections::HashMap;
        let char_map = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let mut res = 0;

        for (idx, ch) in s.chars().enumerate() {
            let &curr = char_map.get(&ch).unwrap();
            let &next = char_map
                .get(&s.chars().nth(idx + 1).unwrap_or('a'))
                .unwrap_or(&0);
            if curr < next {
                res -= curr;
            } else {
                res += curr;
            }
        }

        res
    }
}
