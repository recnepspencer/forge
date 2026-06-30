use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::evidence_ledger::{WorkloadEvidenceRow, WorkloadEvidenceStage};

use super::link::WorkloadEvidenceStageLink;

pub(crate) fn stage_link_identity(
    stage: WorkloadEvidenceStage,
    row: &WorkloadEvidenceRow,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "workload-evidence-stage-link".to_string(),
            format!("stage:{}", stage.human_name()),
            format!("identity:{}", row.evidence_identity()),
            format!(
                "counter-total:{}",
                row.counters().total_receipt_backed_counters()
            ),
        ],
    )
}

pub(crate) fn stage_link_set_identity(
    stage_index_identity: &str,
    links: &[WorkloadEvidenceStageLink],
) -> String {
    let _ = stage_index_identity;
    let mut parts = vec![
        "workload-evidence-stage-link-set".to_string(),
        format!("links:{}", links.len()),
    ];
    parts.extend(links.iter().map(|link| {
        format!(
            "stage:{}|evidence:{}|link:{}",
            link.stage().human_name(),
            link.evidence_identity(),
            link.link_identity()
        )
    }));
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
