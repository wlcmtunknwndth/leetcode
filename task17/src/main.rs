use std::collections::HashSet;

const Keypad: [&[char]; 10] = [
    &[],                   // 0
    &[],                   // 1
    &['a', 'b', 'c'],      // 2
    &['d', 'e', 'f'],      // 3
    &['g', 'h', 'i'],      // 4
    &['j', 'k', 'l'],      // 5
    &['m', 'n', 'o'],      // 6
    &['p', 'q', 'r', 's'], // 7
    &['t', 'u', 'v'],      // 8
    &['w', 'x', 'y', 'z'], // 9
];

fn num_to_letters(n: u32) -> Vec<char> {
    return Keypad[n as usize].to_vec();
}

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.len() == 0 {
        return Vec::new();
    }

    let digits_u32: Vec<u32> = digits
        .chars()
        .map(|c: char| c.to_digit(10u32).unwrap())
        .collect();

    let mut letters: Vec<Vec<char>> = Vec::with_capacity(digits_u32.len());
    for n in digits_u32 {
        letters.push(num_to_letters(n));
    }

    let mut perms: Vec<String> = letters[0].iter().map(|x: &char| x.to_string()).collect();
    for i in 1..letters.len() {
        let mut res: Vec<String> = Vec::with_capacity(perms.len() * letters[i].len());
        for ch in &letters[i] {
            let mut ch_perms: Vec<String> = perms
                .clone()
                .iter()
                .map(|x: &String| {
                    let mut res = x.clone();
                    res.push(*ch);
                    res
                })
                .collect();

            res.append(&mut ch_perms)
        }

        perms = res
    }

    return perms;
}

/*
    Example 1:

    Input: digits = "23"
    Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
    Example 2:

    Input: digits = "2"
    Output: ["a","b","c"]
*/

pub fn main() {
    assert_eq!(
        letter_combinations("23".to_string()),
        Vec::from(["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"])
    );

    assert_eq!(
        letter_combinations("2".to_string()),
        Vec::from(["a", "b", "c"])
    );
}
