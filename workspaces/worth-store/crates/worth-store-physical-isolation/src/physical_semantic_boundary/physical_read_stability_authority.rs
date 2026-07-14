use worth_proof::{AuthorityMarker, AuthorityWitness};

use crate::{
    CompactionCutoverStabilityProof, PhysicalIsolationEntryAdmission,
    PhysicalIsolationRootEpochBasis,
};

#[derive(Debug, Clone)]
pub struct PhysicalReadStabilityAuthority {
    root_epoch_basis: PhysicalIsolationRootEpochBasis,
}

#[derive(Debug, Clone)]
pub struct PhysicalReadStabilityCorrelationBasis {
    root_epoch_basis: PhysicalIsolationRootEpochBasis,
}

impl AuthorityMarker for PhysicalReadStabilityAuthority {}

pub fn admit_physical_read_stability_authority(
    entry: &PhysicalIsolationEntryAdmission,
) -> Result<PhysicalReadStabilityAuthority, core::convert::Infallible> {
    Ok(PhysicalReadStabilityAuthority::from_entry(entry))
}

pub fn admit_post_compaction_read_stability_authority(
    proof: &CompactionCutoverStabilityProof,
) -> Result<PhysicalReadStabilityAuthority, core::convert::Infallible> {
    Ok(PhysicalReadStabilityAuthority::from_current_root(
        proof.post_cutover_root(),
    ))
}

impl PhysicalReadStabilityAuthority {
    fn from_entry(entry: &PhysicalIsolationEntryAdmission) -> Self {
        Self {
            root_epoch_basis: entry.root_epoch_basis(),
        }
    }

    fn from_current_root(root: crate::CurrentPhysicalRoot) -> Self {
        Self {
            root_epoch_basis: PhysicalIsolationRootEpochBasis::from_current_root(root),
        }
    }

    pub const fn root_epoch_basis(&self) -> PhysicalIsolationRootEpochBasis {
        self.root_epoch_basis
    }

    pub fn correlation_basis(&self) -> PhysicalReadStabilityCorrelationBasis {
        PhysicalReadStabilityCorrelationBasis {
            root_epoch_basis: self.root_epoch_basis,
        }
    }

    pub fn authority_witness(&self) -> AuthorityWitness<PhysicalReadStabilityAuthority> {
        AuthorityWitness::from_authority_marker(self.clone())
    }

    pub const fn is_store_physical_stability_authority(&self) -> bool {
        true
    }
}

impl PhysicalReadStabilityCorrelationBasis {
    pub const fn root_epoch_basis(&self) -> PhysicalIsolationRootEpochBasis {
        self.root_epoch_basis
    }
}
