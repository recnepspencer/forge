use topology::facade::{
    TopologyDeclaredTouchedGraphBasisProof, TopologyPrimitiveConstructionBirthDeclaredTouchedBasis,
};
use worth_spatial::facade::workload_vocabulary::SpatialEvidenceQueryTouchDescriptor;

use super::super::selection_substrate::QueryObligationSelectionInput;
use super::error::QueryGraphObligationSelectionFacadeError;
use super::kinds::QueryGraphObligationSelectionAuthorityKind;

#[derive(Clone, Debug)]
pub struct QueryGraphObligationSelectionRequest {
    input: QueryObligationSelectionInput,
}

impl QueryGraphObligationSelectionRequest {
    pub fn from_topology_touched_basis(
        touched_basis: &TopologyDeclaredTouchedGraphBasisProof,
    ) -> Result<Self, QueryGraphObligationSelectionFacadeError> {
        Ok(Self {
            input: QueryObligationSelectionInput::from_topology_touched_basis(touched_basis)?,
        })
    }

    pub fn from_primitive_construction_touched_basis(
        touched_basis: &TopologyPrimitiveConstructionBirthDeclaredTouchedBasis,
    ) -> Result<Self, QueryGraphObligationSelectionFacadeError> {
        Self::from_topology_touched_basis(touched_basis.proof())
    }

    pub fn from_spatial_descriptor(
        descriptor: &SpatialEvidenceQueryTouchDescriptor,
    ) -> Result<Self, QueryGraphObligationSelectionFacadeError> {
        Ok(Self {
            input: QueryObligationSelectionInput::from_spatial_query_descriptor(descriptor)?,
        })
    }

    pub fn touch_descriptor_digest(&self) -> &str {
        self.input.touch_descriptor().descriptor_digest()
    }

    pub fn authority_digest(&self) -> &str {
        self.input.authority_digest()
    }

    pub fn authority_kind(&self) -> QueryGraphObligationSelectionAuthorityKind {
        self.input.authority_kind().into()
    }

    pub(crate) fn spatial_descriptor(&self) -> Option<&SpatialEvidenceQueryTouchDescriptor> {
        self.input.spatial_descriptor()
    }

    pub(crate) fn into_selection_input(self) -> QueryObligationSelectionInput {
        self.input
    }
}
