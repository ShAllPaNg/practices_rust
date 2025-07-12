fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let input = input.trim().trim_matches('\"').to_string();
    let res = Solution::is_valid(input);
    println!("{}", res);
}

struct Solution {}
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::<char>::new();
        for ch in s.chars() {
            match ch {
                '(' | '[' | '{' => {
                    stack.push(ch);
                }
                ')' => {
                    if stack.pop().unwrap_or('\0') != '(' {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop().unwrap_or('\0') != '[' {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop().unwrap_or('\0') != '{' {
                        return false;
                    }
                }
                _ => {}
            }
        }
        stack.len() == 0
    }
}

/*
input:
"()"
output:
true

input:
"()[]{}"
output:
true

input:
"(]"
output:
false

*/
