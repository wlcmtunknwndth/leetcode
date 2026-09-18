struct Solution;

pub fn get_sum(w: String) -> i32 {
    let mut ans = 0i32;

    for b in w.bytes() {
        ans = ans * 10 + (b - b'a') as i32;
    }

    return ans;
}

impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        return get_sum(first_word) + get_sum(second_word) == get_sum(target_word);
    }
}

fn main() {
    println!("Hello, world!");
}
