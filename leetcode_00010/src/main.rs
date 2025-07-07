fn main() {
    let mut str = String::new();
    let mut pat = String::new();
    std::io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");
    std::io::stdin()
        .read_line(&mut pat)
        .expect("Failed to read line");
    let ret = Solution::is_match(str, pat);
    println!("{}", ret);
}

struct Solution {}
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        let p = p.chars().collect::<Vec<char>>();
        let mut is_match: Vec<Vec<bool>> = vec![vec![false; s.len() + 1]; p.len() + 1]; //is_match[p_idx][s_idx]表示p[0..p_idx]和s[0..s_idx]是否匹配
        is_match[0][0] = true;
        for (p_idx, &p_ch) in p.iter().enumerate() {
            if p_ch == '*' {
                if p_idx == 0 {
                    //*不能出现在第一个位置
                    is_match[p_idx + 1][0] = false;
                } else {
                    is_match[p_idx + 1][0] = is_match[p_idx - 1][0];
                }
            }
        }
        for (p_idx, &p_ch) in p.iter().enumerate() {
            for (s_idx, &s_ch) in s.iter().enumerate() {
                if p_ch == s_ch || p_ch == '.' {
                    is_match[p_idx + 1][s_idx + 1] = is_match[p_idx][s_idx];
                } else if p_ch == '*' {
                    //*和其前面的字符要组合考虑
                    if p[p_idx - 1] == s_ch || p[p_idx - 1] == '.' {
                        //可以匹配0个||多个前面的字符
                        is_match[p_idx + 1][s_idx + 1] =
                            is_match[p_idx - 1][s_idx + 1] || is_match[p_idx + 1][s_idx];
                    } else {
                        //*前面的字符不匹配s_ch，只能匹配0个前面的字符
                        is_match[p_idx + 1][s_idx + 1] = is_match[p_idx - 1][s_idx + 1];
                    }
                } else {
                    is_match[p_idx + 1][s_idx + 1] = false;
                }
            }
        }
        is_match[p.len()][s.len()]
    }
}
