use forge_proof::raw::{AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness};

use crate::access::execution::{S8AccessLoweringBasis, S8AccessPathCounterSnapshot};
use crate::materialization::S8LayoutCoverageWitness;

pub(crate) struct S8LoweringCapability;
impl CapabilityMarker for S8LoweringCapability {}

pub(crate) struct S8ExecutionReadinessAuthority;
impl AuthorityMarker for S8ExecutionReadinessAuthority {}

pub(crate) struct S8ReadmissionAuthority;
impl AuthorityMarker for S8ReadmissionAuthority {}

pub(crate) fn lowering_capability() -> CapabilityWitness<S8LoweringCapability> {
    CapabilityWitness::from_capability_marker(S8LoweringCapability)
}

pub(crate) fn readiness_authority() -> AuthorityWitness<S8ExecutionReadinessAuthority> {
    AuthorityWitness::from_authority_marker(S8ExecutionReadinessAuthority)
}

pub(crate) fn readmission_authority() -> AuthorityWitness<S8ReadmissionAuthority> {
    AuthorityWitness::from_authority_marker(S8ReadmissionAuthority)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8ExecutionReadmissionWitness {
    basis: S8AccessLoweringBasis,
    planned: S8AccessPathCounterSnapshot,
    coverage: S8LayoutCoverageWitness,
}

impl S8ExecutionReadmissionWitness {
    pub(crate) const fn new(
        basis: S8AccessLoweringBasis,
        planned: S8AccessPathCounterSnapshot,
        coverage: S8LayoutCoverageWitness,
    ) -> Self {
        Self {
            basis,
            planned,
            coverage,
        }
    }

    pub const fn basis(self) -> S8AccessLoweringBasis {
        self.basis
    }

    pub const fn planned(self) -> S8AccessPathCounterSnapshot {
        self.planned
    }

    pub const fn coverage(self) -> S8LayoutCoverageWitness {
        self.coverage
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8ExecutionRebindWitness {
    basis: S8AccessLoweringBasis,
    coverage: S8LayoutCoverageWitness,
}

impl S8ExecutionRebindWitness {
    pub(crate) const fn new(
        basis: S8AccessLoweringBasis,
        coverage: S8LayoutCoverageWitness,
    ) -> Self {
        Self { basis, coverage }
    }

    pub const fn basis(self) -> S8AccessLoweringBasis {
        self.basis
    }

    pub const fn coverage(self) -> S8LayoutCoverageWitness {
        self.coverage
    }
}
