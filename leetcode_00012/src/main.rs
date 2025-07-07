fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let num = input.trim().parse::<i32>().expect("failed to parse number");
    let roman = Solution::int_to_roman(num);
    print!("{roman}");
}

struct Solution {}
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let units = [('I', 'V'), ('X', 'L'), ('C', 'D'), ('M', ' ')];
        let mut res = String::new();
        let mut idx: usize = 0;
        let mut num = num;
        while num > 0 {
            let digit = num % 10;
            num /= 10;
            res = Self::handle_digit(digit, &units[idx..], res);
            idx += 1;
        }
        res
    }

    fn handle_digit(digit: i32, units: &[(char, char)], res: String) -> String {
        let str = match digit {
            1 => units[0].0.to_string(),
            2 => units[0].0.to_string().repeat(2),
            3 => units[0].0.to_string().repeat(3),
            4 => units[0].0.to_string() + &units[0].1.to_string(),
            5 => units[0].1.to_string(),
            6 => units[0].1.to_string() + &units[0].0.to_string(),
            7 => units[0].1.to_string() + &units[0].0.to_string().repeat(2),
            8 => units[0].1.to_string() + &units[0].0.to_string().repeat(3),
            9 => units[0].0.to_string() + &units[1].0.to_string(),
            _ => "".to_string(),
        };
        str + &res
    }
}
