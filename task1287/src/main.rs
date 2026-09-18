use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for &v in &arr {
            *map.entry(v).or_insert(0) += 1;
        }

        let threshold = (arr.len() as f32 * 0.25) as i32;

        for (&k, &v) in &map {
            if v > threshold {
                return k;
            }
        }

        -1
    }
}

fn main() {
    // Example 1: [1,2,2,6,6,6,6,7,10] -> Output: 6
    let arr1 = vec![1, 2, 2, 6, 6, 6, 6, 7, 10];
    println!(
        "Test 1: {:?} -> {}",
        arr1,
        Solution::find_special_integer(arr1.clone())
    );

    // Example 2: [1,1] -> Output: 1
    let arr2 = vec![1, 1];
    println!(
        "Test 2: {:?} -> {}",
        arr2,
        Solution::find_special_integer(arr2.clone())
    );
}
