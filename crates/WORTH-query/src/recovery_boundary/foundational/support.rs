use worth_foundational::facade::{
    FoundationalBoundaryEvidenceSupportBasisDisclosure,
    FoundationalBoundaryEvidenceSupportRecoveryPosture,
    FoundationalBoundaryEvidenceSupportTruthKind,
};

use crate::recovery_boundary::{
    WorthQueryRecoveryBasisPosture, WorthQueryRecoveryFoundationalSupportContext,
};

pub(crate) fn support_context_for_stale_basis() -> WorthQueryRecoveryFoundationalSupportContext {
    WorthQueryRecoveryFoundationalSupportContext::new(
        FoundationalBoundaryEvidenceSupportTruthKind::StaleBasisDisclosure,
        FoundationalBoundaryEvidenceSupportBasisDisclosure::StaleBasis,
        Some(FoundationalBoundaryEvidenceSupportRecoveryPosture::ReplayReconstructed),
    )
}

pub(crate) fn support_context_for_basis_mismatch() -> WorthQueryRecoveryFoundationalSupportContext {
    WorthQueryRecoveryFoundationalSupportContext::new(
        FoundationalBoundaryEvidenceSupportTruthKind::DegradedRecoveryReport,
        FoundationalBoundaryEvidenceSupportBasisDisclosure::ReducedBasis,
        Some(FoundationalBoundaryEvidenceSupportRecoveryPosture::RebuildRequired),
    )
}

pub(crate) fn basis_posture_for_foundational_disclosure(
    disclosure: FoundationalBoundaryEvidenceSupportBasisDisclosure,
) -> WorthQueryRecoveryBasisPosture {
    match disclosure {
        FoundationalBoundaryEvidenceSupportBasisDisclosure::CompleteBasis => {
            WorthQueryRecoveryBasisPosture::CompleteBasis
        }
        FoundationalBoundaryEvidenceSupportBasisDisclosure::StaleBasis => {
            WorthQueryRecoveryBasisPosture::StaleBasis
        }
        FoundationalBoundaryEvidenceSupportBasisDisclosure::ReducedBasis => {
            WorthQueryRecoveryBasisPosture::ReducedBasis
        }
        FoundationalBoundaryEvidenceSupportBasisDisclosure::ReducedAndStaleBasis => {
            WorthQueryRecoveryBasisPosture::ReducedAndStaleBasis
        }
    }
}
