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