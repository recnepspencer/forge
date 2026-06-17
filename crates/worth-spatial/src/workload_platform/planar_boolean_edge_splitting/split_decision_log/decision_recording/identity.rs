use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::affected_artifact::PlanarBooleanSplitAffectedArtifact;
use super::kind::PlanarBooleanSplitDecisionKind;
use super::phase::PlanarBooleanSplitDecisionPhase;
use super::row::PlanarBooleanSplitDecisionRow;

pub(super) fn query_declaration_identity(
    split_request_identity: &str,
    split_chain_validation_identity: &str,
    split_persistent_naming_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-split-decision-log-query-declaration".to_string(),
            format!("request:{split_request_identity}"),
            format!("validation:{split_chain_validation_identity}"),
            format!("naming:{split_persistent_naming_identity}"),
        ],
    )
}

pub(super) fn decision_identity(
    phase: PlanarBooleanSplitDecisionPhase,
    kind: PlanarBooleanSplitDecisionKind,
    artifact_kind: PlanarBooleanSplitAffectedArtifact,
    affected_artifact_identity: &str,
    upstream_receipt_identity: &str,
) -> String {
    decision_identity_with_detail(
        phase,
        kind,
        artifact_kind,
        affected_artifact_identity,
        upstream_receipt_identity,
        &[],
    )
}

pub(super) fn decision_identity_with_detail(
    phase: PlanarBooleanSplitDecisionPhase,
    kind: PlanarBooleanSplitDecisionKind,
    artifact_kind: PlanarBooleanSplitAffectedArtifact,
    affected_artifact_identity: &str,
    upstream_receipt_identity: &str,
    extra_identity_parts: &[String],
) -> String {
    let mut parts = vec![
        "planar-boolean-split-decision-row".to_string(),
        format!("phase:{}", phase.as_str()),
        format!("kind:{}", kind.as_str()),
        format!("artifact-kind:{}", artifact_kind.as_str()),
        format!("artifact:{affected_artifact_identity}"),
        format!("upstream:{upstream_receipt_identity}"),
    ];
    parts.extend(extra_identity_parts.iter().cloned());
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &parts,
    )
}

pub(super) fn receipt_identity(
    query_declaration_identity: &str,
    rows: &[PlanarBooleanSplitDecisionRow],
) -> String {
    let mut parts = vec![
        "planar-boolean-split-decision-log-receipt".to_string(),
        format!("query-declaration:{query_declaration_identity}"),
    ];
    parts.extend(
        rows.iter()
            .map(|row| format!("decision:{}", row.decision_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(super) fn diagnostic_report_identity(
    localization_identity: &str,
    decision_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-structured-edge-split-failure-report".to_string(),
            format!("localization:{localization_identity}"),
            format!("decision:{decision_identity}"),
        ],
    )
}
