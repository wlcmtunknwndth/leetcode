struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut words: Vec<String> = Vec::new();

        let chars: Vec<char> = s.chars().collect();
        let mut i = chars.len() as i32 - 1;
        loop {
            while i >= 0 && chars[i as usize] == ' ' {
                i -= 1;
            }

            if i < 0 {
                break;
            }

            let end = i;
            while i >= 0 && chars[i as usize] != ' ' {
                i -= 1;
            }

            words.push(chars[(i + 1) as usize..=end as usize].iter().collect());
        }

        return words.join(" ");
    }
}

fn main() {
    /*
        Example 1:
            Input: s = "the sky is blue"
            Output: "blue is sky the"
    */
    debug_assert_eq!(
        Solution::reverse_words("the sky is blue".to_string()),
        "blue is sky the".to_string()
    );

    /*
        Example 2:
            Input: s = "  hello world  "
            Output: "world hello"
            Explanation: Your reversed string should not contain leading or trailing spaces.
    */
    debug_assert_eq!(
        Solution::reverse_words("  hello world  ".to_string()),
        "world hello".to_string()
    );

    /*
        Example 3:
            Input: s = "a good   example"
            Output: "example good a"
            Explanation: You need to reduce multiple spaces between two words to a single space in the reversed string.
    */
    debug_assert_eq!(
        Solution::reverse_words("a good   example".to_string()),
        "example good a".to_string()
    );
}
