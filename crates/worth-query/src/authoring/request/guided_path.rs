use crate::authoring::{
    AuthoredQuery, AuthoredResultShape, CollectionAuthoredQuery, CollectionAuthoredResultShape,
    DetailAuthoredQuery, DetailAuthoredResultShape, QueryAuthoringFamily,
    ResultShapeAuthoringFamily,
};
use crate::binding::QueryBindingDescriptor;
use crate::canonicalization::{
    canonicalize_request, CanonicalQueryBundle, QueryCanonicalizationError,
};

use super::bundle_request::AuthoredQueryBundleRequest;
use super::error::AuthoredBundleError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuidedAuthoringPath;

impl GuidedAuthoringPath {
    pub fn pair_detail(
        query: DetailAuthoredQuery,
        result_shape: DetailAuthoredResultShape,
    ) -> Result<AuthoredQueryBundleRequest, AuthoredBundleError> {
        pair(query, result_shape, QueryBindingDescriptor::default())
    }

    pub fn pair_detail_with_bindings(
        query: DetailAuthoredQuery,
        result_shape: DetailAuthoredResultShape,
        bindings: QueryBindingDescriptor,
    ) -> Result<AuthoredQueryBundleRequest, AuthoredBundleError> {
        pair(query, result_shape, bindings)
    }

    pub fn pair_collection(
        query: CollectionAuthoredQuery,
        result_shape: CollectionAuthoredResultShape,
    ) -> Result<AuthoredQueryBundleRequest, AuthoredBundleError> {
        pair(query, result_shape, QueryBindingDescriptor::default())
    }

    pub fn pair_collection_with_bindings(
        query: CollectionAuthoredQuery,
        result_shape: CollectionAuthoredResultShape,
        bindings: QueryBindingDescriptor,
    ) -> Result<AuthoredQueryBundleRequest, AuthoredBundleError> {
        pair(query, result_shape, bindings)
    }

    pub(crate) fn canonicalize_detail(
        query: DetailAuthoredQuery,
        result_shape: DetailAuthoredResultShape,
    ) -> Result<CanonicalQueryBundle, QueryCanonicalizationError> {
        canonicalize(query, result_shape, QueryBindingDescriptor::default())
    }

    pub(crate) fn canonicalize_detail_with_bindings(
        query: DetailAuthoredQuery,
        result_shape: DetailAuthoredResultShape,
        bindings: QueryBindingDescriptor,
    ) -> Result<CanonicalQueryBundle, QueryCanonicalizationError> {
        canonicalize(query, result_shape, bindings)
    }

    pub(crate) fn canonicalize_collection(
        query: CollectionAuthoredQuery,
        result_shape: CollectionAuthoredResultShape,
    ) -> Result<CanonicalQueryBundle, QueryCanonicalizationError> {
        canonicalize(query, result_shape, QueryBindingDescriptor::default())
    }

    pub(crate) fn canonicalize_collection_with_bindings(
        query: CollectionAuthoredQuery,
        result_shape: CollectionAuthoredResultShape,
        bindings: QueryBindingDescriptor,
    ) -> Result<CanonicalQueryBundle, QueryCanonicalizationError> {
        canonicalize(query, result_shape, bindings)
    }
}

fn pair<Q, S>(
    query: AuthoredQuery<Q>,
    result_shape: AuthoredResultShape<S>,
    bindings: QueryBindingDescriptor,
) -> Result<AuthoredQueryBundleRequest, AuthoredBundleError>
where
    Q: QueryAuthoringFamily,
    S: ResultShapeAuthoringFamily,
{
    AuthoredQueryBundleRequest::new(query.into_raw(), result_shape.into_raw(), bindings)
}

fn canonicalize<Q, S>(
    query: AuthoredQuery<Q>,
    result_shape: AuthoredResultShape<S>,
    bindings: QueryBindingDescriptor,
) -> Result<CanonicalQueryBundle, QueryCanonicalizationError>
where
    Q: QueryAuthoringFamily,
    S: ResultShapeAuthoringFamily,
{
    let request = pair(query, result_shape, bindings).map_err(map_bundle_error)?;
    canonicalize_request(request)
}

fn map_bundle_error(error: AuthoredBundleError) -> QueryCanonicalizationError {
    match error {
        AuthoredBundleError::QueryShapeFamilyMismatch {
            query_family,
            result_shape_family,
        } => QueryCanonicalizationError::QueryShapeFamilyMismatch {
            query_family,
            result_shape_family,
        },
        AuthoredBundleError::UnprojectedShapeField {
            source_aspect,
            source_field,
            delivered_name,
        } => QueryCanonicalizationError::UnprojectedShapeField {
            source_aspect,
            source_field,
            delivered_name,
        },
    }
}
