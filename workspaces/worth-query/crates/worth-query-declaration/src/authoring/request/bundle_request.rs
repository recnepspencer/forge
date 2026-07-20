use crate::authoring::{OrderingSelector, QueryFamily, RawAuthoredQuery, RawAuthoredResultShape};
use crate::binding::QueryBindingDescriptor;

use super::compatibility::{enforce_family_match, enforce_shape_projection_compatibility};
use super::error::AuthoredBundleError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthoredQueryBundleRequest {
    query: RawAuthoredQuery,
    result_shape: RawAuthoredResultShape,
    bindings: QueryBindingDescriptor,
}

impl AuthoredQueryBundleRequest {
    pub fn new(
        query: RawAuthoredQuery,
        result_shape: RawAuthoredResultShape,
        bindings: QueryBindingDescriptor,
    ) -> Result<Self, AuthoredBundleError> {
        enforce_family_match(query.family(), result_shape.family())?;
        enforce_shape_projection_compatibility(&query, &result_shape)?;
        Ok(Self {
            query,
            result_shape,
            bindings,
        })
    }

    pub fn for_ordinary_read(
        mut query: RawAuthoredQuery,
        result_shape: RawAuthoredResultShape,
        bindings: QueryBindingDescriptor,
    ) -> Result<Self, AuthoredBundleError> {
        if query.family() == QueryFamily::Collection && query.ordering().is_empty() {
            query = query.with_ordering(
                OrderingSelector::ascending("identity", "id")
                    .expect("ordinary collection cursor vocabulary is a valid selector"),
            );
        }
        Self::new(query, result_shape, bindings)
    }

    pub fn query(&self) -> &RawAuthoredQuery {
        &self.query
    }

    pub fn result_shape(&self) -> &RawAuthoredResultShape {
        &self.result_shape
    }

    pub fn bindings(&self) -> &QueryBindingDescriptor {
        &self.bindings
    }

    pub fn into_parts(
        self,
    ) -> (
        RawAuthoredQuery,
        RawAuthoredResultShape,
        QueryBindingDescriptor,
    ) {
        (self.query, self.result_shape, self.bindings)
    }
}
