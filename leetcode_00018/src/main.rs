fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let nums = input
        .trim()
        .trim_matches('[')
        .trim_matches(']')
        .split(',')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();
    input.clear();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let target = input.trim().parse::<i32>().unwrap();
    let res = Solution::four_sum(nums, target);
    println!("{:?}", res);
}
struct Solution {}
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut res = Vec::<Vec<i32>>::new();
        if nums.len() < 4 {
            return res;
        }
        for idx1 in 0..nums.len() - 3 {
            if idx1 > 0 && nums[idx1] == nums[idx1 - 1] {
                continue;
            }
            for idx2 in idx1 + 1..nums.len() - 2 {
                if idx2 > idx1 + 1 && nums[idx2] == nums[idx2 - 1] {
                    continue;
                }
                let cur_target = target as i64 - nums[idx1] as i64 - nums[idx2] as i64;
                Self::two_sum(&nums, &mut res, idx1, idx2, cur_target);
            }
        }
        res
    }

    fn two_sum(
        nums: &Vec<i32>,
        res: &mut Vec<Vec<i32>>,
        idx1: usize,
        idx2: usize,
        cur_target: i64,
    ) {
        let mut idx3 = idx2 + 1;
        let mut idx4 = nums.len() - 1;
        while idx3 < idx4 {
            let sum = nums[idx3] as i64 + nums[idx4] as i64;
            if sum == cur_target {
                res.push(vec![nums[idx1], nums[idx2], nums[idx3], nums[idx4]]);
                idx3 += 1;
                idx4 -= 1;
                while idx3 < idx4 && nums[idx3] == nums[idx3 - 1] {
                    idx3 += 1;
                }
                while idx3 < idx4 && nums[idx4] == nums[idx4 + 1] {
                    idx4 -= 1;
                }
            } else if sum < cur_target {
                idx3 += 1;
                while idx3 < idx4 && nums[idx3] == nums[idx3 - 1] {
                    idx3 += 1;
                }
            } else {
                idx4 -= 1;
                while idx3 < idx4 && nums[idx4] == nums[idx4 + 1] {
                    idx4 -= 1;
                }
            }
        }
    }
}

/*
input:
[1,0,-1,0,-2,2]
0
output:
[[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]

input:
[2,2,2,2,2]
8
output:
[[2,2,2,2]]
*/
