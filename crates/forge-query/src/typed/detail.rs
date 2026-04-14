use crate::authoring::{DetailAuthoredQuery, DetailAuthoredResultShape, DetailFamily};

use super::query_family::{TypedAuthoredQuery, TypedQueryBuilder, TypedQuerySurfaceFamily};
use super::result_shape_family::{
    TypedAuthoredResultShape, TypedResultShapeBuilder, TypedResultShapeSurfaceFamily,
};
use super::traits::TypedSchemaRoot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedDetailFamily;

impl TypedQuerySurfaceFamily for TypedDetailFamily {
    type AuthoringFamily = DetailFamily;
}

impl TypedResultShapeSurfaceFamily for TypedDetailFamily {
    type AuthoringFamily = crate::authoring::DetailResultShapeFamily;
}

pub type TypedDetailQuery<S> = TypedAuthoredQuery<S, TypedDetailFamily>;
pub type TypedDetailQueryBuilder<S> = TypedQueryBuilder<S, TypedDetailFamily>;
pub type TypedDetailResultShape<S> = TypedAuthoredResultShape<S, TypedDetailFamily>;
pub type TypedDetailResultShapeBuilder<S> = TypedResultShapeBuilder<S, TypedDetailFamily>;

#[allow(dead_code)]
fn _type_check_detail_aliases<S: TypedSchemaRoot>(
    query: TypedDetailQuery<S>,
    result_shape: TypedDetailResultShape<S>,
) -> (DetailAuthoredQuery, DetailAuthoredResultShape) {
    (query.inner, result_shape.inner)
}
