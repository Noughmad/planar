use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div};

use oned::*;
use twod::*;

pub trait AxisAlignedTransform<T, UnitFrom> {
    type OutT;
    type OutUnit;

    fn transform_position_x(&self, x: PosX<T, UnitFrom>) -> PosX<Self::OutT, Self::OutUnit>;
    fn transform_position_y(&self, y: PosY<T, UnitFrom>) -> PosY<Self::OutT, Self::OutUnit>;

    fn transform_width(&self, w: Width<T, UnitFrom>) -> Width<Self::OutT, Self::OutUnit>;
    fn transform_height(&self, h: Height<T, UnitFrom>) -> Height<Self::OutT, Self::OutUnit>;

    fn transform_size(&self, s: Size<T, UnitFrom>) -> Size<Self::OutT, Self::OutUnit> {
        Size {
            width: self.transform_width(s.width),
            height: self.transform_height(s.height),
        }
    }

    fn transform_rect(&self, r: Rect<T, UnitFrom>) -> Rect<Self::OutT, Self::OutUnit> {
        Rect {
            origin: self.transform_point(r.origin),
            size: self.transform_size(r.size),
        }
    }

    fn transform_point(&self, p: Point<T, UnitFrom>) -> Point<Self::OutT, Self::OutUnit> {
        Point {
            x: self.transform_position_x(p.x),
            y: self.transform_position_y(p.y),
        }
    }
}

pub trait Transform<T, UnitFrom> {
    type OutT;
    type OutUnit;

    fn transform_point(&self, p: Point<T, UnitFrom>) -> Point<Self::OutT, Self::OutUnit>;
}

impl<T, UnitFrom, W, UnitTo> Transform<T, UnitFrom>
    for AxisAlignedTransform<T, UnitFrom, OutT = W, OutUnit = UnitTo> {
    type OutT = W;
    type OutUnit = UnitTo;

    fn transform_point(&self, p: Point<T, UnitFrom>) -> Point<Self::OutT, Self::OutUnit> {
        Point {
            x: self.transform_position_x(p.x),
            y: self.transform_position_y(p.y),
        }
    }
}

pub struct IdentityTransform<T, UnitFrom, W, UnitTo>(PhantomData<(T, UnitFrom, W, UnitTo)>);

impl<T: Into<W>, UnitFrom, W, UnitTo> AxisAlignedTransform<T, UnitFrom>
    for IdentityTransform<T, UnitFrom, W, UnitTo> {
    type OutT = W;
    type OutUnit = UnitTo;

    fn transform_position_x(&self, x: PosX<T, UnitFrom>) -> PosX<W, UnitTo> {
        PosX::new(x.into_inner().into())
    }
    fn transform_position_y(&self, y: PosY<T, UnitFrom>) -> PosY<W, UnitTo> {
        PosY::new(y.into_inner().into())
    }

    fn transform_width(&self, w: Width<T, UnitFrom>) -> Width<W, UnitTo> {
        Width::new(w.into_inner().into())
    }
    fn transform_height(&self, h: Height<T, UnitFrom>) -> Height<W, UnitTo> {
        Height::new(h.into_inner().into())
    }
}

pub struct Translation<T, Unit>(Size<T, Unit>);

impl<T: Clone + Add<T, Output = T>, Unit> AxisAlignedTransform<T, Unit> for Translation<T, Unit> {
    type OutT = T;
    type OutUnit = Unit;

    fn transform_position_x(&self, x: PosX<T, Unit>) -> PosX<T, Unit> {
        x + self.0.width.clone()
    }
    fn transform_position_y(&self, y: PosY<T, Unit>) -> PosY<T, Unit> {
        y + self.0.height.clone()
    }

    fn transform_width(&self, w: Width<T, Unit>) -> Width<T, Unit> {
        w
    }
    fn transform_height(&self, h: Height<T, Unit>) -> Height<T, Unit> {
        h
    }
}

pub struct ScaleFactor<T: Mul<V, Output = W>, V: Clone, W, UnitFrom, UnitTo>(
    V,
    PhantomData<(T, UnitFrom, W, UnitTo)>
);

