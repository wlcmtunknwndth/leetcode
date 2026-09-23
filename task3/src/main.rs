struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashMap<char, usize> = HashMap::new();
        let (mut l, mut ans) = (-1i32, 0i32);

        for (r, ch) in s.chars().enumerate() {
            if let Some(v) = map.insert(ch, r) {
                l = l.max(v as i32);
            }

            ans = ans.max(r as i32 - l)
        }

        return ans;
    }
}

fn main() {
    let s: String = "abcabcbb".to_string();
    debug_assert_eq!(Solution::length_of_longest_substring(s), 3);

    let s: String = "bbbbb".to_string();
    debug_assert_eq!(Solution::length_of_longest_substring(s), 1);

    let s: String = "pwwkew".to_string();
    debug_assert_eq!(Solution::length_of_longest_substring(s), 3);

    println!("passed");
}
