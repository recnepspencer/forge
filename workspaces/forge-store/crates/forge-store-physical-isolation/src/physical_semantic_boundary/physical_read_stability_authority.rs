use forge_proof::{AuthorityMarker, AuthorityWitness};

use crate::{PhysicalIsolationEntryAdmission, PhysicalIsolationRootEpochBasis};

#[derive(Debug, Clone)]
pub struct PhysicalReadStabilityAuthority {
    recovered_root: String,
    root_epoch_basis: PhysicalIsolationRootEpochBasis,
}

#[derive(Debug, Clone)]
pub struct PhysicalReadStabilityCorrelationBasis {
    recovered_root: String,
    root_epoch_basis: PhysicalIsolationRootEpochBasis,
}

impl AuthorityMarker for PhysicalReadStabilityAuthority {}

pub fn admit_physical_read_stability_authority(
    entry: &PhysicalIsolationEntryAdmission,
) -> Result<PhysicalReadStabilityAuthority, core::convert::Infallible> {
    Ok(PhysicalReadStabilityAuthority::from_entry(entry))
}

impl PhysicalReadStabilityAuthority {
    fn from_entry(entry: &PhysicalIsolationEntryAdmission) -> Self {
        Self {
            recovered_root: entry.recovered_root().to_string(),
            root_epoch_basis: entry.root_epoch_basis(),
        }
    }

    pub fn recovered_root(&self) -> &str {
        &self.recovered_root
    }

    pub const fn root_epoch_basis(&self) -> PhysicalIsolationRootEpochBasis {
        self.root_epoch_basis
    }

    pub fn correlation_basis(&self) -> PhysicalReadStabilityCorrelationBasis {
        PhysicalReadStabilityCorrelationBasis {
            recovered_root: self.recovered_root.clone(),
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
    pub fn recovered_root(&self) -> &str {
        &self.recovered_root
    }

    pub const fn root_epoch_basis(&self) -> PhysicalIsolationRootEpochBasis {
        self.root_epoch_basis
    }
}
