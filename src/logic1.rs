pub fn love6(a: i32, b: i32) -> bool {
    if a == 6 || b == 6 || a + b == 6 || a - b == 6 || b - a == 6 {
        return true;
    }
    return false;
}

pub fn more20(n: u32) -> bool {
    if n % 20 == 1 || n % 20 == 2 {
        return true;
    }
    return false;
}

pub fn two_as_one(a: i32, b: i32, c: i32) -> bool {
    a + b == c || a + c == b || b + c == a
}

pub fn teen_sum(a: i32, b: i32) -> i32 {
    if a >= 13 && a <= 19 || b >= 13 && b <= 19 {
        return 19;
    }
    return a + b;
}

pub fn old35(n: u32) -> bool {
    (n % 3 == 0) ^ (n % 5 == 0)
}

pub fn green_ticket(a: i32, b: i32, c: i32) -> i32 {
    if a == b && b == c {
        return 20;
    }
    else if a == b || a == c || b == c {
        return 10;
    }
    else {
        return 0;
    }
}

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