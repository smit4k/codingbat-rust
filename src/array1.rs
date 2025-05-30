/// Given an array of ints, return `true` if 6 appears as either the first or last element in the array. 
/// The array will be length 1 or more.
/// 
/// # Test Cases
/// ```
/// use codingbat_rust::array1::first_last6;
/// 
/// assert_eq!(first_last6(&[1, 3, 6]), true);
/// assert_eq!(first_last6(&[6, 1, 2, 3]), true);
/// assert_eq!(first_last6(&[13, 6, 1, 2, 3]), false);
/// ```
pub fn first_last6(nums: &[i32]) -> bool {
    nums[0] == 6 || nums[nums.len()-1] == 6
}

/// Given 2 arrays of ints, a and b, return `true` if they have the same first element or they have the same last element. 
/// Both arrays will be length 1 or more.
/// 
/// # Test Cases
/// ```
/// use codingbat_rust::array1::common_end;
/// 
/// assert_eq!(common_end(&[1, 2, 3], &[7, 3]), true);
/// assert_eq!(common_end(&[1, 2, 3], &[7, 3, 2]), false);
/// assert_eq!(common_end(&[1, 2, 3], &[1, 3]), true);
/// 
/// ```
pub fn common_end(a: &[i32], b: &[i32]) -> bool {
    a[0] == b[0] || a[a.len()-1] == b[b.len()-1]
}

/// Return an int array length 3 containing the first 3 digits of pi, {3, 1, 4}
/// 
/// # Test Cases
/// ```
/// use codingbat_rust::array1::make_pi;
/// 
/// assert_eq!(make_pi(), [3, 1, 4])
/// ``` 
/// 
pub fn make_pi() -> [i32; 3] {
    [3, 1, 4]
}

/// Given an integer slice `nums` of length 0, 1, or 2,
/// return `true` if the array contains the number 2 twice or the number 3 twice.
///
/// # Examples
///
/// ```
/// use codingbat_rust::list1::double23;
///
/// assert_eq!(double23(&[2, 2]), true);
/// assert_eq!(double23(&[3, 3]), true);
/// assert_eq!(double23(&[2, 3]), false);
/// assert_eq!(double23(&[]), false);
/// assert_eq!(double23(&[2]), false);
/// ```
pub fn double23(nums: &[i32]) -> bool {
    if nums.len() == 2 {
        nums == [2, 2] || nums == [3, 3]
    }
    else {
        false
    }
}