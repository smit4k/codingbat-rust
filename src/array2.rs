/// Return the number of even ints in the given array. 
/// Note: the `%` "mod" operator computes the remainder, e.g. `5 % 2` is `1`.
///
/// # Test Cases
/// ```
/// use codingbat_rust::array2::count_evens;
/// 
/// assert_eq!(count_evens(&[2, 1, 2, 3, 4,]), 3);
/// assert_eq!(count_evens(&[2, 2, 0]), 3);
/// assert_eq!(count_evens(&[1, 3, 5]), 0);
/// ```
pub fn count_evens(nums: &[i32]) -> u32 {
    nums.iter().filter(|&n| n % 2 == 0).count() as u32
}


/// Return the sum of the numbers in the array, returning 0 for an empty array. 
/// Except the number 13 is very unlucky, so it does not count and numbers that come immediately after a 13 also do not count.
/// 
/// # Test Cases
/// ```
/// use codingbat_rust::array2::sum13;
/// 
/// assert_eq!(sum13(&[1, 2, 2, 1]), 6);
/// assert_eq!(sum13(&[1, 1]), 2);
/// assert_eq!(sum13(&[1, 2, 2, 1, 13]), 6);
/// ```
pub fn sum13(nums: &[i32]) -> i32 {
    let mut sum = 0;
    let mut skip = false;

    for &num in nums {
        if skip {
            skip = false;
            continue;
        }

        if num == 13 {
            skip = true;
            continue;
        }

        sum+=num;
    }

    sum
}