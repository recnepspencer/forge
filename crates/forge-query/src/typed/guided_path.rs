use crate::authoring::GuidedAuthoringPath;
use crate::binding::QueryBindingDescriptor;
use crate::canonicalization::{CanonicalQueryBundle, QueryCanonicalizationError};

use super::collection::{TypedCollectionQuery, TypedCollectionResultShape};
use super::detail::{TypedDetailQuery, TypedDetailResultShape};
use super::traits::TypedSchemaRoot;

pub struct TypedGuidedAuthoringPath;

impl TypedGuidedAuthoringPath {
    pub fn canonicalize_detail<S: TypedSchemaRoot>(
        query: TypedDetailQuery<S>,
        result_shape: TypedDetailResultShape<S>,
    ) -> Result<CanonicalQueryBundle, QueryCanonicalizationError> {
        GuidedAuthoringPath::canonicalize_detail(query.inner, result_shape.inner)
    }

    pub fn canonicalize_detail_with_bindings<S: TypedSchemaRoot>(
        query: TypedDetailQuery<S>,
        result_shape: TypedDetailResultShape<S>,
        bindings: QueryBindingDescriptor,
    ) -> Result<CanonicalQueryBundle, QueryCanonicalizationError> {
        GuidedAuthoringPath::canonicalize_detail_with_bindings(
            query.inner,
            result_shape.inner,
            bindings,
        )
    }

    pub fn canonicalize_collection<S: TypedSchemaRoot>(
        query: TypedCollectionQuery<S>,
        result_shape: TypedCollectionResultShape<S>,
    ) -> Result<CanonicalQueryBundle, QueryCanonicalizationError> {
        GuidedAuthoringPath::canonicalize_collection(query.inner, result_shape.inner)
    }

    pub fn canonicalize_collection_with_bindings<S: TypedSchemaRoot>(
        query: TypedCollectionQuery<S>,
        result_shape: TypedCollectionResultShape<S>,
        bindings: QueryBindingDescriptor,
    ) -> Result<CanonicalQueryBundle, QueryCanonicalizationError> {
        GuidedAuthoringPath::canonicalize_collection_with_bindings(
            query.inner,
            result_shape.inner,
            bindings,
        )
    }
}
