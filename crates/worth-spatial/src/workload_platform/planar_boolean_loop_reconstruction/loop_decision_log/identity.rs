use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::row::PlanarBooleanLoopDecisionRow;

pub(crate) fn decision_identity(
    phase: &str,
    affected_artifact: &str,
    affected_artifact_identity: &str,
    detail: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-loop-decision-row".to_string(),
            format!("phase:{phase}"),
            format!("artifact:{affected_artifact}"),
            format!("artifact-identity:{affected_artifact_identity}"),
            format!("detail:{detail}"),
        ],
    )
}

pub(crate) fn decision_log_identity(
    request_identity: &str,
    split_ledger_receipt_identity: &str,
    rows: &[PlanarBooleanLoopDecisionRow],
) -> String {
    let mut parts = vec![
        "planar-boolean-loop-decision-log".to_string(),
        format!("request:{request_identity}"),
        format!("split-ledger:{split_ledger_receipt_identity}"),
    ];
    parts.extend(
        rows.iter()
            .map(|row| format!("decision:{}", row.decision_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
