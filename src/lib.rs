#![forbid(unsafe_code)]
#![no_std]

#![doc = include_str!("../README.md")]

use core::marker::PhantomData;
use core::num::FpCategory;
use core::cmp::Ordering;
use core::fmt;

/// `num-traits` re-export:
pub use num_traits::float::Float;

#[cfg(test)] mod test;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// A property checker for a float type.
pub trait FloatChecker<T: Float> {
    /// A custom error resulting from a violated property check.
    type Error;
    /// Checks if a value satisfies a property.
    fn check(value: T) -> Result<(), Self::Error>;
}

macro_rules! prop_ops {
    ($($f:ident : $t:ty),*$(,)?) => {$(
        pub fn $f(self) -> $t {
            self.0.$f()
        }
    )*}
}
macro_rules! noarg_ops {
    ($($f:ident),*$(,)?) => {$(
        pub fn $f() -> Result<Self, C::Error> {
            Self::new(T::$f())
        }
    )*}
}
macro_rules! unary_ops {
    ($($f:ident),*$(,)?) => {$(
        pub fn $f(self) -> Result<Self, C::Error> {
            Self::new(self.0.$f())
        }
    )*}
}
macro_rules! binary_ops {
    ($($f:ident : $other:ident),*$(,)?) => {$(
        pub fn $f(self, $other: Self) -> Result<Self, C::Error> {
            Self::new(self.0.$f($other.0))
        }
    )*}
}

/// A checked floating point type.
///
/// Every instance of [`CheckedFloat`] is guaranteed to satisfy the property given by the provided [`FloatChecker`].
/// In particular, this can be used to have a floating point type that forbids values like
/// NaN, infinity, negatives, etc. all by providing different checkers.
///
/// [`CheckedFloat`] supports all the typical operations of a normal float type.
/// However, all operations that yield another float type (e.g., `add`, `sub`, `sin`, etc.)
/// instead yield a custom [`Result`] type specified by the [`FloatChecker`].
///
/// All methods from the [`Float`] type are supported by this wrapper in checked context.
/// For documentation, see the original method definitions in [`Float`].
///
/// [`CheckedFloat`] also supports an implementation of [`Ord`], which allows for directly sorting [`CheckedFloat`] collections.
/// The convention for this implementation has ordering `-NaN < -Inf < ... < -0 = +0 < ... < +Inf < +NaN`.
///
/// [`CheckedFloat::get`] can be used to get the underlying float value.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CheckedFloat<T: Float, C: FloatChecker<T>>(T, PhantomData<C>);
impl<T: Float, C: FloatChecker<T>> CheckedFloat<T, C> {
    pub fn new(value: T) -> Result<Self, C::Error> {
        C::check(value).map(|_| Self(value, PhantomData))
    }
    pub fn get(self) -> T {
        self.0
    }
    prop_ops! {
        classify: FpCategory, integer_decode: (u64, i16, i8), is_finite: bool, is_infinite: bool,
        is_nan: bool, is_normal: bool, is_sign_negative: bool, is_sign_positive: bool,
        is_zero: bool, is_one: bool,
    }
    noarg_ops! {
        infinity, max_value, min_positive_value, min_value, nan, neg_infinity, neg_zero, zero,
        one, epsilon,
    }
    unary_ops! {
        abs, acos, acosh, asin, asinh, atan, atanh, cbrt, ceil, cos, cosh,
        exp, exp2, exp_m1, floor, fract, ln, ln_1p, log10, log2, neg, recip,
        round, signum, sin, sinh, sqrt, tan, tanh, trunc, to_degrees, to_radians,
    }
    binary_ops! {
        abs_sub: other, add: other, atan2: other, div: other, hypot: other, log: base,
        mul: other, powf: n, rem: other, sub: other, copysign: sign,
    }
    pub fn mul_add(self, a: Self, b: Self) -> Result<Self, C::Error> {
        Self::new(self.0.mul_add(a.0, b.0))
    }
    pub fn powi(self, n: i32) -> Result<Self, C::Error> {
        Self::new(self.0.powi(n))
    }
    pub fn sin_cos(self) -> (Result<Self, C::Error>, Result<Self, C::Error>) {
        let (sin, cos) = self.0.sin_cos();
        (Self::new(sin), Self::new(cos))
    }
}

impl<T: Float, C: FloatChecker<T>> Ord for CheckedFloat<T, C> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.0.is_nan(), other.0.is_nan()) {
            (true, true) => match (self.0.is_sign_positive(), other.0.is_sign_positive()) {
                (true, true) | (false, false) => Ordering::Equal,
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
            }
            (true, false) => if self.0.is_sign_positive() { Ordering::Greater } else { Ordering::Less },
            (false, true) => if other.0.is_sign_positive() { Ordering::Less } else { Ordering::Greater },
            (false, false) => self.0.partial_cmp(&other.0).unwrap(),
        }
    }
}
impl<T: Float, C: FloatChecker<T>> PartialOrd for CheckedFloat<T, C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Float, C: FloatChecker<T>> Eq for CheckedFloat<T, C> { }
impl<T: Float, C: FloatChecker<T>> PartialEq for CheckedFloat<T, C> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<T: Float, C: FloatChecker<T>> Copy for CheckedFloat<T, C> { }
impl<T: Float, C: FloatChecker<T>> Clone for CheckedFloat<T, C> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}

impl<T: Float + fmt::Debug, C: FloatChecker<T>> fmt::Debug for CheckedFloat<T, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl<T: Float + fmt::Display, C: FloatChecker<T>> fmt::Display for CheckedFloat<T, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
