fn main() {
    let mut input = String::new();
    std::io::stdin()
       .read_line(&mut input)
       .expect("failed to read line");
    let n = input.trim().parse::<i32>().unwrap();
    let res = Solution::generate_parenthesis(n);
    println!("{:?}", res);
}

struct Solution {}
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = Vec::<String>::new();
        Self::back_trace(
            (n, n),
            &mut String::new(),
            &mut res,
            &mut Vec::<char>::new(),
        );
        res
    }

    fn back_trace(
        (left_rem, right_rem): (i32, i32),
        path: &mut String,
        res: &mut Vec<String>,
        stack: &mut Vec<char>,
    ) {
        if left_rem == 0 && right_rem == 0 {
            res.push(path.clone());
            return;
        }
        if left_rem > 0 {
            path.push('(');
            stack.push('(');
            Self::back_trace((left_rem - 1, right_rem), path, res, stack);
            stack.pop();
            path.pop();
        }
        if right_rem > 0 && stack.last().unwrap_or(&'\0') == &'(' {
            path.push(')');
            stack.pop();
            Self::back_trace((left_rem, right_rem - 1), path, res, stack);
            stack.push('(');
            path.pop();
        }
    }
}
