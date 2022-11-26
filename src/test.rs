use super::*;

struct NoopChecker;
impl<T: Float> FloatChecker<T> for NoopChecker {
    type Error = ();
    fn check(_: T) -> Result<(), Self::Error> { Ok(()) }
}

#[test]
fn test_ord_eq() {
    type F64 = CheckedFloat<f64, NoopChecker>;

    assert_eq!(F64::new(f64::NAN).unwrap().eq(&F64::new(f64::NAN).unwrap()), true);
    assert_eq!(F64::new(f64::NAN).unwrap().eq(&F64::new(-f64::NAN).unwrap()), false);
    assert_eq!(F64::new(-f64::NAN).unwrap().eq(&F64::new(f64::NAN).unwrap()), false);
    assert_eq!(F64::new(-f64::NAN).unwrap().eq(&F64::new(-f64::NAN).unwrap()), true);

    assert_eq!(F64::new(f64::NAN).unwrap().cmp(&F64::new(f64::NAN).unwrap()), Ordering::Equal);
    assert_eq!(F64::new(f64::NAN).unwrap().cmp(&F64::new(-f64::NAN).unwrap()), Ordering::Greater);
    assert_eq!(F64::new(-f64::NAN).unwrap().cmp(&F64::new(f64::NAN).unwrap()), Ordering::Less);
    assert_eq!(F64::new(-f64::NAN).unwrap().cmp(&F64::new(-f64::NAN).unwrap()), Ordering::Equal);

    assert_eq!(F64::new(f64::NAN).unwrap().cmp(&F64::new(1.0).unwrap()), Ordering::Greater);
    assert_eq!(F64::new(f64::NAN).unwrap().cmp(&F64::new(-1.0).unwrap()), Ordering::Greater);
    assert_eq!(F64::new(-f64::NAN).unwrap().cmp(&F64::new(1.0).unwrap()), Ordering::Less);
    assert_eq!(F64::new(-f64::NAN).unwrap().cmp(&F64::new(-1.0).unwrap()), Ordering::Less);

    assert_eq!(F64::new(1.0).unwrap().cmp(&F64::new(f64::NAN).unwrap()), Ordering::Less);
    assert_eq!(F64::new(-1.0).unwrap().cmp(&F64::new(f64::NAN).unwrap()), Ordering::Less);
    assert_eq!(F64::new(1.0).unwrap().cmp(&F64::new(-f64::NAN).unwrap()), Ordering::Greater);
    assert_eq!(F64::new(-1.0).unwrap().cmp(&F64::new(-f64::NAN).unwrap()), Ordering::Greater);

    assert_eq!(F64::new(f64::NAN).unwrap().cmp(&F64::new(f64::INFINITY).unwrap()), Ordering::Greater);
    assert_eq!(F64::new(f64::NAN).unwrap().cmp(&F64::new(-f64::INFINITY).unwrap()), Ordering::Greater);
    assert_eq!(F64::new(-f64::NAN).unwrap().cmp(&F64::new(f64::INFINITY).unwrap()), Ordering::Less);
    assert_eq!(F64::new(-f64::NAN).unwrap().cmp(&F64::new(-f64::INFINITY).unwrap()), Ordering::Less);

    assert_eq!(F64::new(f64::INFINITY).unwrap().cmp(&F64::new(f64::NAN).unwrap()), Ordering::Less);
    assert_eq!(F64::new(-f64::INFINITY).unwrap().cmp(&F64::new(f64::NAN).unwrap()), Ordering::Less);
    assert_eq!(F64::new(f64::INFINITY).unwrap().cmp(&F64::new(-f64::NAN).unwrap()), Ordering::Greater);
    assert_eq!(F64::new(-f64::INFINITY).unwrap().cmp(&F64::new(-f64::NAN).unwrap()), Ordering::Greater);

    assert_eq!(F64::new(1.0).unwrap().cmp(&F64::new(2.0).unwrap()), Ordering::Less);
    assert_eq!(F64::new(-1.0).unwrap().cmp(&F64::new(2.0).unwrap()), Ordering::Less);
    assert_eq!(F64::new(1.0).unwrap().cmp(&F64::new(-2.0).unwrap()), Ordering::Greater);
    assert_eq!(F64::new(-1.0).unwrap().cmp(&F64::new(-2.0).unwrap()), Ordering::Greater);
}
