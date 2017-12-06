use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};
use std::fmt;

use oned::*;

pub struct Size<T, Unit> {
    pub width: Width<T, Unit>,
    pub height: Height<T, Unit>,
}

pub struct Point<T, Unit> {
    pub x: PosX<T, Unit>,
    pub y: PosY<T, Unit>,
}

macro_rules! impl_twod {
    ($s:ident, $x:ident, $y:ident) => {
        impl<T: Copy, Unit> Copy for $s<T, Unit> {}

        impl<T: Clone, Unit> Clone for $s<T, Unit> {
            fn clone(&self) -> Self {
                $s {
                    $x: self.$x.clone(),
                    $y: self.$y.clone(),
                }
            }
        }

        impl<T: Mul<V, Output = W>, V: Clone, W, Unit> Mul<V> for $s<T, Unit> {
            type Output = $s<W, Unit>;
            fn mul(self, scale: V) -> Self::Output {
                Self::Output {
                    $x: self.$x * scale.clone(),
                    $y: self.$y * scale,
                }
            }
        }

        impl<T: Div<V, Output = W>, V: Clone, W, Unit> Div<V> for $s<T, Unit> {
            type Output = $s<W, Unit>;
            fn div(self, scale: V) -> Self::Output {
                Self::Output {
                    $x: self.$x / scale.clone(),
                    $y: self.$y / scale,
                }
            }
        }

        impl<T: MulAssign<V>, V: Clone, Unit> MulAssign<V> for $s<T, Unit> {
            fn mul_assign(&mut self, scale: V) {
                self.$x *= scale.clone();
                self.$y *= scale;
            }
        }

        impl<T: DivAssign<V>, V: Clone, Unit> DivAssign<V> for $s<T, Unit> {
            fn div_assign(&mut self, scale: V) {
                self.$x /= scale.clone();
                self.$y /= scale;
            }
        }

        impl<T: Neg<Output = W>, W, Unit> Neg for $s<T, Unit> {
            type Output = $s<W, Unit>;
            fn neg(self) -> Self::Output {
                Self::Output {
                    $x: - self.$x,
                    $y: - self.$y,
                }
            }
        }

        impl<T: PartialEq, Unit> PartialEq for $s<T, Unit> {
            fn eq(&self, other: &Self) -> bool {
                self.$x == other.$x && self.$y == other.$y
            }
        }

        impl<T: Eq, Unit> Eq for $s<T, Unit> {}

        impl<T: fmt::Debug, Unit> fmt::Debug for $s<T, Unit> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "TwoD {{ x = {:?}, y = {:?} }}", self.$x, self.$y)
            }
        }
    }
}

macro_rules! impl_twod_add_width_height {
    ($s:ident, $x:ident, $y:ident) => {
        impl<T: Add<V, Output=T>, V, Unit> Add<Width<V, Unit>> for $s<T, Unit> {
            type Output = Self;
            fn add(self, other: Width<V, Unit>) -> Self::Output {
                $s {
                    $x: self.$x + other,
                    $y: self.$y,
                }
            }
        }

        impl<T: AddAssign<V>, V, Unit> AddAssign<Width<V, Unit>> for $s<T, Unit> {
            fn add_assign(&mut self, other: Width<V, Unit>) {
                self.$x += other;
            }
        }

        impl<T: Add<V, Output=T>, V, Unit> Add<Height<V, Unit>> for $s<T, Unit> {
            type Output = Self;
            fn add(self, other: Height<V, Unit>) -> Self::Output {
                $s {
                    $x: self.$x,
                    $y: self.$y + other,
                }
            }
        }

        impl<T: AddAssign<V>, V, Unit> AddAssign<Height<V, Unit>> for $s<T, Unit> {
            fn add_assign(&mut self, other: Height<V, Unit>) {
                self.$y += other;
            }
        }

        impl<T: Sub<V, Output=T>, V, Unit> Sub<Width<V, Unit>> for $s<T, Unit> {
            type Output = Self;
            fn sub(self, other: Width<V, Unit>) -> Self::Output {
                $s {
                    $x: self.$x - other,
                    $y: self.$y,
                }
            }
        }

        impl<T: SubAssign<V>, V, Unit> SubAssign<Width<V, Unit>> for $s<T, Unit> {
            fn sub_assign(&mut self, other: Width<V, Unit>) {
                self.$x -= other;
            }
        }

        impl<T: Sub<V, Output=T>, V, Unit> Sub<Height<V, Unit>> for $s<T, Unit> {
            type Output = Self;
            fn sub(self, other: Height<V, Unit>) -> Self::Output {
                $s {
                    $x: self.$x,
                    $y: self.$y - other,
                }
            }
        }

        impl<T: SubAssign<V>, V, Unit> SubAssign<Height<V, Unit>> for $s<T, Unit> {
            fn sub_assign(&mut self, other: Height<V, Unit>) {
                self.$y -= other;
            }
        }
    }
}

