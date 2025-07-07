use std::{
    cmp::{max, min},
    i32::MAX,
    io,
};

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("input failed");
    let nums1 = get_vec(&input);
    input.clear();
    io::stdin().read_line(&mut input).expect("input failed");
    let nums2 = get_vec(&input);

    let ret = Solution3::find_median_sorted_arrays(nums1, nums2);
    println!("{ret}");
}

fn get_vec(input: &String) -> Vec<i32> {
    let ret = input
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    ret
}

pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let k = (nums1.len() + nums2.len()) / 2; //k>=0
        let mid1 = Self::find_kth_recur(&nums1[..], &nums2[..], k);

        let mid2 = if (nums1.len() + nums2.len()) % 2 == 0 {
            Self::find_kth_recur(&nums1[..], &nums2[..], k - 1)
        } else {
            mid1
        };
        let ret = (mid1 + mid2) as f64 / 2 as f64;
        ret
    }

    fn find_kth_recur(nums1: &[i32], nums2: &[i32], k: usize) -> i32 {
        if nums1.len() == 0 {
            return nums2[k];
        }
        if nums2.len() == 0 {
            return nums1[k];
        }
        if k == 0 {
            return min(nums1[0], nums2[0]);
        }
        let idx1 = min(nums1.len() - 1, (k / 2).saturating_sub(1));
        let idx2 = min(nums2.len() - 1, (k / 2).saturating_sub(1));
        if nums1[idx1] <= nums2[idx2] {
            return Self::find_kth_recur(&nums1[idx1 + 1..], nums2, k - idx1 - 1);
        } else {
            return Self::find_kth_recur(nums1, &nums2[idx2 + 1..], k - idx2 - 1);
        }
    }
}

struct Solution2 {}
impl Solution2 {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let k = (nums1.len() + nums2.len()) / 2; //k>=0
        let mid1 = Self::find_kth_circle(&nums1, &nums2, k);

        let mid2 = if (nums1.len() + nums2.len()) % 2 == 0 {
            Self::find_kth_circle(&nums1, &nums2, k - 1)
        } else {
            mid1
        };
        let ret = (mid1 + mid2) as f64 / 2 as f64;
        ret
    }

    fn find_kth_circle(nums1: &Vec<i32>, nums2: &Vec<i32>, k: usize) -> i32 {
        let (mut idx1, mut idx2) = (0, 0);
        let mut k = k;
        let ret: i32 = loop {
            if idx1 == nums1.len() {
                break nums2[idx2 + k];
            }
            if idx2 == nums2.len() {
                break nums1[idx1 + k];
            }
            if k == 0 {
                break min(nums1[idx1], nums2[idx2]);
            }
            let next_idx1 = min(nums1.len() - 1, idx1 + (k / 2).saturating_sub(1));
            let next_idx2 = min(nums2.len() - 1, idx2 + (k / 2).saturating_sub(1));
            if nums1[next_idx1] <= nums2[next_idx2] {
                k = k - (next_idx1 - idx1 + 1);
                idx1 = next_idx1 + 1;
            } else {
                k = k - (next_idx2 - idx2 + 1);
                idx2 = next_idx2 + 1;
            }
        };
        ret
    }
}

struct Solution3 {}
impl Solution3 {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (m, n) = (nums1.len(), nums2.len());
        if m > n {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }
        let target_len = (m + n + 1) / 2;
        let (mut median1, mut median2) = (0, 0);
        let (mut min_len, mut max_len) = (0, m + 1); //[min_len，max_len)

        while min_len < max_len {
            let len_a = (min_len + max_len) / 2;
            let len_b = target_len - len_a;
            let &a_left_max = nums1[0..len_a].last().unwrap_or(&i32::MIN);
            let &a_right_min = nums1[len_a..].first().unwrap_or(&i32::MAX);
            let &b_left_max = nums2[0..len_b].last().unwrap_or(&i32::MIN);
            let &b_right_min = nums2[len_b..].first().unwrap_or(&i32::MAX);

            if a_left_max <= b_right_min {
                median1 = max(a_left_max, b_left_max);
                median2 = min(a_right_min, b_right_min);
                min_len = len_a + 1;
            } else {
                max_len = len_a;
            }
        }
        let ret = if (m + n) % 2 == 1 {
            median1 as f64
        } else {
            (median1 as f64 + median2 as f64) / 2.0
        };
        ret
    }
}



/*
input:
[1,3]
[2]
output:
2.0


input:
[1,2]
[3,4]

output:
2.5

input:
[1,3,5,7,9]
[2,4,6,8,10]
output:
5.5
*/