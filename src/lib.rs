pub mod warmups;
pub mod logic1;
pub mod array1;
pub mod array2;

#[cfg(test)]
mod tests {
    use crate::warmups::*;
    use crate::logic1::*;
    use crate::array1::*;
    use crate::array2::*;


    // Warmup tests

    #[test]
    fn test_sleep_in() {
        assert_eq!(sleep_in(false, false), true);
        assert_eq!(sleep_in(true, false), false);
        assert_eq!(sleep_in(false, true), true);
    }

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

    #[test]
    fn test_cigar_party() {
        assert_eq!(cigar_party(30, false), false);
        assert_eq!(cigar_party(50, false), true);
        assert_eq!(cigar_party(70, true), true);
    }

    #[test]
    fn test_last_digit() {
        assert_eq!(last_digit(23, 19, 13), true);
        assert_eq!(last_digit(23, 19, 12), false);
        assert_eq!(last_digit(23, 19, 3), true);
    }

    #[test]
    fn test_red_ticket() {
        assert_eq!(red_ticket(2, 2, 2), 10);
        assert_eq!(red_ticket(2, 2, 1), 0);
        assert_eq!(red_ticket(0, 0, 0), 5);
    }
    
    #[test]
    fn test_blue_ticket() {
        assert_eq!(blue_ticket(9, 1, 0), 10);
        assert_eq!(blue_ticket(9, 2, 0), 0);
        assert_eq!(blue_ticket(6, 1, 4), 10);
    }

    #[test]
    fn test_in_order() {
        assert_eq!(in_order(1, 2, 4, false), true);
        assert_eq!(in_order(1, 2, 1, false), false);
        assert_eq!(in_order(1, 1, 2, true), true);
    }

    #[test]
    fn test_less_by_10() {
        assert_eq!(less_by_10(1, 7, 11), true);
        assert_eq!(less_by_10(1, 7, 10), false);
        assert_eq!(less_by_10(11, 1, 7), true);  
    }

    // Array 1 tests

    #[test]
    fn test_first_last6() {
        assert_eq!(first_last6(&[1, 3, 6]), true);
        assert_eq!(first_last6(&[6, 1, 2, 3]), true);
        assert_eq!(first_last6(&[13, 6, 1, 2, 3]), false);
    }

    #[test]
    fn test_common_end() {
        assert_eq!(common_end(&[1, 2, 3], &[7, 3]), true);
        assert_eq!(common_end(&[1, 2, 3], &[7, 3, 2]), false);
        assert_eq!(common_end(&[1, 2, 3], &[1, 3]), true);
    }

    #[test]
    fn test_max_triple() {
        assert_eq!(max_triple(&[1, 2, 3]), 3);
        assert_eq!(max_triple(&[1, 5, 3]), 5);
        assert_eq!(max_triple(&[5, 2, 3]), 5);
    }

    #[test]
    fn test_rotate_left3() {
        assert_eq!(rotate_left3([1, 2, 3]), [2, 3, 1]);
        assert_eq!(rotate_left3([5, 11, 9]), [11, 9, 5]);
        assert_eq!(rotate_left3([7, 0, 0]), [0, 0, 7]);
    }

    #[test]
    fn test_unlucky1() {
        assert_eq!(unlucky1(&[1, 3, 4, 5]), true);
        assert_eq!(unlucky1(&[2, 1, 3, 4, 5]), true);
        assert_eq!(unlucky1(&[1, 1, 1]), false);
    }

    #[test]
    fn test_double23() {
        assert_eq!(double23(&[2, 2]), true);
        assert_eq!(double23(&[3, 3]), true);
        assert_eq!(double23(&[2, 3]), false);
        assert_eq!(double23(&[]), false);
        assert_eq!(double23(&[2]), false);
    }

    #[test]
    fn test_same_first_last() {
        assert_eq!(same_first_last(&[1, 2, 3]), false);
        assert_eq!(same_first_last(&[1, 2, 3, 1]), true);
        assert_eq!(same_first_last(&[1, 2, 1]), true);
    }

