/// Given an array of ints, return `true` if 6 appears as either the first or last element in the array. 
/// The array will be length 1 or more.
/// 
/// # Test Cases
/// 
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
/// 
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

/// Given an array of ints of odd length, look at the first, last, and middle values in the array and return the largest. 
/// The array length will be a least 1.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::array1::max_triple;
/// 
/// assert_eq!(max_triple(&[1, 2, 3]), 3);
/// assert_eq!(max_triple(&[1, 5, 3]), 5);
/// assert_eq!(max_triple(&[5, 2, 3]), 5);
/// ```
pub fn max_triple(nums: &[i32]) -> i32 {
    let first = nums[0];
    let middle = nums[nums.len() / 2];
    let last = nums[nums.len()-1];

    first.max(middle).max(last)
}

/// Return an int array length 3 containing the first 3 digits of pi, {3, 1, 4}
/// 
/// # Test Cases
/// 
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
/// # Test Cases
///
/// ```
/// use codingbat_rust::array1::double23;
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

/// Given an array of ints, 
/// return true if the array is length 1 or more, 
/// and the first element and the last element are equal.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::array1::same_first_last;
/// 
/// assert_eq!(same_first_last(&[1, 2, 3]), false);
/// assert_eq!(same_first_last(&[1, 2, 3, 1]), true);
/// assert_eq!(same_first_last(&[1, 2, 1]), true);
/// ```
pub fn same_first_last(nums: &[i32]) -> bool {
    nums.len() >= 1 && nums[0] == nums[nums.len()-1]
}

/// Given an array of ints length 3,
/// return a new array with the elements in reverse order, 
/// so {1, 2, 3} becomes {3, 2, 1}.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::array1::reverse3;
/// 
/// assert_eq!(reverse3([1, 2, 3]), [3, 2, 1]);
/// assert_eq!(reverse3([5, 11, 9]), [9, 11, 5]);
/// assert_eq!(reverse3([7, 0, 0]), [0, 0, 7]);
/// ```
pub fn reverse3(nums: [i32; 3]) -> [i32; 3] {
    let mut nums2 = nums;
    nums2.reverse();
    nums2
}

/// Given 2 int arrays, a and b, each length 3, return a new array length 2 containing their middle elements.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::array1::middle_way;
/// 
/// assert_eq!(middle_way([1, 2, 3], [4, 5, 6]), [2, 5]);
/// assert_eq!(middle_way([7, 7, 7], [3, 8, 0]), [7, 8]);
/// assert_eq!(middle_way([5, 2, 9], [1, 4, 5]), [2, 4]);
/// ```
pub fn middle_way(a: [i32; 3], b: [i32; 3]) -> [i32; 2] {
    [a[1], b[1]]
}

/// Given an array of ints of odd length, return a new array length 3 containing the elements from the middle of the array. 
/// The array length will be at least 3.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::array1::mid_three;
/// 
/// assert_eq!(mid_three(&[1, 2, 3, 4, 5]), [2, 3, 4]);
/// assert_eq!(mid_three(&[8, 6, 7, 5, 3, 0, 9]), [7, 5, 3]);
/// assert_eq!(mid_three(&[1, 2, 3]), [1, 2, 3]);
/// ```
pub fn mid_three(nums: &[i32]) -> [i32; 3] {
    let mid = nums.len() / 2;

    [nums[mid-1], nums[mid], nums[mid+1]]
}

/// Given an int array length 2, return `true` if it does not contain a 2 or 3.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::array1::no23;
/// 
/// assert_eq!(no23([4, 5]), true);
/// assert_eq!(no23([4, 2]), false);
/// assert_eq!(no23([3, 5]), false);
/// ```
pub fn no23(nums: [i32; 2]) -> bool {
    nums.into_iter().all(|n| n != 2 && n!= 3)
}

/// Given an int array of any length, return a new array of its first 2 elements. 
/// If the array is smaller than length 2, 
/// use whatever elements are present.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::array1::front_piece;
/// 
/// assert_eq!(front_piece(&[1, 2, 3]), [1, 2]);
/// assert_eq!(front_piece(&[1, 2]), [1, 2]);
/// assert_eq!(front_piece(&[1]), [1]);   
/// ```
pub fn front_piece(nums: &[i32]) -> Vec<i32> {
    nums.iter().take(2).cloned().collect()
}

/// Given 2 int arrays, a and b, of any length, return a new array with the first element of each array. 
/// If either array is length 0, ignore that array.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::array1::front11;
/// 
/// assert_eq!(front11(&[1, 2, 3], &[7, 8, 9]), [1, 7]);
/// assert_eq!(front11(&[1], &[2]), [1, 2]);
/// assert_eq!(front11(&[1, 7], &[]), [1]);    
/// ```
pub fn front11(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();

    if let Some(&first_a) = a.first() {
        result.push(first_a);
    }

    if let Some(&first_b) = b.first() {
        result.push(first_b);
    }

    result
}

/// Given an array of ints, swap the first and last elements in the array. 
/// Return the modified array. The array length will be at least 1.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::array1::swap_ends;
/// 
/// assert_eq!(swap_ends(&[1, 2, 3, 4]), [4, 2, 3, 1]);
/// assert_eq!(swap_ends(&[1, 2, 3]), [3, 2, 1]);
/// assert_eq!(swap_ends(&[8, 6, 7, 9, 5]), [5, 6, 7, 9, 8]);
/// ```
pub fn swap_ends(nums: &[i32]) -> Vec<i32> {
    let mut result = nums.to_vec();

    if result.len() > 1 {
        let last_index = result.len() - 1;
        result.swap(0, last_index);
    }

    result
}

