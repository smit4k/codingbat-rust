pub mod warmups;
pub mod logic1;

#[cfg(test)]
mod tests {
    use crate::warmups::*;
    use crate::logic1::*;


    // Warmup tests

    #[test]
    fn test_diff21() {
        assert_eq!(diff21(19), 2);
        assert_eq!(diff21(10), 11);
        assert_eq!(diff21(21), 0);
    }

    #[test]
    fn test_sum_double() {
        assert_eq!(sum_double(2, 2), 8);
        assert_eq!(sum_double(0, 0), 0);
        assert_eq!(sum_double(-3, -3), -12);
    }

    #[test]
    fn test_makes_ten() {
        assert_eq!(makes_ten(9, 10), true);
        assert_eq!(makes_ten(9, 9), false);
        assert_eq!(makes_ten(1, 9), true);
        assert_eq!(makes_ten(10, 1), true);
        assert_eq!(makes_ten(5, 5), true);
    }

    #[test]
    fn test_close10() {
        assert_eq!(close10(8, 13), 8);
        assert_eq!(close10(13, 8), 8);
        assert_eq!(close10(13, 7), 0);
    }

    #[test]
    fn test_or35() {
        assert_eq!(or35(3), true);
        assert_eq!(or35(5), true) ;
        assert_eq!(or35(10), true);
        assert_eq!(or35(8), false);
    }

    #[test]
    fn test_has_teen() {
        assert_eq!(has_teen(13, 20, 10), true);
        assert_eq!(has_teen(20, 19, 10), true) ;
        assert_eq!(has_teen(20, 10, 13), true);
        assert_eq!(has_teen(8, 7, 24), false);
    }

    #[test]
    fn test_in3050() {
        assert_eq!(in3050(30, 31), true);
        assert_eq!(in3050(30, 41), false);
        assert_eq!(in3050(40, 50), true);
    }

    #[test]
    fn test_max1020() {
        assert_eq!(max1020(11, 19), 19);
        assert_eq!(max1020(19, 11), 19);
        assert_eq!(max1020(11, 9), 11);
    }

    #[test]
    fn test_icy_hot() {
        assert_eq!(icy_hot(120, -1), true);
        assert_eq!(icy_hot(-1, 120), true);
        assert_eq!(icy_hot(2, 120), false);
    }

    #[test]
    fn test_pos_neg() {
        assert_eq!(pos_neg(1, -1, false), true);
        assert_eq!(pos_neg(-1, 1, false), true);
        assert_eq!(pos_neg(-4, -4, true), true);
        assert_eq!(pos_neg(-1, -1, false), false);
        assert_eq!(pos_neg(1, 1, false), false);
        assert_eq!(pos_neg(4, 3, true), false);
        assert_eq!(pos_neg(6, -2, true), false);
    }

    #[test]
    fn test_lone_teen() {
        assert_eq!(lone_teen(13, 99), true);
        assert_eq!(lone_teen(21, 19), true);
        assert_eq!(lone_teen(13, 13), false);
    }

    #[test]
    fn test_near_hundred() {
        assert_eq!(near_hundred(93), true);
        assert_eq!(near_hundred(90), true);
        assert_eq!(near_hundred(89), false);
        assert_eq!(near_hundred(198), true);
        assert_eq!(near_hundred(178), false);
    }

    // Logic 1 tests
    #[test]
    fn test_love6() {
        assert_eq!(love6(6, 4), true);
        assert_eq!(love6(4, 5), false);
        assert_eq!(love6(1, 5), true);
        assert_eq!(love6(9, 3), true);
    }

    #[test]
    fn test_more20() {
        assert_eq!(more20(20), false);
        assert_eq!(more20(21), true);
        assert_eq!(more20(22), true);
        assert_eq!(more20(81), true);
        assert_eq!(more20(124), false);
    }

    #[test]
    fn test_two_as_one() {
        assert_eq!(two_as_one(1, 2, 3), true);
        assert_eq!(two_as_one(3, 1, 2), true);
        assert_eq!(two_as_one(3, 2, 2), false);
    }

    #[test]
    fn test_teen_sum() {
        assert_eq!(teen_sum(3, 4), 7);
        assert_eq!(teen_sum(10, 13), 19);
        assert_eq!(teen_sum(13, 2), 19);
    }

    #[test]
    fn test_old35() {
        assert_eq!(old35(3), true);
        assert_eq!(old35(10), true);
        assert_eq!(old35(15), false);
    }

    #[test]
    fn test_green_ticket() {
        assert_eq!(green_ticket(1, 2, 3), 0);
        assert_eq!(green_ticket(2, 2, 2), 20);
        assert_eq!(green_ticket(1, 1, 2), 10);
    }

    #[test]
    fn test_tea_party() {
        assert_eq!(tea_party(6, 8), 1);
        assert_eq!(tea_party(3, 8), 0);
        assert_eq!(tea_party(20, 6), 2);
    }
}
