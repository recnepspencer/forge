use std::collections::BTreeSet;

use super::super::inventory::CausalEvidenceFamily;
use super::super::materialization::{CausalInspectionArtifactKind, QueryCausalInspectionArtifact};
use super::error::{CausalInspectionCertificationError, CausalInspectionCertificationErrorKind};
use super::failure_evidence::CausalInspectionCertificationFailureEvidence;
use super::matrix::CausalInspectionRepresentativeEvidence;
use super::matrix_kind::CausalInspectionRepresentativeKind;
use super::row_digest::CausalInspectionRepresentativeRowDigestSet;

pub(super) fn validate_missing_evidence_kind(
    kind: CausalInspectionRepresentativeKind,
    family: CausalEvidenceFamily,
) -> Result<(), CausalInspectionCertificationError> {
    let matches_kind = match kind {
        CausalInspectionRepresentativeKind::MissingSignalInvalidationEvidenceDenied => {
            family == CausalEvidenceFamily::SignalInvalidation
        }
        CausalInspectionRepresentativeKind::MissingSignalEvaluationEvidenceDenied => {
            family == CausalEvidenceFamily::SignalEvaluation
        }
        CausalInspectionRepresentativeKind::MissingBridgeRouteEvidenceDenied => {
            family == CausalEvidenceFamily::BridgeRoute
        }
        _ => false,
    };
    if matches_kind {
        return Ok(());
    }
    Err(CausalInspectionCertificationError::new(
        CausalInspectionCertificationErrorKind::RepresentativeMatrixMismatch,
        "missing-evidence representative kind must match the missing evidence family",
        &[
            format!("kind:{}", kind.as_str()),
            format!("family:{}", family.as_str()),
        ],
    ))
}

pub(super) fn validate_failure_kind(
    kind: CausalInspectionRepresentativeKind,
) -> Result<(), CausalInspectionCertificationError> {
    if matches!(
        kind,
        CausalInspectionRepresentativeKind::RelationalAuthorityMismatchDenied
            | CausalInspectionRepresentativeKind::RedactionPolicyOverclaimDenied
            | CausalInspectionRepresentativeKind::UnsupportedExplanationFamilyDenied
            | CausalInspectionRepresentativeKind::DirectBridgeDiagnosticsDomainExplanationForbidden
            | CausalInspectionRepresentativeKind::DirectRelationalRuntimeDomainExplanationForbidden
            | CausalInspectionRepresentativeKind::DirectSignalGraphDomainExplanationForbidden
            | CausalInspectionRepresentativeKind::DurableCausalArchiveOverclaimForbidden
            | CausalInspectionRepresentativeKind::StoreBackedReplayReconstructionOverclaimForbidden
    ) {
        return Ok(());
    }
    Err(CausalInspectionCertificationError::new(
        CausalInspectionCertificationErrorKind::RepresentativeMatrixMismatch,
        "failure representative kind must be a named rejection or forbidden lane",
        &[format!("kind:{}", kind.as_str())],
    ))
}

pub(super) fn validate_failure_evidence_kind(
    kind: CausalInspectionRepresentativeKind,
    evidence: &CausalInspectionCertificationFailureEvidence,
) -> Result<(), CausalInspectionCertificationError> {
    if evidence.representative_kind() == kind {
        return Ok(());
    }
    Err(CausalInspectionCertificationError::new(
        CausalInspectionCertificationErrorKind::RepresentativeMatrixMismatch,
        "failure representative kind must match its typed certification failure evidence",
        &[
            format!("kind:{}", kind.as_str()),
            format!("evidence-kind:{}", evidence.kind().as_str()),
            format!(
                "evidence-representative:{}",
                evidence.representative_kind().as_str()
            ),
        ],
    ))
}

