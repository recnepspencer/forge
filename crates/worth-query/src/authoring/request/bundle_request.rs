use crate::authoring::{RawAuthoredQuery, RawAuthoredResultShape};
use crate::binding::QueryBindingDescriptor;

use super::compatibility::{enforce_family_match, enforce_shape_projection_compatibility};
use super::error::AuthoredBundleError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthoredQueryBundleRequest {
    query: RawAuthoredQuery,
    result_shape: RawAuthoredResultShape,
    bindings: QueryBindingDescriptor,
    helper_residue_marker: Option<&'static str>,
}

impl AuthoredQueryBundleRequest {
    pub(crate) fn new(
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
            helper_residue_marker: None,
        })
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

    pub(crate) fn helper_residue_marker(&self) -> Option<&'static str> {
        self.helper_residue_marker
    }

    #[cfg(test)]
    pub(crate) fn with_helper_residue_for_test(mut self, residue: &'static str) -> Self {
        self.helper_residue_marker = Some(residue);
        self
    }
}
