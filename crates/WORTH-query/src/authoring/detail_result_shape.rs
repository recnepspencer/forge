use super::{
    result_shape_family::{AuthoredResultShape, ResultShapeAuthoringFamily, ResultShapeBuilder},
    RawAuthoredResultShape,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DetailResultShapeFamily;

impl ResultShapeAuthoringFamily for DetailResultShapeFamily {
    fn initialize() -> RawAuthoredResultShape {
        RawAuthoredResultShape::detail()
    }
}

pub type DetailAuthoredResultShape = AuthoredResultShape<DetailResultShapeFamily>;
pub type DetailResultShapeBuilder = ResultShapeBuilder<DetailResultShapeFamily>;