impl<T: Clone + Mul<V, Output = W>, V: Clone, UnitFrom, W, UnitTo> AxisAlignedTransform<T, UnitFrom>
    for ScaleFactor<T, V, W, UnitFrom, UnitTo> {
    type OutT = W;
    type OutUnit = UnitTo;

    fn transform_position_x(&self, x: PosX<T, UnitFrom>) -> PosX<W, UnitTo> {
        PosX::new(x.into_inner() * self.0.clone())
    }
    fn transform_position_y(&self, y: PosY<T, UnitFrom>) -> PosY<W, UnitTo> {
        PosY::new(y.into_inner() * self.0.clone())
    }

    fn transform_width(&self, w: Width<T, UnitFrom>) -> Width<W, UnitTo> {
        Width::new(w.into_inner() * self.0.clone())
    }
    fn transform_height(&self, h: Height<T, UnitFrom>) -> Height<W, UnitTo> {
        Height::new(h.into_inner() * self.0.clone())
    }
}

pub struct MatrixTransform<T, UnitFrom, UnitTo>([T; 6], PhantomData<(UnitFrom, UnitTo)>);

impl<T, UnitFrom, UnitTo> MatrixTransform<T, UnitFrom, UnitTo> {
    pub fn new(data: [T; 6]) -> Self
    where T: Clone {
        MatrixTransform(data.clone(), PhantomData{})
    }

    pub fn iter<'a>(&'a self) -> ::std::slice::Iter<'a, T> {
        self.0.iter()
    }
}

impl<T, UnitFrom, UnitTo> Transform<T, UnitFrom>
    for MatrixTransform<T, UnitFrom, UnitTo>
where
    T: Clone + Add<T, Output = T> + Mul<T, Output = T>,
{
    type OutT = T;
    type OutUnit = UnitTo;

    fn transform_point(&self, p: Point<T, UnitFrom>) -> Point<T, UnitTo> {
        Point {
            x: PosX::new(
                p.x.get() * self.0[0].clone() + p.y.get() * self.0[2].clone() + self.0[4].clone(),
            ),
            y: PosY::new(
                p.x.get() * self.0[1].clone() + p.y.get() * self.0[3].clone() + self.0[5].clone(),
            ),
        }
    }
}

pub struct AxisAlignedMatrixTransform<T, V, W, Y, Z, UnitFrom, UnitTo>(
    V,
    V,
    Y,
    Y,
    PhantomData<(UnitFrom, UnitTo, T, W, Z)>
);

impl<T, V, W, Y, Z, UnitFrom, UnitTo> AxisAlignedTransform<T, UnitFrom>
    for AxisAlignedMatrixTransform<T, V, W, Y, Z, UnitFrom, UnitTo>
where
    T: Mul<V, Output = W>,
    V: Clone,
    W: Add<Y, Output = Z> + Into<Z>,
    Y: Clone,
{
    type OutT = Z;
    type OutUnit = UnitTo;

    fn transform_position_x(&self, x: PosX<T, UnitFrom>) -> PosX<Z, UnitTo> {
        PosX::new(x.into_inner() * self.0.clone() + self.2.clone())
    }
    fn transform_position_y(&self, y: PosY<T, UnitFrom>) -> PosY<Z, UnitTo> {
        PosY::new(y.into_inner() * self.1.clone() + self.3.clone())
    }

    fn transform_width(&self, w: Width<T, UnitFrom>) -> Width<Z, UnitTo> {
        Width::new((w.into_inner() * self.0.clone()).into())
    }
    fn transform_height(&self, h: Height<T, UnitFrom>) -> Height<Z, UnitTo> {
        Height::new((h.into_inner() * self.1.clone()).into())
    }

    fn transform_point(&self, p: Point<T, UnitFrom>) -> Point<Z, UnitTo> {
        Point {
            x: PosX::new(p.x.into_inner() * self.0.clone() + self.2.clone()),
            y: PosY::new(p.y.into_inner() * self.1.clone() + self.3.clone()),
        }
    }

    fn transform_size(&self, s: Size<T, UnitFrom>) -> Size<Self::OutT, Self::OutUnit> {
        Size {
            width: Width::new((s.width.into_inner() * self.0.clone()).into()),
            height: Height::new((s.height.into_inner() * self.1.clone()).into()),
        }
    }
}

impl<T, V, W, Y, Z, UnitFrom, UnitTo> AxisAlignedMatrixTransform<T, V, W, Y, Z, UnitFrom, UnitTo> {
    pub fn new(scale_x: V, scale_y: V, translate_x: Y, translate_y: Y) -> Self {
        AxisAlignedMatrixTransform(scale_x, scale_y, translate_x, translate_y, PhantomData {})
    }

    pub fn from_rects(from: Rect<T, UnitFrom>, to: Rect<Z, UnitTo>) -> Self
    where
        Z: Div<T, Output = V> + Sub<W, Output = Y>,
        V: Clone,
        T: Mul<V, Output = W>,
    {
        let scale_x = to.size.width.into_inner() / from.size.width.into_inner();
        let scale_y = to.size.height.into_inner() / from.size.height.into_inner();

        let translate_x = to.origin.x.into_inner() - from.origin.x.into_inner() * scale_x.clone();
        let translate_y = to.origin.y.into_inner() - from.origin.y.into_inner() * scale_y.clone();

        AxisAlignedMatrixTransform::new(scale_x, scale_y, translate_x, translate_y)
    }
}

