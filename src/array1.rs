/// Given an array of ints, return `true` if 6 appears as either the first or last element in the array. 
/// The array will be length 1 or more.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::array1::first_last_6;
/// 
/// assert_eq!(first_last_6(&[1, 3, 6]), true);
/// assert_eq!(first_last_6(&[6, 1, 2, 3]), true);
/// assert_eq!(first_last_6(&[13, 6, 1, 2, 3]), false);
/// ```
pub fn first_last_6(nums: &[i32]) -> bool {
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

/// Given an array of ints length 3, figure out which is larger, the first or last element in the array, and set all the other elements to be that value. 
/// Return the changed array
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::array1::max_end_3;
/// 
/// assert_eq!(max_end_3([1, 2, 3]), [3, 3, 3]);
/// assert_eq!(max_end_3([11, 5, 9]), [11, 11, 11]);
/// assert_eq!(max_end_3([2, 11, 3]), [3, 3, 3]);
/// ```
pub fn max_end_3(nums: [i32; 3]) -> [i32; 3] {
    let max = if nums[0] > nums[2] { nums[0] } else { nums[2] };
    [max; 3]
}

/// Given an int array, return a new array with double the length where its last element is the same as the original array, 
/// and all the other elements are 0. 
/// The original array will be length 1 or more. Note: by default, a new int array contains all 0's.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::array1::make_last;
/// 
/// assert_eq!(make_last(&[4, 5, 6]), [0, 0, 0, 0, 0, 6]);
/// assert_eq!(make_last(&[1, 2]), [0, 0, 0, 2]);
/// assert_eq!(make_last(&[3]), [0, 3]);
/// ```
pub fn make_last(nums: &[i32]) -> Vec<i32> {
    let mut res = vec![0; nums.len() * 2];
    let len = res.len();
    if let Some(&last) = nums.last() {
        res[len - 1] = last;    
    }

    res
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

/// Given an array of ints length 3, return an array with the elements "rotated left" so `{1, 2, 3}` yields `{2, 3, 1}`.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::array1::rotate_left_3;
/// 
/// assert_eq!(rotate_left_3([1, 2, 3]), [2, 3, 1]);
/// assert_eq!(rotate_left_3([5, 11, 9]), [11, 9, 5]);
/// assert_eq!(rotate_left_3([7, 0, 0]), [0, 0, 7]);
/// ```
pub fn rotate_left_3(nums: [i32; 3]) -> [i32; 3] {
    [nums[1], nums[2], nums[0]]
}

/// Given an integer slice `nums` of length 0, 1, or 2,
/// return `true` if the array contains the number 2 twice or the number 3 twice.
///
/// # Test Cases
///
/// ```
/// use codingbat_rust::array1::double_23;
///
/// assert_eq!(double_23(&[2, 2]), true);
/// assert_eq!(double_23(&[3, 3]), true);
/// assert_eq!(double_23(&[2, 3]), false);
/// assert_eq!(double_23(&[]), false);
/// assert_eq!(double_23(&[2]), false);
/// ```
pub fn double_23(nums: &[i32]) -> bool {
    if nums.len() == 2 {
        nums == [2, 2] || nums == [3, 3]
    }
    else {
        false
    }
}

/// Start with 2 int arrays, a and b, each length 2. 
/// Consider the sum of the values in each array. 
/// Return the array which has the largest sum. In event of a tie, return a.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::array1::bigger_two;
/// 
/// assert_eq!(bigger_two([1, 2], [3, 4]), [3, 4]);
/// assert_eq!(bigger_two([3, 4], [1, 2]), [3, 4]);
/// assert_eq!(bigger_two([1, 1], [1, 2]), [1, 2]);
/// ```
pub fn bigger_two(a: [i32; 2], b: [i32; 2]) -> [i32; 2] {
    match (a[0] + a[1]).cmp(&(b[0] + b[1])) {
        std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => a,
        std::cmp::Ordering::Less => b,
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
/// use codingbat_rust::array1::reverse_3;
/// 
/// assert_eq!(reverse_3([1, 2, 3]), [3, 2, 1]);
/// assert_eq!(reverse_3([5, 11, 9]), [9, 11, 5]);
/// assert_eq!(reverse_3([7, 0, 0]), [0, 0, 7]);
/// ```
pub fn reverse_3(nums: [i32; 3]) -> [i32; 3] {
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

/// We'll say that a 1 immediately followed by a 3 in an array is an "unlucky" 1. 
/// Return true if the given array contains an unlucky 1 in the first 2 or last 2 positions in the array.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::array1::unlucky_1;
/// 
/// assert_eq!(unlucky_1(&[1, 3, 4, 5]), true);
/// assert_eq!(unlucky_1(&[2, 1, 3, 4, 5]), true);
/// assert_eq!(unlucky_1(&[1, 1, 1]), false);
/// ```
pub fn unlucky_1(nums: &[i32]) -> bool {
    let len = nums.len();

    if len >= 2 {
        if nums[0] == 1 && nums[1] == 3 {
            return true;
        }
        if len >= 3 && nums[1] == 1 && nums[2] == 3 {
            return true;
        }
        if nums[len - 2] == 1 && nums[len - 1] == 3 {
            return true;
        }
    }

    false
}

/// Given an int array length 2, return `true` if it does not contain a 2 or 3.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::array1::no_23;
/// 
/// assert_eq!(no_23([4, 5]), true);
/// assert_eq!(no_23([4, 2]), false);
/// assert_eq!(no_23([3, 5]), false);
/// ```
pub fn no_23(nums: [i32; 2]) -> bool {
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
/// use codingbat_rust::array1::front_11;
/// 
/// assert_eq!(front_11(&[1, 2, 3], &[7, 8, 9]), [1, 7]);
/// assert_eq!(front_11(&[1], &[2]), [1, 2]);
/// assert_eq!(front_11(&[1, 7], &[]), [1]);    
/// ```
pub fn front_11(a: &[i32], b: &[i32]) -> Vec<i32> {
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

