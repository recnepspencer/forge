use crate::authoring::{CollectionAuthoredQuery, CollectionAuthoredResultShape, CollectionFamily};

use super::query_family::{TypedAuthoredQuery, TypedQueryBuilder, TypedQuerySurfaceFamily};
use super::result_shape_family::{
    TypedAuthoredResultShape, TypedResultShapeBuilder, TypedResultShapeSurfaceFamily,
};
use super::traits::TypedSchemaRoot;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedCollectionFamily;

impl TypedQuerySurfaceFamily for TypedCollectionFamily {
    type AuthoringFamily = CollectionFamily;
}

impl TypedResultShapeSurfaceFamily for TypedCollectionFamily {
    type AuthoringFamily = crate::authoring::CollectionResultShapeFamily;
}

pub type TypedCollectionQuery<S> = TypedAuthoredQuery<S, TypedCollectionFamily>;
pub type TypedCollectionQueryBuilder<S> = TypedQueryBuilder<S, TypedCollectionFamily>;
pub type TypedCollectionResultShape<S> = TypedAuthoredResultShape<S, TypedCollectionFamily>;
pub type TypedCollectionResultShapeBuilder<S> = TypedResultShapeBuilder<S, TypedCollectionFamily>;

#[allow(dead_code)]
fn _type_check_collection_aliases<S: TypedSchemaRoot>(
    query: TypedCollectionQuery<S>,
    result_shape: TypedCollectionResultShape<S>,
) -> (CollectionAuthoredQuery, CollectionAuthoredResultShape) {
    (query.inner, result_shape.inner)
}
