//! COLRv1 affine transform state shared by clips and brushes.

use kurbo::Point;

#[derive(Clone, Copy)]
pub(super) struct ColorTransform {
    xx: f64,
    yx: f64,
    xy: f64,
    yy: f64,
    dx: f64,
    dy: f64,
}

impl ColorTransform {
    pub(super) const IDENTITY: Self = Self {
        xx: 1.0,
        yx: 0.0,
        xy: 0.0,
        yy: 1.0,
        dx: 0.0,
        dy: 0.0,
    };

    pub(super) fn concat(self, next: Self) -> Self {
        Self {
            xx: self.xx * next.xx + self.xy * next.yx,
            yx: self.yx * next.xx + self.yy * next.yx,
            xy: self.xx * next.xy + self.xy * next.yy,
            yy: self.yx * next.xy + self.yy * next.yy,
            dx: self.xx * next.dx + self.xy * next.dy + self.dx,
            dy: self.yx * next.dx + self.yy * next.dy + self.dy,
        }
    }

    pub(super) fn inverse_apply(self, point: Point) -> Option<Point> {
        let determinant = self.xx * self.yy - self.xy * self.yx;
        if determinant.abs() <= f64::EPSILON {
            return None;
        }
        let x = point.x - self.dx;
        let y = point.y - self.dy;
        Some(Point::new(
            (self.yy * x - self.xy * y) / determinant,
            (-self.yx * x + self.xx * y) / determinant,
        ))
    }

    pub(super) fn to_kurbo(self) -> kurbo::Affine {
        kurbo::Affine::new([self.xx, self.yx, self.xy, self.yy, self.dx, self.dy])
    }
}

impl From<skrifa::color::Transform> for ColorTransform {
    fn from(value: skrifa::color::Transform) -> Self {
        Self {
            xx: f64::from(value.xx),
            yx: f64::from(value.yx),
            xy: f64::from(value.xy),
            yy: f64::from(value.yy),
            dx: f64::from(value.dx),
            dy: f64::from(value.dy),
        }
    }
}
