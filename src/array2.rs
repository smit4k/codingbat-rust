/// Return the number of even ints in the given array.
/// Note: the `%` "mod" operator computes the remainder, e.g. `5 % 2` is `1`.
///
/// # Test Cases
///
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
///
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

        sum += num;
    }

    sum
}

/// Given an array of ints, return `true` if the array contains no 1's and no 3's.
///
/// # Test Cases
///
/// ```
/// use codingbat_rust::array2::lucky13;
///
/// assert_eq!(lucky13(&[0, 2, 4]), true);
/// assert_eq!(lucky13(&[1, 2, 3]), false);
/// assert_eq!(lucky13(&[1, 2, 4]), false);
/// ```
pub fn lucky13(nums: &[i32]) -> bool {
    nums.iter().all(|&n| n != 1 && n != 3)
}

/// Given an array length 1 or more of ints, return the difference between the largest and smallest values in the array.
/// Note: the built-in Math.min(v1, v2) and Math.max(v1, v2) methods return the smaller or larger of two values.
///
/// # Test Cases
///
/// ```
/// use codingbat_rust::array2::big_diff;
///
/// assert_eq!(big_diff(&[10, 3, 5, 6]), 7);
/// assert_eq!(big_diff(&[7, 2, 10, 9]), 8);
/// assert_eq!(big_diff(&[2, 10, 7, 2]), 8);
/// ```
pub fn big_diff(nums: &[i32]) -> i32 {
    let max = nums.iter().max().copied().unwrap();
    let min = nums.iter().min().copied().unwrap();
    max - min
}

/// Given an array of ints, return `true` if the number of 1's is greater than the number of 4's
///
/// # Test Cases
///
/// ```
/// use codingbat_rust::array2::more_14;
///
/// assert_eq!(more_14(&[1, 4, 1]), true);
/// assert_eq!(more_14(&[1, 4, 1, 4]), false);
/// assert_eq!(more_14(&[1, 1]), true);  
/// ```
pub fn more_14(nums: &[i32]) -> bool {
    nums.iter().filter(|&&x| x == 1).count() > nums.iter().filter(|&&x| x == 4).count()
}

/// Given a non-empty array of ints, return a new array containing the elements from the original array that come before the first 4 in the original array.
/// The original array will contain at least one 4.
///
/// # Test Cases
///
/// ```
/// use codingbat_rust::array2::pre4;
///
/// assert_eq!(pre4(&[1, 2, 4, 1]), [1, 2]);
/// assert_eq!(pre4(&[3, 1, 4]), [3, 1]);
/// assert_eq!(pre4(&[1, 4, 4]), [1]);
/// ```
pub fn pre4(nums: &[i32]) -> Vec<i32> {
    if let Some(index) = nums.iter().position(|&x| x == 4) {
        nums[..index].to_vec()
    } else {
        vec![]
    }
}

/// Given an array of ints, return `true` if every 2 that appears in the array is next to another 2
///
/// # Test Cases
///
/// ```
/// use codingbat_rust::array2::two_two;
///
/// assert_eq!(two_two(&[4, 2, 2, 3]), true);
/// assert_eq!(two_two(&[2, 2, 4]), true);
/// assert_eq!(two_two(&[2, 2, 4, 2]), false);
/// ```
pub fn two_two(nums: &[i32]) -> bool {
    for i in 0..nums.len() {
        if nums[i] == 2 {
            let left = i > 0 && nums[i - 1] == 2;
            let right = i < nums.len() - 1 && nums[i + 1] == 2;

            if !(left || right) {
                return false;
            }
        }
    }

    true
}

/// Return an array that contains the exact same numbers as the given array, but rearranged so that all the zeros are grouped at the start of the array.
/// The order of the non-zero numbers does not matter. So `{1, 0, 0, 1}` becomes `{0 ,0, 1, 1}`. You may modify and return the given array or make a new array.
///
/// # Test Cases
///
/// ```
/// use codingbat_rust::array2::zero_front;
///
/// assert_eq!(zero_front(&[1, 0, 0, 1]), [0, 0, 1, 1]);
/// assert_eq!(zero_front(&[0, 1, 1, 0, 1]), [0 ,0, 1, 1, 1]);
/// assert_eq!(zero_front(&[1, 0]), [0, 1]);
/// ```
pub fn zero_front(nums: &[i32]) -> Vec<i32> {
    let zero_count = nums.iter().filter(|&&x| x == 0).count();
    let non_zeros: Vec<i32> = nums.iter().copied().filter(|&x| x != 0).collect();

    let mut result = Vec::with_capacity(nums.len());
    result.extend(std::iter::repeat(0).take(zero_count));
    result.extend(non_zeros);

    result
}

/// Return an array that contains the exact same numbers as the given array,
/// but rearranged so that all the even numbers come before all the odd numbers.
/// Other than that, the numbers can be in any order. You may modify and return the given array, or make a new array.
///
/// # Test Cases
/// ```
/// use codingbat_rust::array2::even_odd;
/// assert_eq!(even_odd(&[1, 0, 1, 0, 0, 1, 1]), [0, 0, 0, 1, 1, 1, 1]);
/// assert_eq!(even_odd(&[3, 3, 2]), [2, 3, 3]);
/// assert_eq!(even_odd(&[2, 2, 2]), [2, 2, 2]);
/// ```
pub fn even_odd(nums: &[i32]) -> Vec<i32> {
    let mut even_vec = Vec::new();
    let mut odd_vec = Vec::new();

    for num in nums {
        if num % 2 == 0 {
            even_vec.push(*num);
        }
    }

    for num in nums {
        if num % 2 != 0 {
            odd_vec.push(*num);
        }
    }

    even_vec.extend(odd_vec);
    even_vec
}

