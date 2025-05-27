pub mod warmups;

#[cfg(test)]
mod tests {
    use crate::warmups::*;

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
}
