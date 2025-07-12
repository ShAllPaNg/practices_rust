fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let nums = input
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    input.clear();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let target = input.trim().parse::<i32>().unwrap();

    let res = Solution::three_sum_closest(nums, target);
    println!("{}", res);
}

struct Solution {}
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut dif = i32::MAX;
        let mut res = 0;
        for idx in 0..nums.len() - 2 {
            let mut left = idx + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let new_diff = target - (nums[idx] + nums[left] + nums[right]);
                if new_diff.abs() < dif.abs() {
                    dif = new_diff;
                    res = nums[idx] + nums[left] + nums[right];
                }
                if new_diff == 0 {
                    return res;
                } else if new_diff < 0 {
                    right -= 1;
                } else {
                    left += 1;
                }
            }
        }
        res
    }
}
/*
input:
[-1,2,1,-4]
1
output:
2
*/
