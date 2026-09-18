/*
    You are given an integer n.
    Define its mirror distance as: abs(n - reverse(n))
    where reverse(n) is the integer formed by reversing the digits of n.
    Return an integer denoting the mirror distance of n.
    abs(x) denotes the absolute value of x.
*/

struct Solution {}

impl Solution {
    fn reverse(n: i32) -> i32 {
        let revd: String = n.to_string().chars().rev().collect();

        revd.parse::<i32>().unwrap_or(0)
    }

    pub fn mirror_distance(n: i32) -> i32 {
        return (n - Solution::reverse(n)).abs();
    }
}

fn main() {
    assert!(27 == Solution::mirror_distance(25));
    println!("passed");

    assert!(9 == Solution::mirror_distance(10));
    println!("passed");

    assert!(0 == Solution::mirror_distance(7));
    println!("passed");
}
