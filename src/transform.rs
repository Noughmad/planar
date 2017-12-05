use std::marker::PhantomData;
use std::ops::{Add, Mul};

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
    for AxisAlignedTransform<T, UnitFrom, OutT=W, OutUnit=UnitTo> {

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

impl<T: Clone + Add<T, Output = T>, Unit> AxisAlignedTransform<T, Unit>
    for Translation<T, Unit> {

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

impl<
    T: Clone + Mul<V, Output = W>,
    V: Clone,
    UnitFrom,
    W,
    UnitTo,
> AxisAlignedTransform<T, UnitFrom> for ScaleFactor<T, V, W, UnitFrom, UnitTo> {

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

pub struct MatrixTransform<T, UnitFrom, UnitTo>([T; 6], PhantomData<UnitFrom>, PhantomData<UnitTo>);

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
        impl<'a, T: Clone + Mul<V, Output = W>, V: Clone, W, UnitFrom, UnitTo> Mul<$s<T, UnitFrom>> for &'a ScaleFactor<T, V, W, UnitFrom, UnitTo> {
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
        impl<T: Into<W>, UnitFrom, W, UnitTo> Mul<$s<T, UnitFrom>> for IdentityTransform<T, UnitFrom, W, UnitTo> {
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
        let f: ScaleFactor<f64, f64, f64, Point, Pixel> = ScaleFactor(12.0, PhantomData{});
        let w2: Width<f64, Pixel> = Width::new(84.0);

        assert_eq!(f.transform_width(w), w2);
    }

    #[test]
    fn scale_factor_inferred_types() {
        let w: Width<_, Point> = Width::new(7.0);
        let f = ScaleFactor(12.0, PhantomData{});
        let w2: Width<_, Pixel> = Width::new(84.0);

        assert_eq!(f.transform_width(w), w2);
    }

    #[test]
    fn scale_factor_mul() {
        let w: Width<_, Point> = Width::new(7.0);
        let f = ScaleFactor(12.0, PhantomData{});
        let w2: Width<_, Pixel> = Width::new(84.0);

        assert_eq!(&f * w, w2);
    }
}
