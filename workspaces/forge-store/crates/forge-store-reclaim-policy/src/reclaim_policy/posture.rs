use forge_store_physical_format::ReclaimedByteInterpretation;

use crate::{BackendMediaAssumptionSet, BackendTargetProfile, CapabilityEvidenceClass};

use super::ReclaimPolicyOperation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReclaimPolicyPosture {
    backend_profile: BackendTargetProfile,
    evidence_class: CapabilityEvidenceClass,
    media_assumptions: BackendMediaAssumptionSet,
    operation: ReclaimPolicyOperation,
    interpretation: ReclaimedByteInterpretation,
}

impl ReclaimPolicyPosture {
    pub(crate) const fn admitted(
        backend_profile: BackendTargetProfile,
        evidence_class: CapabilityEvidenceClass,
        media_assumptions: BackendMediaAssumptionSet,
        operation: ReclaimPolicyOperation,
        interpretation: ReclaimedByteInterpretation,
    ) -> Self {
        Self {
            backend_profile,
            evidence_class,
            media_assumptions,
            operation,
            interpretation,
        }
    }

    pub const fn backend_profile(self) -> BackendTargetProfile {
        self.backend_profile
    }

    pub const fn evidence_class(self) -> CapabilityEvidenceClass {
        self.evidence_class
    }

    pub const fn media_assumptions(self) -> BackendMediaAssumptionSet {
        self.media_assumptions
    }

    pub const fn operation(self) -> ReclaimPolicyOperation {
        self.operation
    }

    pub const fn interpretation(self) -> ReclaimedByteInterpretation {
        self.interpretation
    }
}
