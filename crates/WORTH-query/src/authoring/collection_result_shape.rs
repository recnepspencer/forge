use super::{
    result_shape_family::{AuthoredResultShape, ResultShapeAuthoringFamily, ResultShapeBuilder},
    RawAuthoredResultShape,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectionResultShapeFamily;

impl ResultShapeAuthoringFamily for CollectionResultShapeFamily {
    fn initialize() -> RawAuthoredResultShape {
        RawAuthoredResultShape::collection()
    }
}

pub type CollectionAuthoredResultShape = AuthoredResultShape<CollectionResultShapeFamily>;
pub type CollectionResultShapeBuilder = ResultShapeBuilder<CollectionResultShapeFamily>;