macro_rules! impl_mul_for_transform {
    ($mac:ident) => {
        $mac!(PosX, transform_position_x);
        $mac!(PosY, transform_position_y);

        $mac!(Width, transform_width);
        $mac!(Height, transform_height);

        $mac!(Point, transform_point);
        $mac!(Size, transform_size);
        $mac!(Rect, transform_rect);
    }
}

macro_rules! impl_scale_factor_mul {
    ($s:ident, $m:ident) => {
        impl<'a, T: Clone + Mul<V, Output = W>, V: Clone, W, UnitFrom, UnitTo> Mul<$s<T, UnitFrom>>
            for &'a ScaleFactor<T, V, W, UnitFrom, UnitTo> {
            type Output = $s<W, UnitTo>;
            fn mul(self, p: $s<T, UnitFrom>) -> Self::Output {
                self.$m(p)
            }
        }
    }
}

impl_mul_for_transform!(impl_scale_factor_mul);

macro_rules! impl_identity_mul {
    ($s:ident, $m:ident) => {
        impl<T: Into<W>, UnitFrom, W, UnitTo> Mul<$s<T, UnitFrom>>
        for IdentityTransform<T, UnitFrom, W, UnitTo> {
            type Output = $s<W, UnitTo>;
            fn mul(self, p: $s<T, UnitFrom>) -> Self::Output {
                self.$m(p)
            }
        }
    }
}

impl_mul_for_transform!(impl_identity_mul);

macro_rules! impl_translation_mul {
    ($s:ident, $m:ident) => {
        impl<T: Clone + Add<T, Output = T>, Unit> Mul<$s<T, Unit>>
        for Translation<T, Unit> {
            type Output = $s<T, Unit>;
            fn mul(self, p: $s<T, Unit>) -> Self::Output {
                self.$m(p)
            }
        }
    }
}

impl_mul_for_transform!(impl_translation_mul);

macro_rules! impl_axis_aligned_matrix_mul {
    ($s:ident, $m:ident) => {
    impl<T, V, W, Y, Z, UnitFrom, UnitTo> Mul<$s<T, UnitFrom>>
    for AxisAlignedMatrixTransform<T, V, W, Y, Z, UnitFrom, UnitTo>
where
    T: Mul<V, Output=W>,
    V: Clone,
    W: Add<Y, Output=Z> + Into<Z>,
    Y: Clone,
    {
            type Output = $s<Z, UnitTo>;
            fn mul(self, p: $s<T, UnitFrom>) -> Self::Output {
                self.$m(p)
            }
        }
    }
}

impl_mul_for_transform!(impl_axis_aligned_matrix_mul);

macro_rules! impl_matrix_mul {
    ($s:ident, $m:ident) => {
        impl<T, UnitFrom, UnitTo> Mul<$s<T, UnitFrom>>
            for MatrixTransform<T, UnitFrom, UnitTo>
        where
            T: Clone + Add<T, Output = T> + Mul<T, Output = T> {

            type Output = $s<T, UnitTo>;
            fn mul(self, p: $s<T, UnitFrom>) -> Self::Output {
                self.$m(p)
            }
        }
    }
}

impl_matrix_mul!(Point, transform_point);

#[cfg(test)]
mod tests {
    pub use super::*;

    struct Point;
    struct Pixel;

    #[test]
    fn scale_factor() {
        let w: Width<f64, Point> = Width::new(7.0);
        let f: ScaleFactor<f64, f64, f64, Point, Pixel> = ScaleFactor(12.0, PhantomData {});
        let w2: Width<f64, Pixel> = Width::new(84.0);

        assert_eq!(f.transform_width(w), w2);
    }

    #[test]
    fn scale_factor_inferred_types() {
        let w: Width<_, Point> = Width::new(7.0);
        let f = ScaleFactor(12.0, PhantomData {});
        let w2: Width<_, Pixel> = Width::new(84.0);

        assert_eq!(f.transform_width(w), w2);
    }

    #[test]
    fn scale_factor_mul() {
        let w: Width<_, Point> = Width::new(7.0);
        let f = ScaleFactor(12.0, PhantomData {});
        let w2: Width<_, Pixel> = Width::new(84.0);

        assert_eq!(&f * w, w2);
    }
}
