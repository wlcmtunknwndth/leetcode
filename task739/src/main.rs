struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut queue: Vec<usize> = Vec::with_capacity(temperatures.len());
        let mut answers: Vec<i32> = vec![0; temperatures.len()];

        for r in 0..temperatures.len() {
            while !queue.is_empty() && temperatures[r] > temperatures[queue[queue.len() - 1]] {
                let l = queue.pop().unwrap();
                answers[l] = (r - l) as i32;
            }

            queue.push(r);
        }

        return answers;
    }
}

fn main() {
    // Example 1:
    // Input: temperatures = [73,74,75,71,69,72,76,73]
    // Output: [1,1,4,2,1,1,0,0]
    debug_assert_eq!(
        Solution::daily_temperatures([73, 74, 75, 71, 69, 72, 76, 73].to_vec()),
        [1, 1, 4, 2, 1, 1, 0, 0].to_vec()
    );

    // Example 2:
    // Input: temperatures = [30,40,50,60]
    // Output: [1,1,1,0]
    debug_assert_eq!(
        Solution::daily_temperatures([30, 40, 50, 60].to_vec()),
        [1, 1, 1, 0].to_vec()
    );

    // Example 3:
    // Input: temperatures = [30,60,90]
    // Output: [1,1,0]
    debug_assert_eq!(
        Solution::daily_temperatures([30, 60, 90].to_vec()),
        [1, 1, 0].to_vec()
    );
}
