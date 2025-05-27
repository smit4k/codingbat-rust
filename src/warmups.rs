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
