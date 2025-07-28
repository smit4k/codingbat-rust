/// The number 6 is a truly great number. 
/// Given two int values, `a` and `b`, return `true` if either one is 6. 
/// Or if their sum or difference is 6.
///
/// # Test Cases
///
/// ```
/// use codingbat_rust::logic1::love_6;
///
/// assert_eq!(love_6(6, 4), true);
/// assert_eq!(love_6(4, 5), false);
/// assert_eq!(love_6(1, 5), true);
/// ```
pub fn love_6(a: i32, b: i32) -> bool {
    a == 6 || b == 6 || a + b == 6 || a - b == 6 || b - a == 6
}

/// Return `true` if the given non-negative number is 1 or 2 more than a multiple of 20.
///
/// # Test Cases
///
/// ```
/// use codingbat_rust::logic1::more_20;
///
/// assert_eq!(more_20(20), false);
/// assert_eq!(more_20(21), true);
/// assert_eq!(more_20(22), true);
/// ```
pub fn more_20(n: u32) -> bool {
    n % 20 == 1 || n % 20 == 2
}

/// Return `true` if the given non-negative number is within 2 of a multiple of 10.
///
/// # Test Cases
///
/// ```
/// use codingbat_rust::logic1::near_ten;
///
/// assert_eq!(near_ten(12), true);
/// assert_eq!(near_ten(17), false);
/// assert_eq!(near_ten(19), true);
/// ```
pub fn near_ten(num: u32) -> bool {
    num % 10 <= 2 || num % 10 >= 8
}

/// Given three integers `a`, `b`, and `c`, returns `true` if it is possible 
/// to add two of the integers to get the third.
///
/// # Test Cases
///
/// ```
/// use codingbat_rust::logic1::two_as_one;
///
/// assert_eq!(two_as_one(1, 2, 3), true);
/// assert_eq!(two_as_one(3, 1, 2), true);
/// assert_eq!(two_as_one(3, 2, 2), false);
/// ```
pub fn two_as_one(a: i32, b: i32, c: i32) -> bool {
    a + b == c || a + c == b || b + c == a
}

/// Given 2 ints, `a` and `b`, return their sum. However, "teen" values in the range 13..19 inclusive, are extra lucky. 
/// So if either value is a teen, just return 19. 
///
/// # Test Cases
///
/// ```
/// use codingbat_rust::logic1::teen_sum;
///
/// assert_eq!(teen_sum(3, 4), 7);
/// assert_eq!(teen_sum(10, 13), 19);
/// assert_eq!(teen_sum(13, 2), 19);
/// ```
pub fn teen_sum(a: i32, b: i32) -> i32 {
    if (13..=19).contains(&a) || (13..=19).contains(&b) {
        19
    }
    else {
        a+b
    }
}

/// Return `true` if the given non-negative number is a multiple of 3 or 5, but not both.
///
/// # Test Cases
///
/// ```
/// use codingbat_rust::logic1::old_35;
///
/// assert_eq!(old_35(3), true);
/// assert_eq!(old_35(10), true);
/// assert_eq!(old_35(15), false);
/// ```
pub fn old_35(n: u32) -> bool {
    (n % 3 == 0) ^ (n % 5 == 0)
}

/// Return `true` if the given non-negative number is 1 or 2 less than a multiple of 20. So for example 38 and 39 return `true`, but 40 returns false.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::logic1::less_20;
/// 
/// assert_eq!(less_20(18), true);
/// assert_eq!(less_20(19), true);
/// assert_eq!(less_20(20), false);
/// ```
pub fn less_20(n: i32) -> bool {
    let remainder = n % 20;
    remainder == 18 || remainder == 19
}

/// You have a green lottery ticket, with ints a, b, and c on it. 
/// If the numbers are all different from each other, the result is 0. 
/// If all of the numbers are the same, the result is 20.
/// If two of the numbers are the same, the result is 10.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::logic1::green_ticket;
/// 
/// assert_eq!(green_ticket(1, 2, 3), 0);
/// assert_eq!(green_ticket(2, 2, 2), 20);
/// assert_eq!(green_ticket(1, 1, 2), 10);
/// ```
pub fn green_ticket(a: i32, b: i32, c: i32) -> i32 {
    if a == b && b == c {
        20
    }
    else if a == b || a == c || b == c {
        10
    }
    else {
        0
    }
}

/// We are having a party with amounts of tea and candy. 
/// Return the int outcome of the party encoded as 0=bad, 1=good, or 2=great. 
/// A party is good (1) if both tea and candy are at least 5.
/// However, if either tea or candy is at least double the amount of the other one, the party is great (2).
/// However, in all cases, if either tea or candy is less than 5, the party is always bad (0).
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::logic1::tea_party;
/// 
/// assert_eq!(tea_party(6, 8), 1);
/// assert_eq!(tea_party(3, 8), 0);
/// assert_eq!(tea_party(20, 6), 2);
/// ```
pub fn tea_party(tea: i32, candy: i32) -> i32 {
    if tea < 5 || candy < 5 {
        0
    }
    else if tea >= candy * 2 || candy >= tea * 2 {
        2
    }
    else {
        1
    }
}