impl_twod!(Size, width, height);
impl_twod!(Point, x, y);

impl_twod_add_width_height!(Size, width, height);
impl_twod_add_width_height!(Point, x, y);

macro_rules! impl_twod_add {
    ($length: ident, $pos: ident) => {

        impl<T: Add<V, Output = W>, V, W, Unit> Add<$length<V, Unit>> for $length<T, Unit> {
            type Output = $length<W, Unit>;
            fn add(self, other: $length<V, Unit>) -> Self::Output {
                $length {
                    width: self.width + other.width,
                    height: self.height + other.height,
                }
            }
        }

        impl<T: Add<V, Output = W>, V, W, Unit> Add<$length<V, Unit>> for $pos<T, Unit> {
            type Output = $pos<W, Unit>;
            fn add(self, other: $length<V, Unit>) -> Self::Output {
                $pos {
                    x: self.x + other.width,
                    y: self.y + other.height,
                }
            }
        }

        impl<T: AddAssign<V>, V, Unit> AddAssign<$length<V, Unit>> for $length<T, Unit> {
            fn add_assign(&mut self, other: $length<V, Unit>) {
                self.width += other.width;
                self.height += other.height;
            }
        }

        impl<T: AddAssign<V>, V, Unit> AddAssign<$length<V, Unit>> for $pos<T, Unit> {
            fn add_assign(&mut self, other: $length<V, Unit>) {
                self.x += other.width;
                self.y += other.height;
            }
        }

        impl<T: Sub<V, Output = W>, V, W, Unit> Sub<$length<V, Unit>> for $length<T, Unit> {
            type Output = $length<W, Unit>;
            fn sub(self, other: $length<V, Unit>) -> Self::Output {
                $length {
                    width: self.width - other.width,
                    height: self.height - other.height,
                }
            }
        }

        impl<T: Sub<V, Output = W>, V, W, Unit> Sub<$length<V, Unit>> for $pos<T, Unit> {
            type Output = $pos<W, Unit>;
            fn sub(self, other: $length<V, Unit>) -> Self::Output {
                $pos {
                    x: self.x - other.width,
                    y: self.y - other.height,
                }
            }
        }

        impl<T: Sub<V, Output = W>, V, W, Unit> Sub<$pos<V, Unit>> for $pos<T, Unit> {
            type Output = $length<W, Unit>;
            fn sub(self, other: $pos<V, Unit>) -> Self::Output {
                $length {
                    width: self.x - other.x,
                    height: self.y - other.y,
                }
            }
        }

        impl<T: SubAssign<V>, V, Unit> SubAssign<$length<V, Unit>> for $length<T, Unit> {
            fn sub_assign(&mut self, other: $length<V, Unit>) {
                self.width -= other.width;
                self.height -= other.height;
            }
        }

        impl<T: SubAssign<V>, V, Unit> SubAssign<$length<V, Unit>> for $pos<T, Unit> {
            fn sub_assign(&mut self, other: $length<V, Unit>) {
                self.x -= other.width;
                self.y -= other.height;
            }
        }
    }
}

impl_twod_add!(Size, Point);

pub struct Rect<T, Unit> {
    pub origin: Point<T, Unit>,
    pub size: Size<T, Unit>,
}

impl<T, Unit> Rect<T, Unit> {
    pub fn new(origin: Point<T, Unit>, size: Size<T, Unit>) -> Self {
        Self { origin, size }
    }

    pub fn from_points<V>(origin: Point<T, Unit>, opposite: Point<V, Unit>) -> Self
    where
        T: Clone,
        V: Sub<T, Output = T>,
    {
        let size = opposite - origin.clone();
        Self { size, origin }
    }

    pub fn corner<W>(&self) -> Point<W, Unit>
    where
        T: Clone + Add<T, Output = W>,
    {
        self.origin.clone() + self.size.clone()
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    struct Pixel;

    #[test]
    fn construct_size() {
        let w: Width<f64, Pixel> = Width::new(40.0);
        let h: Height<f64, Pixel> = Height::new(20.0);
        let size = Size {
            width: w,
            height: h,
        };

        assert_eq!(
            size,
            Size {
                width: Width::<f64, Pixel>::new(40.0),
                height: Height::<f64, Pixel>::new(20.0),
            }
        );
    }

    #[test]
    fn size_is_clone() {
        let size = Size {
            width: Width::<f64, Pixel>::new(40.0),
            height: Height::<f64, Pixel>::new(20.0),
        };
        let size_clone = size.clone();

        assert_eq!(size, size_clone);
    }

    #[test]
    fn size_is_copy() {
        let size = Size {
            width: Width::<f64, Pixel>::new(40.0),
            height: Height::<f64, Pixel>::new(20.0),
        };
        let size_copy = size;

        assert_eq!(size, size_copy);
    }
}
