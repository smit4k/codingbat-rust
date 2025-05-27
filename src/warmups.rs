pub fn sum_double(a: i32, b: i32) -> i32 {
    if a == b {
        return 2*(a+b);
    }
    return a+b;
}

pub fn makes_ten(a: i32, b:i32) -> bool {
    if a == 10 || b == 10 {
        return true;
    }
    if a+b == 10 {
        return true;
    }
    return false;
}

pub fn close10(a:i32, b:i32) -> i32 {
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

pub fn or35(n:u32) -> bool {
    if n % 3 == 0 || n % 5 == 0 {
        return true;
    }
    else {
        return false;
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

pub fn in3050(a: i32, b: i32) -> bool {
    ((a>=30 && a<=40) && (b>=30 && b<=40)) || ((a>=40 && a<=50) && b>=40 && b<=50)
}

pub fn max1020(a:i32, b:i32) -> i32 {
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
    if (temp1 < 0 && temp2 > 100) || (temp2 < 0 && temp1 > 100) {
        return true;
    }
    return false;
}

pub fn pos_neg(a:i32, b:i32, negative:bool) -> bool {
    if !negative {
        if (a < 0 && b > 0) || (a > 0 && b < 0) {
            return true;
        }
        else {
            return false;
        }
    }
    else {
        a < 0 && b < 0
    }
}

pub fn near_hundred(n: i32) -> bool {
    let dist_100 = (100-n).abs();
    let dist_200 = (200-n).abs();
    
    dist_100 <=10 || dist_200 <= 10
}
