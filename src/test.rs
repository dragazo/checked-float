use super::*;

struct NoopChecker;
impl<T: Float> FloatChecker<T> for NoopChecker {
    type Error = ();
    fn check(x: T) -> Result<T, Self::Error> { Ok(x) }
}
type F64 = CheckedFloat<f64, NoopChecker>;

struct AbsChecker;
impl<T: Float> FloatChecker<T> for AbsChecker {
    type Error = T;
    fn check(value: T) -> Result<T, Self::Error> {
        if !value.is_nan() {
            Ok(value.abs())
        } else {
            Err(value)
        }
    }
}
type AbsF64 = CheckedFloat<f64, AbsChecker>;

#[test]
fn test_copy() {
    let u = F64::new(6.5).unwrap();
    let v = u;
    assert_eq!(u.get(), 6.5);
    assert_eq!(v.get(), 6.5);
    assert_eq!(u.get(), v.get());
}

#[test]
fn test_ord_eq() {
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

    assert_eq!(F64::new(f64::NAN).unwrap().max(F64::new(f64::INFINITY).unwrap()), F64::new(f64::NAN).unwrap());
    assert_eq!(F64::new(f64::INFINITY).unwrap().max(F64::new(f64::NAN).unwrap()), F64::new(f64::NAN).unwrap());

    assert_eq!(F64::new(-f64::NAN).unwrap().min(F64::new(f64::NEG_INFINITY).unwrap()), F64::new(-f64::NAN).unwrap());
    assert_eq!(F64::new(f64::NEG_INFINITY).unwrap().min(F64::new(-f64::NAN).unwrap()), F64::new(-f64::NAN).unwrap());

    assert!(F64::new(0.0).unwrap().is_sign_positive());
    assert!(F64::new(-0.0).unwrap().is_sign_negative());
    assert!(F64::new(0.0).unwrap().neg().unwrap().is_sign_negative());
    assert!(F64::new(-0.0).unwrap().neg().unwrap().is_sign_positive());

    assert_eq!(F64::new(0.0).unwrap(), F64::new(-0.0).unwrap());
}

#[test]
fn test_mapper() {
    assert_eq!(AbsF64::new(12.0).unwrap(), AbsF64::new(12.0).unwrap());
    assert_eq!(AbsF64::new(12.0).unwrap(), AbsF64::new(-12.0).unwrap());
    assert!(AbsF64::new(core::f64::NAN).is_err());
}
