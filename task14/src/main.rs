struct Solution {}

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }

        let mut ans: Vec<Vec<i32>> = vec![];

        nums.sort();

        let mut i = 0;
        while i < nums.len() - 3 {
            let mut j = i + 1;
            while j < nums.len() - 2 {
                let local_target = target as i64 - nums[i] as i64 - nums[j] as i64;

                let (mut l, mut r) = (j + 1, nums.len() - 1);
                while l < r {
                    if local_target > nums[l] as i64 + nums[r] as i64 {
                        l += 1;
                    } else if local_target < nums[l] as i64 + nums[r] as i64 {
                        r -= 1;
                    } else {
                        ans.push(vec![nums[i], nums[j], nums[l], nums[r]]);

                        let (l_orig, r_orig) = (l, r);
                        while l < r && nums[l] == nums[l_orig] {
                            l += 1;
                        }
                        while l < r && nums[r] == nums[r_orig] {
                            r -= 1;
                        }
                    }
                }
                while j < nums.len() - 1 && nums[j] == nums[j + 1] {
                    j += 1;
                }

                j += 1;
            }
            while i < nums.len() - 1 && nums[i] == nums[i + 1] {
                i += 1
            }

            i += 1;
        }

        return ans;
    }
}

fn main() {
    // Input: nums = [1,0,-1,0,-2,2], target = 0
    // Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
    let input = vec![1, 0, -1, 0, -2, 2];
    let target = 0;
    assert_eq!(
        Solution::four_sum(input, target),
        vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]],
    );

    // Input: nums = [2,2,2,2,2], target = 8
    // Output: [[2,2,2,2]]
    let input = vec![2, 2, 2, 2, 2];
    let target = 8;
    assert_eq!(Solution::four_sum(input, target), vec![vec![2, 2, 2, 2]]);

    // Input: nums = [1000000000,1000000000,1000000000,1000000000], target = -294967296
    // Output: []
    let input = vec![1000000000, 1000000000, 1000000000, 1000000000];
    let target = -294967296;
    assert_eq!(Solution::four_sum(input, target), Vec::<Vec<i32>>::new());

    println!("passed")
}
