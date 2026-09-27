struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);

        let mut ans = 0;
        let chars: Vec<char> = word.chars().collect();
        for l in 0..chars.len() {
            let mut seen: HashSet<char> = HashSet::with_capacity(5);

            for r in l..chars.len() {
                if !vowels.contains(&chars[r]) {
                    break;
                }

                seen.insert(chars[r]);

                if seen.len() == vowels.len() {
                    ans += 1;
                }
            }
        }

        return ans;
    }
}

fn main() {
    /*
     Example 1:
         Input: word = "aeiouu"
         Output: 2
         Explanation: The vowel substrings of word are as follows (underlined):
         - "aeiouu"
         - "aeiouu"
    */

    debug_assert_eq!(Solution::count_vowel_substrings("aeiouu".to_string()), 2);

    /*
     Example 2:
         Input: word = "unicornarihan"
         Output: 0
         Explanation: Not all 5 vowels are present, so there are no vowel substrings.
    */
    debug_assert_eq!(
        Solution::count_vowel_substrings("unicornarihan".to_string()),
        0
    );

    /*
        Example 3:
            Input: word = "cuaieuouac"
            Output: 7
            Explanation: The vowel substrings of word are as follows (underlined):
            - "cuaieuouac"
            - "cuaieuouac"
            - "cuaieuouac"
            - "cuaieuouac"
            - "cuaieuouac"
            - "cuaieuouac"
            - "cuaieuouac"
    */
    debug_assert_eq!(
        Solution::count_vowel_substrings("cuaieuouac".to_string()),
        7
    );
}
