use std::cmp::{max, min};

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let height = input
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let area = Solution::max_area(height);
    print!("{area}");
}

struct Solution {}
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp::{max, min};
        let (mut left, mut right) = (0, height.len() - 1);
        let mut area: i32 = 0;
        while left < right {
            let new_area = min(height[left], height[right]) * (right - left) as i32;
            area = max(area, new_area);
            if height[left] <= height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        area
    }
}
