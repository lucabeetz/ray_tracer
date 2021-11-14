mod vec;

mod tuple_tests {
    use float_cmp::approx_eq;

    use crate::vec::*;

    #[test]
    fn tuple_is_point() {
        let point = Tuple(4.3, -4.2, 3.1, 1.0);
        assert_eq!(point.0, 4.3);
        assert_eq!(point.1, -4.2);
        assert_eq!(point.2, 3.1);

        assert!(point.is_point());
        assert!(!point.is_vector());
    }

    #[test]
    fn tuple_is_vector() {
        let vector = Tuple(4.3, -4.2, 3.1, 0.0);
        assert_eq!(vector.0, 4.3);
        assert_eq!(vector.1, -4.2);
        assert_eq!(vector.2, 3.1);

        assert!(vector.is_vector());
        assert!(!vector.is_point());
    }

    #[test]
    fn point_creates_tuple_with_1() {
        let point = Tuple::point(4.3, -4.2, 3.1);
        assert_eq!(Tuple(4.3, -4.2, 3.1, 1.0), point);
    }

    #[test]
    fn vector_creates_tuple_with_0() {
        let vector = Tuple::vector(4.3, -4.2, 3.1);
        assert_eq!(vector, Tuple(4.3, -4.2, 3.1, 0.0));
    }

    #[test]
    fn add_two_tuples() {
        let a1 = Tuple(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(a1 + a2, Tuple(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn subtract_two_points() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = Tuple::point(3., 2., 1.);
        let v = Tuple::vector(5., 6., 7.);
        assert_eq!(p - v, Tuple::point(-2., -4., -6.));
    }

    #[test]
    fn subtract_two_vectors() {
        let v1 = Tuple::vector(3., 2., 1.);
        let v2 = Tuple::vector(5., 6., 7.);
        assert_eq!(v1 - v2, Tuple::vector(-2., -4., -6.));
    }

    #[test]
    fn negate_a_tuple() {
        let t = Tuple(1., -2., 3., -4.);
        assert_eq!(-t, Tuple(-1., 2., -3., 4.));
    }

    #[test]
    fn multiply_tuple_by_scalar() {
        let t = Tuple(1., -2., 3., -4.);
        assert_eq!(t * 2., Tuple(2., -4., 6., -8.));
    }

    #[test]
    fn multiply_tuple_by_fraction() {
        let t = Tuple(1., -2., 3., -4.);
        assert_eq!(t * 0.5, Tuple(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn divide_tuple_by_scalar() {
        let t = Tuple(1., -2., 3., -4.);
        assert_eq!(t / 2., Tuple(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn mag_of_vector() {
        let v = Tuple::vector(1., 0., 0.);
        assert_eq!(v.mag(), 1.0);
        let v = Tuple::vector(0., 1., 0.);
        assert_eq!(v.mag(), 1.0);
        let v = Tuple::vector(0., 0., 1.);
        assert_eq!(v.mag(), 1.0);

        let v = Tuple::vector(1., 2., 3.);
        assert_eq!(v.mag(), 14.0_f32.sqrt());

        let v = Tuple::vector(-1., -2., -3.);
        assert_eq!(v.mag(), 14.0_f32.sqrt());
    }

    #[test]
    fn normalize_vector() {
        let v = Tuple::vector(4., 0., 0.);
        assert_eq!(v.normalize(), Tuple::vector(1., 0., 0.));
        let v = Tuple::vector(1., 2., 3.);
        assert_eq!(
            v.normalize(),
            Tuple::vector(1. / 14_f32.sqrt(), 2. / 14_f32.sqrt(), 3. / 14_f32.sqrt())
        );

        let v = v.normalize();
        assert!(approx_eq!(f32, v.mag(), 1., ulps = 2));
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let v1 = Tuple::vector(1., 2., 3.);
        let v2 = Tuple::vector(2., 3., 4.);
        assert_eq!(v1.dot(&v2), 20.);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let v1 = Tuple::vector(1., 2., 3.);
        let v2 = Tuple::vector(2., 3., 4.);
        assert_eq!(v1.cross(&v2), Tuple::vector(-1., 2., -1.));
        assert_eq!(v2.cross(&v1), Tuple::vector(1., -2., 1.));
    }
}
