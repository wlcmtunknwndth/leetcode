struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            (nums1, nums2) = (nums2, nums1);
        }

        let (m, n) = (nums1.len(), nums2.len());
        let total = m + n;
        let half = (total + 1) / 2;

        let mut left = 0;
        let mut right = m;

        while left <= right {
            let i = (left + right) / 2;
            let j = half - i;

            let left1 = if i == 0 { i32::MIN } else { nums1[i - 1] };
            let right1 = if i == m { i32::MAX } else { nums1[i] };

            let left2 = if j == 0 { i32::MIN } else { nums2[j - 1] };
            let right2 = if j == n { i32::MAX } else { nums2[j] };

            if left1 <= right2 && left2 <= right1 {
                if total % 2 == 1 {
                    return left1.max(left2) as f64;
                }

                return (left1.max(left2) as f64 + right1.min(right2) as f64) / 2.0;
            }

            if left1 > right2 {
                right = i - 1;
            } else {
                left = i + 1;
            }
        }

        0.0
    }
}

fn main() {
    let (nums1, nums2) = (Vec::from([1, 2]), Vec::from([3, 4]));
    assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.5);

    let (nums1, nums2) = (Vec::from([1, 3]), Vec::from([2]));
    assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.0)
}
