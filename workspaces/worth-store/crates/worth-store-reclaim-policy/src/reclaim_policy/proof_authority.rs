use crate::AdmittedBackendCapabilityWitness;
use worth_store_physical_format::ReclaimedByteInterpretation;

use super::{ReclaimLaterHandoffPolicy, ReclaimPolicyOperation, ReclaimPolicyPosture};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReclaimPolicyProofAuthority {
    backend: AdmittedBackendCapabilityWitness,
}

impl ReclaimPolicyProofAuthority {
    pub const fn for_admitted_backend(backend: &AdmittedBackendCapabilityWitness) -> Self {
        Self { backend: *backend }
    }

    pub fn trim_posture(
        self,
        interpretation: ReclaimedByteInterpretation,
    ) -> Option<ReclaimPolicyPosture> {
        self.backend
            .media_assumptions()
            .supports_trim_posture()
            .then(|| {
                ReclaimPolicyPosture::admitted(
                    self.backend.profile(),
                    self.backend.evidence_class(),
                    self.backend.media_assumptions(),
                    ReclaimPolicyOperation::Trim,
                    interpretation,
                )
            })
    }

    pub fn punch_hole_posture(
        self,
        interpretation: ReclaimedByteInterpretation,
    ) -> Option<ReclaimPolicyPosture> {
        self.backend
            .media_assumptions()
            .supports_punch_hole_posture()
            .then(|| {
                ReclaimPolicyPosture::admitted(
                    self.backend.profile(),
                    self.backend.evidence_class(),
                    self.backend.media_assumptions(),
                    ReclaimPolicyOperation::PunchHole,
                    interpretation,
                )
            })
    }

    pub fn sparse_posture(
        self,
        interpretation: ReclaimedByteInterpretation,
    ) -> Option<ReclaimPolicyPosture> {
        self.backend
            .media_assumptions()
            .supports_sparse_posture()
            .then(|| {
                ReclaimPolicyPosture::admitted(
                    self.backend.profile(),
                    self.backend.evidence_class(),
                    self.backend.media_assumptions(),
                    ReclaimPolicyOperation::SparseDeclare,
                    interpretation,
                )
            })
    }

    pub fn cold_tier_io_posture(
        self,
        interpretation: ReclaimedByteInterpretation,
    ) -> Option<ReclaimPolicyPosture> {
        self.backend
            .media_assumptions()
            .supports_cold_tier_io_posture()
            .then(|| {
                ReclaimPolicyPosture::admitted(
                    self.backend.profile(),
                    self.backend.evidence_class(),
                    self.backend.media_assumptions(),
                    ReclaimPolicyOperation::ColdTierMovementPosture,
                    interpretation,
                )
            })
    }

    pub const fn non_claim_later_handoff(self) -> ReclaimLaterHandoffPolicy {
        ReclaimLaterHandoffPolicy::non_claim()
    }

    pub const fn backend(self) -> AdmittedBackendCapabilityWitness {
        self.backend
    }
}
