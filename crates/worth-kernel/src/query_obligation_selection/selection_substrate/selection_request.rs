use forge_query::facade::runtime::{
    ForgeQueryGraphObligationOperatingWorldDescriptor, ForgeQueryGraphTouchDescriptor,
};
use worth_spatial::facade::workload_vocabulary::SpatialEvidenceQueryTouchDescriptor;

use super::selection_error::QueryObligationSelectionError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryObligationSelectionAuthorityKind {
    TopologyTouchedBasis,
    SpatialQueryDescriptor,
}

#[derive(Clone, Debug)]
pub struct QueryObligationSelectionInput {
    touch_descriptor: ForgeQueryGraphTouchDescriptor,
    operating_world: ForgeQueryGraphObligationOperatingWorldDescriptor,
    authority_digest: String,
    authority_kind: QueryObligationSelectionAuthorityKind,
    spatial_descriptor: Option<SpatialEvidenceQueryTouchDescriptor>,
}

impl QueryObligationSelectionInput {
    pub(crate) fn from_authority_parts(
        touch_descriptor: ForgeQueryGraphTouchDescriptor,
        operating_world: ForgeQueryGraphObligationOperatingWorldDescriptor,
        authority_digest: impl Into<String>,
        authority_kind: QueryObligationSelectionAuthorityKind,
    ) -> Result<Self, QueryObligationSelectionError> {
        let authority_digest = authority_digest.into();
        if authority_digest.trim().is_empty() {
            return Err(QueryObligationSelectionError::missing_authority_digest());
        }
        Ok(Self {
            touch_descriptor,
            operating_world,
            authority_digest,
            authority_kind,
            spatial_descriptor: None,
        })
    }

    pub(crate) fn with_spatial_descriptor(
        mut self,
        descriptor: SpatialEvidenceQueryTouchDescriptor,
    ) -> Self {
        self.spatial_descriptor = Some(descriptor);
        self
    }

    pub fn touch_descriptor(&self) -> &ForgeQueryGraphTouchDescriptor {
        &self.touch_descriptor
    }

    pub fn operating_world(&self) -> &ForgeQueryGraphObligationOperatingWorldDescriptor {
        &self.operating_world
    }

    pub fn authority_digest(&self) -> &str {
        &self.authority_digest
    }

    pub const fn authority_kind(&self) -> QueryObligationSelectionAuthorityKind {
        self.authority_kind
    }

    pub(crate) fn spatial_descriptor(&self) -> Option<&SpatialEvidenceQueryTouchDescriptor> {
        self.spatial_descriptor.as_ref()
    }
}
