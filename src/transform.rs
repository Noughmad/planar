use std::marker::PhantomData;
use std::ops::{Add, Mul};

use oned::*;
use twod::*;

pub trait AxisAlignedTransform<T, UnitFrom, W, UnitTo> {
    fn transform_position_x(&self, x: PosX<T, UnitFrom>) -> PosX<W, UnitTo>;
    fn transform_position_y(&self, y: PosY<T, UnitFrom>) -> PosY<W, UnitTo>;

    fn transform_width(&self, w: Width<T, UnitFrom>) -> Width<W, UnitTo>;
    fn transform_height(&self, h: Height<T, UnitFrom>) -> Height<W, UnitTo>;

    fn transform_size(&self, s: Size<T, UnitFrom>) -> Size<W, UnitTo> {
        Size {
            width: self.transform_width(s.width),
            height: self.transform_height(s.height),
        }
    }

    fn transform_rect(&self, r: Rect<T, UnitFrom>) -> Rect<W, UnitTo> {
        Rect {
            origin: self.transform_point(r.origin),
            size: self.transform_size(r.size),
        }
    }

    fn transform_point(&self, p: Point<T, UnitFrom>) -> Point<W, UnitTo> {
        Point {
            x: self.transform_position_x(p.x),
            y: self.transform_position_y(p.y),
        }
    }
}

pub trait Transform<T, UnitFrom, W, UnitTo> {
    fn transform_point(&self, p: Point<T, UnitFrom>) -> Point<W, UnitTo>;
}

impl<T, UnitFrom, W, UnitTo> Transform<T, UnitFrom, W, UnitTo>
    for AxisAlignedTransform<T, UnitFrom, W, UnitTo> {
    fn transform_point(&self, p: Point<T, UnitFrom>) -> Point<W, UnitTo> {
        Point {
            x: self.transform_position_x(p.x),
            y: self.transform_position_y(p.y),
        }
    }
}

pub struct IdentityTransform<T, UnitFrom, W, UnitTo>(PhantomData<(T, UnitFrom, W, UnitTo)>);

impl<T: Into<W>, UnitFrom, W, UnitTo> AxisAlignedTransform<T, UnitFrom, W, UnitTo>
    for IdentityTransform<T, UnitFrom, W, UnitTo> {
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

impl<T: Clone + Add<T, Output = T>, Unit> AxisAlignedTransform<T, Unit, T, Unit>
    for Translation<T, Unit> {
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
> AxisAlignedTransform<T, UnitFrom, W, UnitTo> for ScaleFactor<T, V, W, UnitFrom, UnitTo> {
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

impl<T: Clone, UnitFrom, UnitTo> Transform<T, UnitFrom, T, UnitTo>
    for MatrixTransform<T, UnitFrom, UnitTo>
where
    T: Clone + Add<T, Output = T> + Mul<T, Output = T>,
{
    fn transform_point(&self, p: Point<T, UnitFrom>) -> Point<T, UnitTo> {
        Point {
            x: PosX::new(
                p.x.get() * self.0[0].clone() + p.y.get() * self.0[1].clone() + self.0[2].clone(),
            ),
            y: PosY::new(
                p.x.get() * self.0[3].clone() + p.y.get() * self.0[4].clone() + self.0[5].clone(),
            ),
        }
    }
}
