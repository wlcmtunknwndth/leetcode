fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    assert!(has_trailing_zeros(nums) == true);
    println!("passed");

    let nums = vec![2, 4, 8, 16];
    assert!(has_trailing_zeros(nums) == true);
    println!("passed");

    let nums = vec![1, 3, 5, 7, 9];
    assert!(has_trailing_zeros(nums) == false);
    println!("passed");
}

pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
    let mut cnt = 0i32;
    for num in nums {
        if num % 2 == 0 {
            cnt += 1;
        }

        if cnt == 2 {
            return true;
        }
    }

    return false;
}