    #[test]
    fn test_reverse3() {
        assert_eq!(reverse3([1, 2, 3]), [3, 2, 1]);
        assert_eq!(reverse3([5, 11, 9]), [9, 11, 5]);
        assert_eq!(reverse3([7, 0, 0]), [0, 0, 7]);
    }

    #[test]
    fn test_middle_way() {
        assert_eq!(middle_way([1, 2, 3], [4, 5, 6]), [2, 5]);
        assert_eq!(middle_way([7, 7, 7], [3, 8, 0]), [7, 8]);
        assert_eq!(middle_way([5, 2, 9], [1, 4, 5]), [2, 4]);
    }

    #[test]
    fn test_no23() {
        assert_eq!(no23([4, 5]), true);
        assert_eq!(no23([4, 2]), false);
        assert_eq!(no23([3, 5]), false);
    }

    #[test]
    fn test_mid_three() {
        assert_eq!(mid_three(&[1, 2, 3, 4, 5]), [2, 3, 4]);
        assert_eq!(mid_three(&[8, 6, 7, 5, 3, 0, 9]), [7, 5, 3]);
        assert_eq!(mid_three(&[1, 2, 3]), [1, 2, 3]);
    }

    #[test]
    fn test_front_piece() {
        assert_eq!(front_piece(&[1, 2, 3]), [1, 2]);
        assert_eq!(front_piece(&[1, 2]), [1, 2]);
        assert_eq!(front_piece(&[1]), [1]);   
    }

    #[test]
    fn test_front11() {
        assert_eq!(front11(&[1, 2, 3], &[7, 8, 9]), [1, 7]);
        assert_eq!(front11(&[1], &[2]), [1, 2]);
        assert_eq!(front11(&[1, 7], &[]), [1]);    
    }

    #[test]
    fn test_swap_ends() {
        assert_eq!(swap_ends(&[1, 2, 3, 4]), [4, 2, 3, 1]);
        assert_eq!(swap_ends(&[1, 2, 3]), [3, 2, 1]);
        assert_eq!(swap_ends(&[8, 6, 7, 9, 5]), [5, 6, 7, 9, 8]);
    }

    // Array 2 Tests

    #[test]
    fn test_count_evens() {
        assert_eq!(count_evens(&[2, 1, 2, 3, 4,]), 3);
        assert_eq!(count_evens(&[2, 2, 0]), 3);
        assert_eq!(count_evens(&[1, 3, 5]), 0);
    }

    #[test]
    fn test_sum13() {
        assert_eq!(sum13(&[1, 2, 2, 1]), 6);
        assert_eq!(sum13(&[1, 1]), 2);
        assert_eq!(sum13(&[1, 2, 2, 1, 13]), 6);
    }

    #[test]
    fn test_lucky13() {
        assert_eq!(lucky13(&[0, 2, 4]), true);
        assert_eq!(lucky13(&[1, 2, 3]), false);
        assert_eq!(lucky13(&[1, 2, 4]), false);
    }

    #[test]
    fn test_big_diff() {
        assert_eq!(big_diff(&[10, 3, 5, 6]), 7);
        assert_eq!(big_diff(&[7, 2, 10, 9]), 8);
        assert_eq!(big_diff(&[2, 10, 7, 2]), 8);
    }

    #[test]
    fn test_pre4() {
        assert_eq!(pre4(&[1, 2, 4, 1]), [1, 2]);
        assert_eq!(pre4(&[3, 1, 4]), [3, 1]);
        assert_eq!(pre4(&[1, 4, 4]), [1]);
    }

    #[test]
    fn test_two_two() {
        assert_eq!(two_two(&[4, 2, 2, 3]), true);
        assert_eq!(two_two(&[2, 2, 4]), true);
        assert_eq!(two_two(&[2, 2, 4, 2]), false);
    }

    #[test]
    fn test_zero_front() {
        assert_eq!(zero_front(&[1, 0, 0, 1]), [0, 0, 1, 1]);
        assert_eq!(zero_front(&[0, 1, 1, 0, 1]), [0 ,0, 1, 1, 1]);
        assert_eq!(zero_front(&[1, 0]), [0, 1]);
    }
}
