/// The number 6 is a truly great number. 
/// Given two int values, `a` and `b`, return `true` if either one is 6. 
/// Or if their sum or difference is 6.
///
/// # Test Cases
///
/// ```
/// use codingbat_rust::logic1::love6;
///
/// assert_eq!(love6(6, 4), true);
/// assert_eq!(love6(4, 5), false);
/// assert_eq!(love6(1, 5), true);
/// ```
pub fn love6(a: i32, b: i32) -> bool {
    a == 6 || b == 6 || a + b == 6 || a - b == 6 || b - a == 6
}

/// Return `true` if the given non-negative number is 1 or 2 more than a multiple of 20.
///
/// # Test Cases
///
/// ```
/// use codingbat_rust::logic1::more20;
///
/// assert_eq!(more20(20), false);
/// assert_eq!(more20(21), true);
/// assert_eq!(more20(22), true);
/// ```
pub fn more20(n: u32) -> bool {
    n % 20 == 1 || n % 20 == 2
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
/// use codingbat_rust::logic1::old35;
///
/// assert_eq!(old35(3), true);
/// assert_eq!(old35(10), true);
/// assert_eq!(old35(15), false);
/// ```
pub fn old35(n: u32) -> bool {
    (n % 3 == 0) ^ (n % 5 == 0)
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
/// # Examples
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
/// Test Cases
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