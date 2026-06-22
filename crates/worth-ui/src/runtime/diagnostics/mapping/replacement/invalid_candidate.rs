use super::digest::phase_denial_diagnostic;
use crate::runtime::diagnostics::{
    WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCode, WorthUiRuntimeDiagnosticFamily,
};
use crate::runtime::WorthUiReplacementCandidateDenial;

pub(crate) fn diagnostic_for_invalid_candidate(
    denial: WorthUiReplacementCandidateDenial,
) -> WorthUiRuntimeDiagnostic {
    phase_denial_diagnostic(
        WorthUiRuntimeDiagnosticFamily::Reload,
        WorthUiRuntimeDiagnosticCode::InvalidCandidateDenied,
        invalid_candidate_digest(denial),
    )
}

fn invalid_candidate_digest(denial: WorthUiReplacementCandidateDenial) -> u64 {
    match denial {
        WorthUiReplacementCandidateDenial::MissingArtifactDigest => 0xC0_00_00_01,
        WorthUiReplacementCandidateDenial::MissingDependencyMetadata => 0xC0_00_00_02,
        WorthUiReplacementCandidateDenial::MissingLoweringBasis => 0xC0_00_00_03,
        WorthUiReplacementCandidateDenial::DependencyMetadataArtifactDigestMismatch => {
            0xC0_00_00_04
        }
        WorthUiReplacementCandidateDenial::StaleDependencyMetadata => 0xC0_00_00_05,
    }
}
