use schema::facade::platform::authority::DerivedInvalidationTarget;
use schema::facade::topology_authoring::DerivedTopologyReadBasis;
use serde::Serialize;

use crate::compiled_product_family::triggered_invalidation_targets_from_read_basis;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationTouchedClosure;

use super::denial::{
    TopologyCompiledProductAdmissionError, TopologyCompiledProductAdmissionErrorKind,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TopologyCompiledProductLocalityBasis {
    locality_digest: String,
    triggered_invalidation_targets: Vec<DerivedInvalidationTarget>,
}

impl TopologyCompiledProductLocalityBasis {
    pub fn from_read_basis(read_basis: &DerivedTopologyReadBasis) -> Self {
        Self {
            locality_digest: crate::compiled_product_family::topology_invalidation_closure_digest(
                read_basis.snapshot().snapshot_id.0,
                &read_basis.branch_id().0,
                read_basis
                    .authority
                    .truth_basis_identity
                    .touched_aspect_count,
                &triggered_invalidation_targets_from_read_basis(read_basis),
            ),
            triggered_invalidation_targets: triggered_invalidation_targets_from_read_basis(
                read_basis,
            ),
        }
    }

    pub fn from_selected_plan(
        read_basis: &DerivedTopologyReadBasis,
        touched_closure: &DerivedInvalidationTouchedClosure,
    ) -> Result<Self, TopologyCompiledProductAdmissionError> {
        if read_basis.touched_aspects().len() != touched_closure.counters().touched_aspect_count() {
            return Err(TopologyCompiledProductAdmissionError::new(
                TopologyCompiledProductAdmissionErrorKind::ReadBasisNotBoundToTouchedClosure,
                "derived topology read basis touched aspect count did not match touched closure",
            ));
        }
        Ok(Self::from_read_basis(read_basis))
    }

    pub fn locality_digest(&self) -> &str {
        &self.locality_digest
    }

    pub fn triggered_invalidation_targets(&self) -> &[DerivedInvalidationTarget] {
        &self.triggered_invalidation_targets
    }
}
