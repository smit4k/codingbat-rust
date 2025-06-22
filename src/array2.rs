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