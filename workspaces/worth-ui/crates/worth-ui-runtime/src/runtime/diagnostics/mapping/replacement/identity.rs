use super::digest::{fold, phase_denial_diagnostic, stable_text_digest};
use crate::runtime::diagnostics::{
    WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCode, WorthUiRuntimeDiagnosticFamily,
};
use crate::runtime::WorthUiIdentityMatchDenial;

pub(crate) fn diagnostic_for_identity_matching(
    denial: &WorthUiIdentityMatchDenial,
) -> WorthUiRuntimeDiagnostic {
    phase_denial_diagnostic(
        WorthUiRuntimeDiagnosticFamily::IdentityMatching,
        WorthUiRuntimeDiagnosticCode::IdentityMatchingDenied,
        identity_matching_digest(denial),
    )
}

fn identity_matching_digest(denial: &WorthUiIdentityMatchDenial) -> u64 {
    match denial {
        WorthUiIdentityMatchDenial::NarrowingActiveBasisMismatch {
            narrowing_active_artifact_digest,
            active_artifact_digest,
            counters,
        } => fold(
            fold(
                fold(0xA4_00_00_01, *narrowing_active_artifact_digest),
                *active_artifact_digest,
            ),
            counters.stable_seed_lookups() as u64,
        ),
        WorthUiIdentityMatchDenial::NarrowingCandidateMismatch {
            narrowing_candidate_artifact_digest,
            admitted_candidate_artifact_digest,
            counters,
        } => fold(
            fold(
                fold(0xA4_00_00_02, *narrowing_candidate_artifact_digest),
                *admitted_candidate_artifact_digest,
            ),
            counters.stable_seed_lookups() as u64,
        ),
        WorthUiIdentityMatchDenial::AdmissionReceiptChanged { counters } => {
            fold(0xA4_00_00_03, counters.stable_seed_lookups() as u64)
        }
        WorthUiIdentityMatchDenial::DuplicateActiveIdentity {
            identity_basis,
            counters,
            ..
        } => fold(
            fold(0xA4_00_00_04, stable_text_digest(identity_basis)),
            counters.duplicate_active_identity_count() as u64,
        ),
        WorthUiIdentityMatchDenial::DuplicateCandidateIdentity {
            identity_basis,
            counters,
            ..
        } => fold(
            fold(0xA4_00_00_05, stable_text_digest(identity_basis)),
            counters.duplicate_candidate_identity_count() as u64,
        ),
        WorthUiIdentityMatchDenial::ActiveIdentityKindMismatch {
            identity_basis,
            counters,
            ..
        } => fold(
            fold(0xA4_00_00_06, stable_text_digest(identity_basis)),
            counters.identity_kind_mismatch_count() as u64,
        ),
        WorthUiIdentityMatchDenial::CandidateIdentityKindMismatch {
            identity_basis,
            counters,
            ..
        } => fold(
            fold(0xA4_00_00_07, stable_text_digest(identity_basis)),
            counters.identity_kind_mismatch_count() as u64,
        ),
        WorthUiIdentityMatchDenial::IdentityKindMismatch {
            identity_basis,
            counters,
            ..
        } => fold(
            fold(0xA4_00_00_08, stable_text_digest(identity_basis)),
            counters.identity_kind_mismatch_count() as u64,
        ),
        WorthUiIdentityMatchDenial::PositionOnlyRepeatedTemplateIdentity {
            identity_basis,
            counters,
            ..
        } => fold(
            fold(0xA4_00_00_09, stable_text_digest(identity_basis)),
            counters.unmatched_candidate_count() as u64,
        ),
    }
}
