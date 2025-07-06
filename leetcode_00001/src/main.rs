
use std::collections::HashMap;
struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut ret: Vec<i32> = Vec::new();
        for (ridx, &elem) in nums.iter().enumerate() {
            let dif = target - elem;

            if let Some(&lidx) = map.get(&dif) {
                ret = vec![lidx as i32, ridx as i32];
                break;
            }
            map.insert(elem, ridx);
        }

        ret
    }
}

//leetcode提交部分到此为止

use std::io;

//通过此题，初步了解了输入的处理，本题学到了去除输入头尾空字符，去除头尾特定字符，按特定字符分割字符串，分割后字符串如何转化为所需类型数据
fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("input failed");

    let nums = input
        .trim()
        .trim_matches('[')
        .trim_matches(']')
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    input.clear();
    io::stdin().read_line(&mut input).expect("input failed");
    let target: i32 = input.trim().parse().unwrap();

    let ret = Solution::two_sum(nums, target);

    println!("{:?}", ret);
}