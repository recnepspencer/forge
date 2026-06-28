use forge_foundational::{
    admit_current_basis_boundary_bundle, foundational_boundary_current_basis_authority,
    BoundaryEpoch, BoundaryHandle, CanonicalDigestId, CanonicalizationRuleVersion,
};
use forge_proof::TransitionOutcome;

use super::super::denial::RecoveryEvidenceDenial;
use super::super::executed_evidence_source::StoreRecoveryEvidenceAuthority;
use super::foundational_bundle::{
    FoundationalRecoveryEvidenceBundle, MaterializedFoundationalRecoveryEvidenceBundle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentBasisRecoveryEvidencePosture {
    Current(StoreRecoveryEvidenceAuthority),
    RawDigest(CanonicalDigestId),
    BoundaryBridgedStale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryCurrentBasisEvidence {
    handle: BoundaryHandle,
    epoch: BoundaryEpoch,
    admitted_foundational_bundle: bool,
}

impl RecoveryCurrentBasisEvidence {
    pub fn admit(
        posture: CurrentBasisRecoveryEvidencePosture,
    ) -> Result<Self, RecoveryEvidenceDenial> {
        match posture {
            CurrentBasisRecoveryEvidencePosture::Current(authority) => Ok(Self {
                handle: authority.handle(),
                epoch: authority.epoch(),
                admitted_foundational_bundle: false,
            }),
            CurrentBasisRecoveryEvidencePosture::RawDigest(_) => {
                Err(RecoveryEvidenceDenial::RawDigestCannotSatisfyCurrentBasis)
            }
            CurrentBasisRecoveryEvidencePosture::BoundaryBridgedStale => {
                Err(RecoveryEvidenceDenial::BoundaryBridgedStaleFormRequiresReadmission)
            }
        }
    }

    pub fn from_foundational_bundle(
        bundle: &FoundationalRecoveryEvidenceBundle,
    ) -> Result<Self, RecoveryEvidenceDenial> {
        Self::from_materialized_bundle(
            bundle.materialized(),
            bundle.receipt().handle(),
            bundle.receipt().epoch(),
        )
    }

    pub(crate) fn from_materialized_bundle(
        materialized: &MaterializedFoundationalRecoveryEvidenceBundle,
        handle: BoundaryHandle,
        epoch: BoundaryEpoch,
    ) -> Result<Self, RecoveryEvidenceDenial> {
        match admit_current_basis_boundary_bundle(
            CanonicalizationRuleVersion::new("store.s4.recovery.current-basis")
                .expect("static canonicalization version"),
            materialized.clone(),
            foundational_boundary_current_basis_authority(),
        ) {
            TransitionOutcome::Success(admitted) => Ok(Self {
                handle,
                epoch,
                admitted_foundational_bundle: admitted.bundle().receipt().is_some(),
            }),
            _ => Err(RecoveryEvidenceDenial::CurrentBasisAdmissionDenied),
        }
    }

    pub const fn handle(&self) -> BoundaryHandle {
        self.handle
    }

    pub const fn epoch(&self) -> BoundaryEpoch {
        self.epoch
    }

    pub const fn admitted_foundational_bundle(&self) -> bool {
        self.admitted_foundational_bundle
    }
}
