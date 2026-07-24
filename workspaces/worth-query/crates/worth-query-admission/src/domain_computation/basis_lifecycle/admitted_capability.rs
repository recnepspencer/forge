use worth_foundational::facade::CanonicalBasisReadyArtifact;

use super::canonical_basis::prepare_admitted_basis;
use super::lanes::BasisOperationLane;
use super::proofs::{BasisEligibility, NormalizedBasisIntent};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedBasisCapability<L: BasisOperationLane> {
    normalized: NormalizedBasisIntent,
    lane: L,
    capability_digest: String,
    canonical_basis: CanonicalBasisReadyArtifact,
}

impl<L: BasisOperationLane> AdmittedBasisCapability<L> {
    pub(crate) fn new(eligibility: BasisEligibility<L>) -> Self {
        let capability_digest = eligibility.normalized.capability_digest::<L>();
        let canonical_basis = prepare_admitted_basis(&eligibility.normalized);
        Self {
            normalized: eligibility.normalized,
            lane: eligibility.lane,
            capability_digest,
            canonical_basis,
        }
    }

    pub fn capability_digest(&self) -> &str {
        &self.capability_digest
    }

    pub fn normalized(&self) -> &NormalizedBasisIntent {
        &self.normalized
    }

    pub fn canonical_basis(&self) -> &CanonicalBasisReadyArtifact {
        &self.canonical_basis
    }
}
