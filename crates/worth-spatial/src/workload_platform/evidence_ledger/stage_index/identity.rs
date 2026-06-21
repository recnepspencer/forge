use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceBacking, WorkloadEvidenceRow, WorkloadEvidenceSupport,
};

use super::counters::WorkloadEvidenceStageIndexCounters;

pub(crate) fn stage_index_identity(
    rows: &[WorkloadEvidenceRow],
    counters: WorkloadEvidenceStageIndexCounters,
) -> String {
    let mut parts = vec![
        "workload-evidence-stage-index-product".to_string(),
        format!("rows:{}", counters.row_count()),
        format!("indexed-stages:{}", counters.indexed_stage_count()),
        format!("duplicate-stages:{}", counters.duplicate_stage_count()),
        format!("manual-rows:{}", counters.manual_row_count()),
        format!("unadmitted-rows:{}", counters.unadmitted_row_count()),
        format!("boolean-rows:{}", counters.boolean_row_count()),
        format!(
            "counterless-boolean-rows:{}",
            counters.counterless_boolean_row_count()
        ),
    ];
    parts.extend(rows.iter().map(|row| {
        format!(
            "stage:{}|identity:{}|backing:{}|support:{}|counter-total:{}|upstream:{}",
            row.stage().human_name(),
            row.evidence_identity(),
            backing_key(row.backing()),
            support_key(row.support()),
            row.counters().total_receipt_backed_counters(),
            upstream_binding_key(row)
        )
    }));
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

fn upstream_binding_key(row: &WorkloadEvidenceRow) -> String {
    row.upstream_stage_binding().map_or_else(
        || "none".to_string(),
        |binding| {
            format!(
                "{}:{}",
                binding.upstream_stage().human_name(),
                binding.upstream_evidence_identity()
            )
        },
    )
}

fn backing_key(backing: WorkloadEvidenceBacking) -> &'static str {
    match backing {
        WorkloadEvidenceBacking::Receipt => "receipt",
        WorkloadEvidenceBacking::CertificationOnly => "certification-only",
        WorkloadEvidenceBacking::Manual => "manual",
    }
}

fn support_key(support: WorkloadEvidenceSupport) -> &'static str {
    match support {
        WorkloadEvidenceSupport::Admitted => "admitted",
        WorkloadEvidenceSupport::Unsupported => "unsupported",
        WorkloadEvidenceSupport::Blocked => "blocked",
        WorkloadEvidenceSupport::Manual => "manual",
    }
}
