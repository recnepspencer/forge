use forge_foundational::facade::{
    FoundationalBoundaryEvidenceSupportBasisDisclosure,
    FoundationalBoundaryEvidenceSupportRecoveryPosture,
    FoundationalBoundaryEvidenceSupportTruthKind,
};

use crate::recovery_boundary::{
    ForgeQueryRecoveryBasisPosture, ForgeQueryRecoveryFoundationalSupportContext,
};

pub(crate) fn support_context_for_stale_basis() -> ForgeQueryRecoveryFoundationalSupportContext {
    ForgeQueryRecoveryFoundationalSupportContext::new(
        FoundationalBoundaryEvidenceSupportTruthKind::StaleBasisDisclosure,
        FoundationalBoundaryEvidenceSupportBasisDisclosure::StaleBasis,
        Some(FoundationalBoundaryEvidenceSupportRecoveryPosture::ReplayReconstructed),
    )
}

pub(crate) fn support_context_for_basis_mismatch() -> ForgeQueryRecoveryFoundationalSupportContext {
    ForgeQueryRecoveryFoundationalSupportContext::new(
        FoundationalBoundaryEvidenceSupportTruthKind::DegradedRecoveryReport,
        FoundationalBoundaryEvidenceSupportBasisDisclosure::ReducedBasis,
        Some(FoundationalBoundaryEvidenceSupportRecoveryPosture::RebuildRequired),
    )
}

pub(crate) fn basis_posture_for_foundational_disclosure(
    disclosure: FoundationalBoundaryEvidenceSupportBasisDisclosure,
) -> ForgeQueryRecoveryBasisPosture {
    match disclosure {
        FoundationalBoundaryEvidenceSupportBasisDisclosure::CompleteBasis => {
            ForgeQueryRecoveryBasisPosture::CompleteBasis
        }
        FoundationalBoundaryEvidenceSupportBasisDisclosure::StaleBasis => {
            ForgeQueryRecoveryBasisPosture::StaleBasis
        }
        FoundationalBoundaryEvidenceSupportBasisDisclosure::ReducedBasis => {
            ForgeQueryRecoveryBasisPosture::ReducedBasis
        }
        FoundationalBoundaryEvidenceSupportBasisDisclosure::ReducedAndStaleBasis => {
            ForgeQueryRecoveryBasisPosture::ReducedAndStaleBasis
        }
    }
}
