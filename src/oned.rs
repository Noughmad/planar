use std::marker::{Copy, PhantomData};
use std::clone::Clone;
use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};
use std::cmp::Ordering;
use std::fmt;

macro_rules! impl_oned {
    ($(#[$attr:meta])* $s:ident) => {
        $(#[$attr])* pub struct $s<T, Unit> (T, PhantomData<Unit>);

        impl<T: Copy, Unit> Copy for $s<T, Unit> {}

        impl<T: Clone, Unit> Clone for $s<T, Unit> {
            fn clone(&self) -> Self {
                $s(self.get(), PhantomData{})
            }
        }

        /// Creates a new $s with a specified value and unit
        impl<T, Unit> $s<T, Unit> {
            pub fn new(x: T) -> Self {
                $s(x, PhantomData {})
            }
        }

        /// Returns the scalar value without a unit, consuming the $s
        impl<T, Unit> $s<T, Unit> {
            pub fn into_inner(self) -> T {
                self.0
            }
        }

        /// Returns the scalar value without a unit
        impl<T: Clone, Unit> $s<T, Unit> {
            pub fn get(&self) -> T {
                self.0.clone()
            }
        }

        impl<T: Mul<V, Output = W>, V, W, Unit> Mul<V> for $s<T, Unit> {
            type Output = $s<W, Unit>;
            fn mul(self, scale: V) -> Self::Output {
                Self::Output::new(self.into_inner() * scale)
            }
        }

        impl<T: Div<V, Output = W>, V, W, Unit> Div<V> for $s<T, Unit> {
            type Output = $s<W, Unit>;
            fn div(self, scale: V) -> Self::Output {
                Self::Output::new(self.into_inner() / scale)
            }
        }

        impl<T: MulAssign<V>, V, Unit> MulAssign<V> for $s<T, Unit> {
            fn mul_assign(&mut self, scale: V) {
                self.0 *= scale
            }
        }

        impl<T: DivAssign<V>, V, Unit> DivAssign<V> for $s<T, Unit> {
            fn div_assign(&mut self, scale: V) {
                self.0 /= scale
            }
        }

        impl<T: Neg<Output = W>, W, Unit> Neg for $s<T, Unit> {
            type Output = $s<W, Unit>;
            fn neg(self) -> Self::Output {
                Self::Output::new(-self.into_inner())
            }
        }

        impl<T: PartialEq, Unit> PartialEq for $s<T, Unit> {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl<T: PartialOrd, Unit> PartialOrd for $s<T, Unit> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl<T: Eq, Unit> Eq for $s<T, Unit> {}

        impl<T: Ord, Unit> Ord for $s<T, Unit> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl<T: fmt::Debug, Unit> fmt::Debug for $s<T, Unit> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "OneD {{ {:?} }}", self.0)
            }
        }
    }
}

impl_oned!(
    /// A generic length with a scalar value and dimension
    Length
);

impl_oned!(Width);
impl_oned!(Height);

impl_oned!(Position);
impl_oned!(PosX);
impl_oned!(PosY);

macro_rules! impl_oned_add {
    ($length: ident, $pos: ident) => {

        impl<T: Add<V, Output = W>, V, W, Unit> Add<$length<V, Unit>> for $length<T, Unit> {
            type Output = $length<W, Unit>;
            fn add(self, other: $length<V, Unit>) -> Self::Output {
                $length::new(self.into_inner() + other.into_inner())
            }
        }

        impl<T: Add<V, Output = W>, V, W, Unit> Add<$length<V, Unit>> for $pos<T, Unit> {
            type Output = $pos<W, Unit>;
            fn add(self, other: $length<V, Unit>) -> Self::Output {
                $pos::new(self.into_inner() + other.into_inner())
            }
        }

        impl<T: AddAssign<V>, V, Unit> AddAssign<$length<V, Unit>> for $length<T, Unit> {
            fn add_assign(&mut self, other: $length<V, Unit>) {
                self.0 += other.into_inner()
            }
        }

        impl<T: AddAssign<V>, V, Unit> AddAssign<$length<V, Unit>> for $pos<T, Unit> {
            fn add_assign(&mut self, other: $length<V, Unit>) {
                self.0 += other.into_inner()
            }
        }

        impl<T: Sub<V, Output = W>, V, W, Unit> Sub<$length<V, Unit>> for $length<T, Unit> {
            type Output = $length<W, Unit>;
            fn sub(self, other: $length<V, Unit>) -> Self::Output {
                $length::new(self.into_inner() - other.into_inner())
            }
        }

        impl<T: Sub<V, Output = W>, V, W, Unit> Sub<$length<V, Unit>> for $pos<T, Unit> {
            type Output = $pos<W, Unit>;
            fn sub(self, other: $length<V, Unit>) -> Self::Output {
                $pos::new(self.into_inner() - other.into_inner())
            }
        }

        impl<T: Sub<V, Output = W>, V, W, Unit> Sub<$pos<V, Unit>> for $pos<T, Unit> {
            type Output = $length<W, Unit>;
            fn sub(self, other: $pos<V, Unit>) -> Self::Output {
                $length::new(self.into_inner() - other.into_inner())
            }
        }

        impl<T: SubAssign<V>, V, Unit> SubAssign<$length<V, Unit>> for $length<T, Unit> {
            fn sub_assign(&mut self, other: $length<V, Unit>) {
                self.0 -= other.into_inner()
            }
        }

        impl<T: SubAssign<V>, V, Unit> SubAssign<$length<V, Unit>> for $pos<T, Unit> {
            fn sub_assign(&mut self, other: $length<V, Unit>) {
                self.0 -= other.into_inner()
            }
        }
    }
}

impl_oned_add!(Length, Position);
impl_oned_add!(Width, PosX);
impl_oned_add!(Height, PosY);

#[cfg(test)]
mod tests {
    pub use super::*;

    struct Pixel;

    #[test]
    fn construct_width() {
        let w: Width<f64, Pixel> = Width::new(40.0);
        let w2: Width<f64, Pixel> = Width::new(20.0);
        assert!(w2.get() < w.get());
        assert!(w2 < w);
        assert_eq!(w.into_inner(), 40.0);
        assert_eq!(w2.into_inner(), 20.0);
    }
}
