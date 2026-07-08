use crate::runtime::active::WorthUiActiveArtifact;
use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiIdentityMatchCounters, WorthUiIdentityMatchDenial,
    WorthUiRuntimeImpactNarrowing,
};

pub(super) fn reject_mismatched_active_basis(
    active_artifact: &WorthUiActiveArtifact,
    narrowing: &WorthUiRuntimeImpactNarrowing,
    counters: WorthUiIdentityMatchCounters,
) -> Result<(), WorthUiIdentityMatchDenial> {
    let active_artifact_digest = active_artifact.digest().raw();
    if narrowing.active_artifact_digest() == active_artifact_digest {
        Ok(())
    } else {
        Err(WorthUiIdentityMatchDenial::NarrowingActiveBasisMismatch {
            narrowing_active_artifact_digest: narrowing.active_artifact_digest(),
            active_artifact_digest,
            counters,
        })
    }
}

pub(super) fn reject_mismatched_candidate(
    narrowing: &WorthUiRuntimeImpactNarrowing,
    admitted: &WorthUiAdmittedReplacementCandidate,
    counters: WorthUiIdentityMatchCounters,
) -> Result<(), WorthUiIdentityMatchDenial> {
    let admitted_candidate_artifact_digest = admitted.artifact_bundle().artifact_digest().raw();
    if narrowing.candidate_artifact_digest() == admitted_candidate_artifact_digest {
        Ok(())
    } else {
        Err(WorthUiIdentityMatchDenial::NarrowingCandidateMismatch {
            narrowing_candidate_artifact_digest: narrowing.candidate_artifact_digest(),
            admitted_candidate_artifact_digest,
            counters,
        })
    }
}

pub(super) fn reject_changed_admission_receipts(
    admitted: &WorthUiAdmittedReplacementCandidate,
    counters: WorthUiIdentityMatchCounters,
) -> Result<(), WorthUiIdentityMatchDenial> {
    admitted
        .verify_receipts_unchanged()
        .map_err(|_| WorthUiIdentityMatchDenial::AdmissionReceiptChanged { counters })
}
