use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let str = input.clone();
    input.clear();
    io::stdin().read_line(&mut input).expect("input num failed");
    let nrows: i32 = input.trim().parse().unwrap();
    let res = Solution::convert(str, nrows);
    println!("res:{res}");
}

struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let (mut map, mut up, mut row) = (vec![String::new(); num_rows as usize], -1, 0);
        for ch in s.chars() {
            map[row as usize].push(ch);
            if row == 0 || row + 1 == num_rows {
                up = -up;
            }
            row += up;
        }

        let mut result = String::new();
        for item in map.iter() {
            result.push_str(item);
        }
        result
    }
}