/// When squirrels get together for a party, they like to have cigars. 
/// A squirrel party is successful when the number of cigars is between 40 and 60, inclusive. 
/// Unless it is the weekend, in which case there is no upper bound on the number of cigars. 
/// Return `true` if the party with the given values is successful, or `false` otherwise.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::logic1::cigar_party;
/// 
/// assert_eq!(cigar_party(30, false), false);
/// assert_eq!(cigar_party(50, false), true);
/// assert_eq!(cigar_party(70, true), true);
/// ```
pub fn cigar_party(cigars: i32, is_weekend: bool) -> bool {
    if is_weekend {
        cigars >= 40
    }
    else {
        cigars >= 40 && cigars <= 60
    }
}

/// Given three non-negative integers `a`, `b`, and `c`,
/// return `true` if two or more of them have the same rightmost digit.
/// The `%` (modulo) operator is used to compute the remainder,
/// e.g., `17 % 10` is `7`.
///
/// # Test Cases
///
/// ```
/// use codingbat_rust::logic1::last_digit;
///
/// assert_eq!(last_digit(23, 19, 13), true);
/// assert_eq!(last_digit(23, 19, 12), false);
/// assert_eq!(last_digit(23, 19, 3), true);
/// ```
pub fn last_digit(a: i32, b: i32, c: i32) -> bool {
    let a_rmd = a % 10;
    let b_rmd = b % 10;
    let c_rmd = c % 10;

    a_rmd == b_rmd || a_rmd == c_rmd || b_rmd == c_rmd
}

/// You have a red lottery ticket showing ints `a`, `b`, and `c`, each of which is 0, 1, or 2. 
/// If they are all the value 2, the result is 10. Otherwise if they are all the same, the result is 5. 
/// Otherwise so long as both `b` and `c` are different from `a`, the result is 1. 
/// Otherwise the result is 0.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::logic1::red_ticket;
/// 
/// assert_eq!(red_ticket(2, 2, 2), 10);
/// assert_eq!(red_ticket(2, 2, 1), 0);
/// assert_eq!(red_ticket(0, 0, 0), 5);
/// ```
pub fn red_ticket(a: i32, b: i32, c: i32) -> i32 {
    if a == 2 && b == 2 && c == 2 {
        10
    }
    else if a == b && b == c {
        5
    }
    else if b != a && c != a  {
        1
    }
    else {
        0
    }
}

/// You have a blue lottery ticket, with ints a, b, and c on it. 
/// This makes three pairs, which we'll call ab, bc, and ac. 
/// Consider the sum of the numbers in each pair. 
/// If any pair sums to exactly 10, the result is 10. 
/// Otherwise if the ab sum is exactly 10 more than either bc or ac sums, the result is 5. 
/// Otherwise the result is 0.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::logic1::blue_ticket;
/// 
/// assert_eq!(blue_ticket(9, 1, 0), 10);
/// assert_eq!(blue_ticket(9, 2, 0), 0);
/// assert_eq!(blue_ticket(6, 1, 4), 10);
/// ```
pub fn blue_ticket(a: i32, b: i32, c: i32) -> i32 {
    let ab = a + b;
    let bc = b + c;
    let ac = a + c;

    if ab == 10 || bc == 10 || ac == 10 {
        10
    } else if ab == bc + 10 || ab == ac + 10 {
        5
    } else {
        0
    }
}

/// Given three ints, a b c, return `true` if b is greater than a, and c is greater than b. 
/// However, with the exception that if "bOk" is `true`, b does not need to be greater than a.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::logic1::in_order;
/// 
/// assert_eq!(in_order(1, 2, 4, false), true);
/// assert_eq!(in_order(1, 2, 1, false), false);
/// assert_eq!(in_order(1, 1, 2, true), true);
/// ```
pub fn in_order(a: i32, b: i32, c: i32, b_ok: bool) -> bool {
    (b_ok || b > a) && c > b
}

/// Given three ints, a b c, return `true` if one of them is 10 or more less than one of the others.
/// 
/// # Test Cases
/// 
/// ```
/// use codingbat_rust::logic1::less_by_10;
/// 
/// assert_eq!(less_by_10(1, 7, 11), true);
/// assert_eq!(less_by_10(1, 7, 10), false);
/// assert_eq!(less_by_10(11, 1, 7), true); 
/// ```
pub fn less_by_10(a: i32, b: i32, c: i32) -> bool {
    (a - b).abs() >= 10 || (a - c).abs() >= 10 || (b - c).abs() >= 10
}