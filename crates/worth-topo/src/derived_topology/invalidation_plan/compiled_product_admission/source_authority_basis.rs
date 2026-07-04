use schema::facade::topology_authoring::DerivedTopologyReadBasis;
use serde::Serialize;

use super::denial::{
    TopologyCompiledProductAdmissionError, TopologyCompiledProductAdmissionErrorKind,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TopologyCompiledProductSourceAuthorityBasis {
    authority_snapshot_id: u64,
    authority_branch_id: String,
    truth_basis_digest_hex: String,
    touched_aspect_count: usize,
    precision_fallback_count: usize,
    precision_budget_fallback_count: usize,
}

impl TopologyCompiledProductSourceAuthorityBasis {
    pub fn from_read_basis(
        read_basis: &DerivedTopologyReadBasis,
    ) -> Result<Self, TopologyCompiledProductAdmissionError> {
        let touched_aspect_count = read_basis.touched_aspects().len();
        if touched_aspect_count
            != read_basis
                .authority
                .truth_basis_identity
                .touched_aspect_count
        {
            return Err(TopologyCompiledProductAdmissionError::new(
                TopologyCompiledProductAdmissionErrorKind::InvalidTruthBasisCount,
                "derived topology read basis touched aspect count did not match truth basis identity",
            ));
        }
        Ok(Self {
            authority_snapshot_id: read_basis.snapshot().snapshot_id.0,
            authority_branch_id: read_basis.branch_id().0.clone(),
            truth_basis_digest_hex: read_basis
                .authority
                .truth_basis_identity
                .mutation_digest_hex
                .clone(),
            touched_aspect_count,
            precision_fallback_count: read_basis.precision_fallbacks.len(),
            precision_budget_fallback_count: read_basis.precision_budget_fallbacks.len(),
        })
    }

    pub const fn authority_snapshot_id(&self) -> u64 {
        self.authority_snapshot_id
    }

    pub fn authority_branch_id(&self) -> &str {
        &self.authority_branch_id
    }

    pub fn truth_basis_digest_hex(&self) -> &str {
        &self.truth_basis_digest_hex
    }

    pub const fn touched_aspect_count(&self) -> usize {
        self.touched_aspect_count
    }

    pub const fn precision_fallback_count(&self) -> usize {
        self.precision_fallback_count
    }

    pub const fn precision_budget_fallback_count(&self) -> usize {
        self.precision_budget_fallback_count
    }
}
