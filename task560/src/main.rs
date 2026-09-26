struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut prefix_sum: HashMap<i32, u32> = HashMap::from([(0, 1)]);
        let (mut sum_r, mut ans) = (0, 0);

        // [1, 2, 1, 2, 1]
        // sum(l, r) = prefix[r] - prefix[l]
        // prefix[l] = prefix[r] - sum(l, r)
        // prefix[l] = , prefix[r] = sum, sum(l, r) = k
        for num in nums {
            sum_r += num;

            ans += *prefix_sum.entry(sum_r - k).or_default();

            prefix_sum.entry(sum_r).and_modify(|x| *x += 1).or_insert(1);
        }

        return ans as i32;
    }
}

fn main() {
    // Input: nums = [1,1,1], k = 2
    // Output: 2
    debug_assert_eq!(Solution::subarray_sum([1, 1, 1].to_vec(), 2), 2);

    // Input: nums = [1,2,3], k = 3
    // Output: 2
    debug_assert_eq!(Solution::subarray_sum([1, 2, 3].to_vec(), 3), 2);

    // Input: nums = [1,2,1,2,1], k = 3
    // Output: 4
    // [1, 2, 3, 4, 5] -> combs(nums) = n(n-1)/2 = 15
    // [1], [1, 2], [1, 2, 3], [1, 2, 3, 4], [1, 2, 3, 4, 5]
    // [2], [2, 3], [2, 3, 4], [2, 3, 4, 5]
    // [3], [3, 4], [3, 4, 5]
    // [4], [4, 5]
    // [5]
    debug_assert_eq!(Solution::subarray_sum([1, 2, 1, 2, 1].to_vec(), 3), 4);
}
