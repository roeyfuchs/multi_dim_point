#[cfg(test)]
mod tests {
    use crate::Point;
    #[test]
    fn constructor() {
        let p: Point<u32> = Point::new(4);
        assert_eq!(p.get_vector(), &[0, 0, 0, 0])
    }
    //indexing test
    #[test]
    fn value_by_dim() {
        let p: Point<u32> = Point::new_from_vec(&vec![1, 2, 3]);
        assert_eq!(p.get_value(1), &1);
        assert_eq!(p.get_value(2), &2);
        assert_eq!(p.get_value(3), &3);
    }
    #[test]
    #[should_panic]
    fn value_from_not_existing_dim() {
        let p: Point<u32> = Point::new_from_vec(&vec![1, 2, 3]);
        let _ = p.get_value(4);
    }
    #[test]
    #[should_panic]
    fn value_from_dim_zero() {
        let p: Point<u32> = Point::new_from_vec(&vec![1, 2, 3]);
        let _ = p.get_value(0);
    }

    //adding tests
    #[test]
    fn add() {
        let p1: Point<u32> = Point::new_from_vec(&vec![1, 2, 3, 4]);
        let p2: Point<u32> = Point::new_from_vec(&vec![1, 2, 3, 4]);
        assert_eq!((p1 + p2).get_vector(), &[2, 4, 6, 8])
    }
    #[test]
    fn add_zeros() {
        let p1: Point<u32> = Point::new_from_vec(&vec![0, 0, 0, 0]);
        let p2: Point<u32> = Point::new_from_vec(&vec![0, 0, 0, 0]);
        assert_eq!((p1 + p2).get_vector(), &[0, 0, 0, 0])
    }
    #[test]
    fn add_negative() {
        let p1: Point<i32> = Point::new_from_vec(&vec![-1, -2, -5, -6]);
        let p2: Point<i32> = Point::new_from_vec(&vec![-2, -5, -7, -9]);
        assert_eq!((p1 + p2).get_vector(), &[-3, -7, -12, -15])
    }
    #[test]
    fn add_negative_and_positive() {
        let p1: Point<i32> = Point::new_from_vec(&vec![-1, -2, -5, -9, 0]);
        let p2: Point<i32> = Point::new_from_vec(&vec![2, 5, 7, 6, -0]);
        assert_eq!((p1 + p2).get_vector(), &[1, 3, 2, -3, 0])
    }
    #[test]
    #[should_panic]
    fn add_not_identical_dim() {
        let p1: Point<u32> = Point::new_from_vec(&vec![0, 0, 0, 0]);
        let p2: Point<u32> = Point::new_from_vec(&vec![0, 0, 0]);
        let _ = p1 + p2;
    }
    #[test]
    fn add_ref() {
        let p1: Point<u32> = Point::new_from_vec(&vec![1, 2, 3, 4]);
        let p2: Point<u32> = Point::new_from_vec(&vec![1, 2, 3, 4]);
        assert_eq!((&p1 + &p2).get_vector(), &[2, 4, 6, 8])
    }
    #[test]
    fn add_zeros_ref() {
        let p1: Point<u32> = Point::new_from_vec(&vec![0, 0, 0, 0]);
        let p2: Point<u32> = Point::new_from_vec(&vec![0, 0, 0, 0]);
        assert_eq!((&p1 + &p2).get_vector(), &[0, 0, 0, 0])
    }
    #[test]
    fn add_negative_ref() {
        let p1: Point<i32> = Point::new_from_vec(&vec![-1, -2, -5, -6]);
        let p2: Point<i32> = Point::new_from_vec(&vec![-2, -5, -7, -9]);
        assert_eq!((&p1 + &p2).get_vector(), &[-3, -7, -12, -15])
    }
    #[test]
    fn add_negative_and_positive_ref() {
        let p1: Point<i32> = Point::new_from_vec(&vec![-1, -2, -5, -9, 0]);
        let p2: Point<i32> = Point::new_from_vec(&vec![2, 5, 7, 6, -0]);
        assert_eq!((&p1 + &p2).get_vector(), &[1, 3, 2, -3, 0])
    }
    #[test]
    #[should_panic]
    fn add_not_identical_dim_ref() {
        let p1: Point<u32> = Point::new_from_vec(&vec![0, 0, 0, 0]);
        let p2: Point<u32> = Point::new_from_vec(&vec![0, 0, 0]);
        let _ = &p1 + &p2;
    }
    //sub tests
    #[test]
    fn sub() {
        let p1: Point<u32> = Point::new_from_vec(&vec![5, 6, 7, 5]);
        let p2: Point<u32> = Point::new_from_vec(&vec![1, 4, 3, 4]);
        assert_eq!((p1 - p2).get_vector(), &[4, 2, 4, 1])
    }
    #[test]
    fn sub_zeros() {
        let p1: Point<u32> = Point::new_from_vec(&vec![0, 0, 0, 0]);
        let p2: Point<u32> = Point::new_from_vec(&vec![0, 0, 0, 0]);
        assert_eq!((p1 - p2).get_vector(), &[0, 0, 0, 0])
    }
    #[test]
    fn sub_negative() {
        let p1: Point<i32> = Point::new_from_vec(&vec![-1, -2, -5, -6]);
        let p2: Point<i32> = Point::new_from_vec(&vec![-2, -5, -7, -9]);
        assert_eq!((p1 - p2).get_vector(), &[1, 3, 2, 3])
    }
    #[test]
    fn sub_negative_and_positive() {
        let p1: Point<i32> = Point::new_from_vec(&vec![-1, -2, -5, -9, 0]);
        let p2: Point<i32> = Point::new_from_vec(&vec![2, 5, 7, 6, -0]);
        assert_eq!((p1 - p2).get_vector(), &[-3, -7, -12, -15, 0])
    }
    #[test]
    #[should_panic]
    fn sub_not_identical_dim() {
        let p1: Point<u32> = Point::new_from_vec(&vec![0, 0, 0, 0]);
        let p2: Point<u32> = Point::new_from_vec(&vec![0, 0, 0]);
        let _ = p1 - p2;
    }
    //mul tests
    #[test]
    fn mul() {
        let p1: Point<u32> = Point::new_from_vec(&vec![1, 2, 3, 4]);
        let s: u32 = 5;
        assert_eq!((p1 * s).get_vector(), &[5, 10, 15, 20])
    }
    #[test]
    fn mul_zero() {
        let p1: Point<u32> = Point::new_from_vec(&vec![1, 2, 3, 4]);
        let s: u32 = 0;
        assert_eq!((p1 * s).get_vector(), &[0, 0, 0, 0])
    }
    #[test]
    fn mul_negative_and_negative_sclar() {
        let p1: Point<i32> = Point::new_from_vec(&vec![-1, -2, -5, -6]);
        let s: i32 = -2;
        assert_eq!((p1 * s).get_vector(), &[2, 4, 10, 12])
    }
    #[test]
    fn mul_negative_and_positive_sclar() {
        let p1: Point<i32> = Point::new_from_vec(&vec![-1, -2, -5, -9, 0]);
        let s: i32 = 2;
        assert_eq!((p1 * s).get_vector(), &[-2, -4, -10, -18, 0])
    }
    #[test]
    fn mul_positive_and_negative_sclar() {
        let p1: Point<i32> = Point::new_from_vec(&vec![1, 2, 5, 9, 0]);
        let s: i32 = -2;
        assert_eq!((p1 * s).get_vector(), &[-2, -4, -10, -18, 0])
    }
    //div tests
    #[test]
    fn div() {
        let p1: Point<u32> = Point::new_from_vec(&vec![20, 30, 40, 100]);
        let s: u32 = 5;
        assert_eq!((p1 / s).get_vector(), &[4, 6, 8, 20])
    }
    #[test]
    #[should_panic]
    fn div_in_zero() {
        let p1: Point<u32> = Point::new_from_vec(&vec![1, 2, 3, 4]);
        let s: u32 = 0;
        let _ = p1 / s;
    }
    #[test]
    fn div_negative_and_negative_sclar() {
        let p1: Point<i32> = Point::new_from_vec(&vec![-2, -8, -64, -256]);
        let s: i32 = -2;
        assert_eq!((p1 / s).get_vector(), &[1, 4, 32, 128])
    }
    #[test]
    fn div_negative_and_positive_sclar() {
        let p1: Point<i32> = Point::new_from_vec(&vec![-2, -8, -64, -256, 0]);
        let s: i32 = 2;
        assert_eq!((p1 / s).get_vector(), &[-1, -4, -32, -128, 0])
    }
    #[test]
    fn div_positive_and_negative_sclar() {
        let p1: Point<i32> = Point::new_from_vec(&vec![2, 8, 64, 256, 0]);
        let s: i32 = -2;
        assert_eq!((p1 / s).get_vector(), &[-1, -4, -32, -128, 0])
    }
    //aplly func test
    #[test]
    fn apply_add_closure() {
        let p1: Point<i32> = Point::new_from_vec(&vec![2, 8, 64, 256, 0]);
        let p2: Point<i32> = Point::new_from_vec(&vec![12, 18, 164, 1256, 0]);
        let add_f: Box<dyn Fn(&i32, &i32) -> i32> = Box::new(|a, b| a + b);
        assert_eq!((p1.apply_func(&p2, &add_f)), &[14, 26, 228, 1512, 0])
    }
    fn apply_add_func() {
        fn add_f(num1: &i32, num2: &i32) -> i32 {
            num1 + num2
        }

        let p1: Point<i32> = Point::new_from_vec(&vec![2, 8, 64, 256, 0]);
        let p2: Point<i32> = Point::new_from_vec(&vec![12, 18, 164, 1256, 0]);

        assert_eq!((p1.apply_func(&p2, &add_f)), &[14, 26, 228, 1512, 0])
    }
    //test size function
    #[test]
    fn dim_test() {
        let p1: Point<i32> = Point::new_from_vec(&vec![2, 8, 64, 256, 0]);
        assert_eq!(p1.get_size(), 5);
    }
    //set value test
    #[test]
    fn value_change() {
        let mut p1: Point<i32> = Point::new_from_vec(&vec![2, 8]);
        assert_eq!(p1.get_value(1), &2);
        p1.set_value(1, &3);
        assert_eq!(p1.get_value(1), &3);
    }
    #[test]
    #[should_panic]
    fn value_change_wrong_dim() {
        let mut p1: Point<i32> = Point::new_from_vec(&vec![2, 8]);
        p1.set_value(3, &1);
    }
    // equal test
    #[test]
    fn equal() {
        let p1: Point<i32> = Point::new_from_vec(&vec![2, 8]);
        let p2: Point<i32> = Point::new_from_vec(&vec![2, 8]);
        assert_eq!(p1 == p2, true);
    }
    #[test]
    fn not_equal() {
        let p1: Point<i32> = Point::new_from_vec(&vec![2, 8]);
        let p2: Point<i32> = Point::new_from_vec(&vec![22, 8]);
        assert_eq!(p1 == p2, false);
    }
    #[test]
    fn not_equal_dim() {
        let p1: Point<i32> = Point::new_from_vec(&vec![2, 8]);
        let p2: Point<i32> = Point::new_from_vec(&vec![2, 8, 9]);
        assert_eq!(p1 == p2, false);
    }
    // epsilon-far test
    #[test]
    fn close() {
        let p1: Point<i32> = Point::new_from_vec(&vec![2, 8]);
        let p2: Point<i32> = Point::new_from_vec(&vec![4, 5]);
        assert_eq!(p1.close(&p2, 3), true);
    }
    #[test]
    fn close_same_point() {
        let p1: Point<i32> = Point::new_from_vec(&vec![2, 8]);
        assert_eq!(p1.close(&p1, 0), true);
    }
    #[test]
    fn not_close() {
        let p1: Point<i32> = Point::new_from_vec(&vec![2, 8]);
        let p2: Point<i32> = Point::new_from_vec(&vec![50, 42]);
        assert_eq!(p1.close(&p2, 3), false);
    }
    #[test]
    fn close_in_part_of_dims() {
        let p1: Point<i32> = Point::new_from_vec(&vec![2, 8]);
        let p2: Point<i32> = Point::new_from_vec(&vec![2, 42]);
        assert_eq!(p1.close(&p2, 3), false);
    }
    //clone test
    #[test]
    fn clone() {
        let p1: Point<i32> = Point::new_from_vec(&vec![2, 8]);
        let p2: Point<i32> = p1.clone();
        assert_eq!(p1 == p2, true);
    }
}