pub(super) fn validate_kind_matches_artifact(
    kind: CausalInspectionRepresentativeKind,
    artifact: &QueryCausalInspectionArtifact,
    row_digest_set: &CausalInspectionRepresentativeRowDigestSet,
) -> Result<(), CausalInspectionCertificationError> {
    let matches_artifact = match kind {
        CausalInspectionRepresentativeKind::BridgeRouteAndSignalEvidenceBindSameObservation => {
            artifact.is_admitted()
                && row_digest_set.bridge_route_digest().is_some()
                && row_digest_set.signal_invalidation_digest().is_some()
        }
        CausalInspectionRepresentativeKind::BridgeRecordsBindThroughExistingDiagnostics => {
            artifact.is_admitted()
                && row_digest_set
                    .bridge_source_materialization_digest()
                    .is_some()
                && row_digest_set.bridge_structural_digest().is_some()
                && row_digest_set.bridge_stream_digest().is_some()
                && row_digest_set.bridge_preview_digest().is_some()
                && row_digest_set.bridge_writeback_digest().is_some()
                && row_digest_set.bridge_replay_digest().is_some()
        }
        CausalInspectionRepresentativeKind::SignalForensicAvailabilityAndReplayCursor => {
            artifact.is_admitted()
                && row_digest_set
                    .signal_forensic_availability_digest()
                    .is_some()
                && row_digest_set.signal_replay_cursor_digest().is_some()
        }
        CausalInspectionRepresentativeKind::CausalInspectionScaleHonesty => {
            artifact.is_admitted() && has_stable_scale_counters(artifact)
        }
        _ => validate_basic_artifact_posture(kind, artifact),
    };
    if matches_artifact {
        return Ok(());
    }
    Err(CausalInspectionCertificationError::new(
        CausalInspectionCertificationErrorKind::RepresentativeMatrixMismatch,
        "representative matrix row kind must match the supplied Query artifact posture and evidence slots",
        &[
            format!("kind:{}", kind.as_str()),
            format!("artifact-kind:{}", artifact.kind().as_str()),
            format!("slots:{}", row_digest_set.populated_named_evidence_slot_count()),
        ],
    ))
}

pub(super) fn validate_required_representatives(
    representatives: &[CausalInspectionRepresentativeEvidence],
) -> Result<(), CausalInspectionCertificationError> {
    let observed = representatives
        .iter()
        .map(CausalInspectionRepresentativeEvidence::kind)
        .collect::<BTreeSet<_>>();
    let missing = CausalInspectionRepresentativeKind::required()
        .iter()
        .copied()
        .find(|kind| !observed.contains(kind));
    if let Some(kind) = missing {
        return Err(CausalInspectionCertificationError::new(
            CausalInspectionCertificationErrorKind::MissingRepresentativeMatrixRow,
            "causal certification requires every Phase 6 representative row",
            &[format!("missing-kind:{}", kind.as_str())],
        ));
    }
    if observed.len() != representatives.len() {
        return Err(CausalInspectionCertificationError::new(
            CausalInspectionCertificationErrorKind::RepresentativeMatrixMismatch,
            "causal certification representative rows must be unique by kind",
            &[format!("row-count:{}", representatives.len())],
        ));
    }
    Ok(())
}

fn validate_basic_artifact_posture(
    kind: CausalInspectionRepresentativeKind,
    artifact: &QueryCausalInspectionArtifact,
) -> bool {
    match kind {
        CausalInspectionRepresentativeKind::ChangedResult
        | CausalInspectionRepresentativeKind::SuppressedResult
        | CausalInspectionRepresentativeKind::BranchPreview
        | CausalInspectionRepresentativeKind::HistoricalReplay
        | CausalInspectionRepresentativeKind::WorthStyleQueryOnlyConsumer
        | CausalInspectionRepresentativeKind::ObservationAnchorBindsOneQueryReceipt
        | CausalInspectionRepresentativeKind::CausalRichnessDoesNotChangeQueryMeaning => {
            artifact.is_admitted()
        }
        CausalInspectionRepresentativeKind::AdvisoryRedactedCausalEnvelope
        | CausalInspectionRepresentativeKind::PolicyRedacted => {
            artifact.kind() == CausalInspectionArtifactKind::Advisory
        }
        CausalInspectionRepresentativeKind::QueryDeniedBeforeBridgeEnvelope => artifact.is_denied(),
        _ => false,
    }
}

fn has_stable_scale_counters(artifact: &QueryCausalInspectionArtifact) -> bool {
    let performance = artifact.performance();
    performance.anchor_derivation_count() == 1
        && performance.evidence_reference_resolution_count() == 1
        && performance.admission_count() == 1
        && performance.materialization_count() == 1
        && performance.bridge_unindexed_scan_count() == 0
}
