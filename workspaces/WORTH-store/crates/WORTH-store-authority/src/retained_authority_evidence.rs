use worth_store_aspect_native::{StoreAspectIdentity, StorePhysicalBoundaryWitness};

use crate::StoreCurrentAuthorityWitness;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreRetainedAuthorityEvidence {
    identity: StoreAspectIdentity,
    physical_witness: StorePhysicalBoundaryWitness,
}

impl StoreRetainedAuthorityEvidence {
    pub(crate) fn from_current_authority(current_authority: &StoreCurrentAuthorityWitness) -> Self {
        Self {
            identity: current_authority.identity().clone(),
            physical_witness: current_authority.physical_witness(),
        }
    }

    pub const fn identity(&self) -> &StoreAspectIdentity {
        &self.identity
    }

    pub const fn physical_witness(&self) -> StorePhysicalBoundaryWitness {
        self.physical_witness
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreRetainedAuthorityEvidenceComparison {
    same_identity: bool,
    same_physical_witness: bool,
}

impl StoreRetainedAuthorityEvidenceComparison {
    pub const fn same_identity(&self) -> bool {
        self.same_identity
    }

    pub const fn same_physical_witness(&self) -> bool {
        self.same_physical_witness
    }

    pub const fn proves_same_retained_authority(&self) -> bool {
        self.same_identity && self.same_physical_witness
    }
}

pub fn report_retained_store_authority_evidence(
    current_authority: &StoreCurrentAuthorityWitness,
) -> StoreRetainedAuthorityEvidence {
    StoreRetainedAuthorityEvidence::from_current_authority(current_authority)
}

pub fn compare_retained_store_authority_evidence(
    left: &StoreRetainedAuthorityEvidence,
    right: &StoreRetainedAuthorityEvidence,
) -> StoreRetainedAuthorityEvidenceComparison {
    StoreRetainedAuthorityEvidenceComparison {
        same_identity: left.identity == right.identity,
        same_physical_witness: left.physical_witness == right.physical_witness,
    }
}
