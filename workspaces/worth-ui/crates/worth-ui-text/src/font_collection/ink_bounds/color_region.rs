use kurbo::{BezPath, Rect, Shape};
use linesweeper::{binary_op, BinaryOp, FillRule};
use skrifa::color::CompositeMode;

pub(super) struct Layer {
    pub(super) backdrop: Coverage,
    pub(super) composite_mode: CompositeMode,
}

#[derive(Clone, Default)]
pub(super) struct Coverage {
    support: Region,
    opaque: Region,
}

#[derive(Clone, Default)]
pub(super) struct Region {
    path: BezPath,
}

impl Coverage {
    pub(super) fn insert(
        &mut self,
        path: BezPath,
        provably_opaque: bool,
    ) -> Result<(), linesweeper::Error> {
        let region = Region::new(path);
        self.support = self.support.clone().union(region.clone())?;
        if provably_opaque {
            self.opaque = self.opaque.clone().union(region)?;
        }
        Ok(())
    }

    pub(super) fn bounds(&self) -> Option<Rect> {
        self.support.bounds()
    }

    pub(super) fn composite(
        source: Self,
        destination: Self,
        mode: CompositeMode,
    ) -> Result<Self, CompositionError> {
        use CompositeMode::*;
        Ok(match mode {
            Clear => Self::default(),
            Src => source,
            Dest => destination,
            SrcIn => Self {
                support: source.support.intersection(destination.support)?,
                opaque: source.opaque.intersection(destination.opaque)?,
            },
            DestIn => Self {
                support: destination.support.intersection(source.support)?,
                opaque: destination.opaque.intersection(source.opaque)?,
            },
            SrcOut => Self {
                support: source.support.difference(destination.opaque)?,
                opaque: source.opaque.difference(destination.support)?,
            },
            DestOut => Self {
                support: destination.support.difference(source.opaque)?,
                opaque: destination.opaque.difference(source.support)?,
            },
            SrcAtop => destination,
            DestAtop => source,
            Xor => Self {
                support: source
                    .support
                    .clone()
                    .difference(destination.opaque.clone())?
                    .union(
                        destination
                            .support
                            .clone()
                            .difference(source.opaque.clone())?,
                    )?,
                opaque: source
                    .opaque
                    .clone()
                    .difference(destination.support)?
                    .union(destination.opaque.difference(source.support)?)?,
            },
            Unknown => return Err(CompositionError::UnknownMode),
            SrcOver | DestOver | Plus | Screen | Overlay | Darken | Lighten | ColorDodge
            | ColorBurn | HardLight | SoftLight | Difference | Exclusion | Multiply | HslHue
            | HslSaturation | HslColor | HslLuminosity => Self {
                support: source.support.union(destination.support)?,
                opaque: source.opaque.union(destination.opaque)?,
            },
        })
    }
}

#[derive(Debug)]
pub(super) enum CompositionError {
    Geometry,
    UnknownMode,
}

impl From<linesweeper::Error> for CompositionError {
    fn from(_: linesweeper::Error) -> Self {
        Self::Geometry
    }
}

impl Region {
    pub(super) fn new(path: BezPath) -> Self {
        Self { path }
    }

    pub(super) fn path(self) -> BezPath {
        self.path
    }

    pub(super) fn intersect(self, other: Self) -> Result<Self, linesweeper::Error> {
        self.operation(other, BinaryOp::Intersection)
    }

    fn union(self, other: Self) -> Result<Self, linesweeper::Error> {
        self.operation(other, BinaryOp::Union)
    }

    fn intersection(self, other: Self) -> Result<Self, linesweeper::Error> {
        self.operation(other, BinaryOp::Intersection)
    }

    fn difference(self, other: Self) -> Result<Self, linesweeper::Error> {
        self.operation(other, BinaryOp::Difference)
    }

    fn operation(self, other: Self, operation: BinaryOp) -> Result<Self, linesweeper::Error> {
        if self.path.is_empty() {
            return Ok(match operation {
                BinaryOp::Union | BinaryOp::Xor => other,
                BinaryOp::Intersection | BinaryOp::Difference => Self::default(),
            });
        }
        if other.path.is_empty() {
            return Ok(match operation {
                BinaryOp::Union | BinaryOp::Difference | BinaryOp::Xor => self,
                BinaryOp::Intersection => Self::default(),
            });
        }
        let contours = binary_op(&self.path, &other.path, FillRule::NonZero, operation)?;
        let mut path = BezPath::new();
        for contour in contours.contours() {
            path.extend(contour.path.iter());
        }
        Ok(Self { path })
    }

    fn bounds(&self) -> Option<Rect> {
        (!self.path.is_empty()).then(|| self.path.bounding_box())
    }
}
