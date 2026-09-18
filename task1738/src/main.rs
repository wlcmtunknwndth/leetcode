struct Solution;

impl Solution {
    pub fn kth_largest_value(mut matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let column_len = matrix.len(); // 2
        let row_len = matrix[0].len(); // 2
        // Fill row-wise
        // a b | a a*b
        // c d | c c*d
        for i in 0..column_len {
            if row_len == 1 {
                continue;
            }

            let mut xor_res = matrix[i][0];
            for j in 1..row_len {
                xor_res = xor_res ^ matrix[i][j];

                matrix[i][j] = xor_res;
            }
        }

        // Fill column-wise
        // a a*b | a   a*b
        // c c*d | a*c a*c*d
        //
        // i = 0, j = 0
        //        j = 1
        for i in 0..row_len {
            let mut xor_res = matrix[0][i];
            for j in 1..column_len {
                xor_res = xor_res ^ matrix[j][i];

                matrix[j][i] = xor_res;
            }
        }

        let mut values: Vec<i32> = matrix.into_iter().flatten().collect();
        values.sort();

        values.reverse();

        return values[k as usize - 1];
    }
}

pub fn main() {
    let matrix: Vec<Vec<i32>> = vec![vec![5, 2], vec![1, 6]];
    let mut k = 1i32;

    assert_eq!(Solution::kth_largest_value(matrix.clone(), k), 7);

    k = 2i32;
    assert_eq!(Solution::kth_largest_value(matrix.clone(), k), 5);

    k = 3i32;
    assert_eq!(Solution::kth_largest_value(matrix.clone(), k), 4);
}
