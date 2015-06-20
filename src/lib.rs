//! Assertions for testing.

use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

/// A floating-point number.
pub trait Float: Add<Output=Self> + Div<Output=Self> + Mul<Output=Self> + Sub<Output=Self> +
                 Copy + PartialEq + PartialOrd
{
    fn abs(&self) -> Self;
    fn is_finite(&self) -> bool;
}

macro_rules! implement(
    ($kind:ty) => (
        impl Float for $kind {
            #[inline(always)] fn abs(&self) -> Self { <$kind>::abs(*self) }
            #[inline(always)] fn is_finite(&self) -> bool { <$kind>::is_finite(*self) }
        }
    );
);

implement!(f32);
implement!(f64);

/// Assert that the distance between the corresponding elements of two vectors
/// is smaller than a given value.
pub fn close<'l, I, T>(x: I, y: I, delta: T)
    where I: IntoIterator<Item=&'l T>, T: 'l + Debug + Float
{
    for (&x, &y) in x.into_iter().zip(y) {
        if x.is_finite() && y.is_finite() {
            assert!((x - y).abs() < delta, "{:?} !~ {:?}", x, y);
        } else {
            assert!(x == y, "{:?} !~ {:?}", x, y);
        }
    }
}

/// Assert that the distance between the absolute values of the corresponding
/// elements of two vectors is smaller than a given value.
pub fn close_abs<'l, I, T>(x: I, y: I, delta: T)
    where I: IntoIterator<Item=&'l T>, T: 'l + Debug + Float
{
    for (&x, &y) in x.into_iter().zip(y) {
        if x.is_finite() && y.is_finite() {
            assert!((x.abs() - y.abs()).abs() < delta, "|{:?}| !~ |{:?}|", x, y);
        } else {
            assert!(x == y, "|{:?}| !~ |{:?}|", x, y);
        }
    }
}

/// Assert that the result is a failure.
pub fn error<S, E>(result: Result<S, E>) {
    match result {
        Ok(..) => assert!(false, "got an OK, expected an error"),
        Err(..) => {},
    }
}

/// Assert that the result is a success.
pub fn success<S, E>(result: Result<S, E>) {
    match result {
        Ok(..) => {},
        Err(..) => assert!(false, "got an error, expected an OK"),
    }
}

#[cfg(test)]
mod test {
    struct Success;
    struct Failure;

    #[test]
    fn close() {
        ::close(&[1.0, 2.0, 3.0], &[1.0, 2.0 + 1e-10, 3.0 - 1e-10], 2e-10);
    }

    #[test]
    fn close_abs() {
        ::close_abs(&[1.0, 2.0, 3.0], &[-1.0, 2.0 + 1e-10, -3.0 - 1e-10], 2e-10);
    }

    #[test]
    fn error() {
        fn work() -> Result<Success, Failure> { Err(Failure) }
        ::error(work());
    }

    #[test]
    fn success() {
        fn work() -> Result<Success, Failure> { Ok(Success) }
        ::success(work());
    }
}
