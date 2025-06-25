pub fn sleep_in(weekday: bool, vacation: bool) -> bool {
    !weekday || vacation
}

pub fn diff_21(n: i32) -> i32 {
    if n > 21 {
        2*((n-21).abs())
    }
    else {
        (n-21).abs()
    }
}

pub fn sum_double(a: i32, b: i32) -> i32 {
    if a == b {
        2*(a+b)
    }
    else {
        a+b
    }
}

pub fn makes_ten(a: i32, b:i32) -> bool {
    a == 10 || b == 10 || a + b == 10
}

pub fn close_10(a:i32, b:i32) -> i32 {
    let dist_a = (10-a).abs();
    let dist_b = (10-b).abs();

    if dist_a < dist_b {
        a
    }
    else if dist_b < dist_a {
        b
    }
    else {
        0
    }
}

pub fn or_35(n:u32) -> bool {
    if n % 3 == 0 || n % 5 == 0 {
        true
    }
    else {
        false
    }
}

pub fn has_teen(a: i32, b: i32, c: i32) -> bool {
    [a, b, c].iter().any(|&n| (13..=19).contains(&n))
}

pub fn lone_teen(a: i32, b: i32) -> bool {
    let a_is_teen = (13..=19).contains(&a);
    let b_is_teen = (13..=19).contains(&b);

    (a_is_teen && !b_is_teen) || (!a_is_teen && b_is_teen)
}

pub fn in_3050(a: i32, b: i32) -> bool {
    ((a>=30 && a<=40) && (b>=30 && b<=40)) || ((a>=40 && a<=50) && b>=40 && b<=50)
}

pub fn max_1020(a:i32, b:i32) -> i32 {
    if [a, b].iter().any(|&n| (10..=20).contains(&n)) {
        if (10..=20).contains(&a) && (10..=20).contains(&b) {
            if a > b {
                a
            } else {
                b
            }
        } else if (10..=20).contains(&a) {
            a
        } else {
            b
        }
    } else {
        0
    }
}

pub fn icy_hot(temp1:i32, temp2:i32) -> bool {
    (temp1 < 0 && temp2 > 100) || (temp2 < 0 && temp1 > 100)
}

pub fn pos_neg(a:i32, b:i32, negative:bool) -> bool {
    if !negative {
        if (a < 0 && b > 0) || (a > 0 && b < 0) {
            true
        }
        else {
            false
        }
    }
    else {
        a < 0 && b < 0
    }
}

pub fn front_3(str: &str) -> String {
    let front = &str[..str.len().min(3)];
    front.repeat(3)
}

pub fn front_22(str: &str) -> String {
    let front = &str[..str.len().min(2)];
    format!("{front}{str}{front}")
} 

pub fn int_max(a: i32, b: i32, c: i32) -> i32 {
    a.max(b).max(c)
}

pub fn near_hundred(n: i32) -> bool {
    let dist_100 = (100-n).abs();
    let dist_200 = (200-n).abs();
    
    dist_100 <=10 || dist_200 <= 10
}

pub fn in_1020(a: i32, b: i32) -> bool {
    (10..=20).contains(&a) || (10..=20).contains(&b)
}
