use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;

use super::counters::EvidenceLookupPlanSelectionCounters;
use super::plan_row::EvidenceLookupSelectedPlanRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupSelectedPlan {
    selected_plan_digest: String,
    catalog_digest: String,
    admitted_input_digest: String,
    spatial_touch_digest: String,
    stage_receipt_digest: String,
    stage: WorkloadEvidenceStage,
    rows: Vec<EvidenceLookupSelectedPlanRow>,
    counters: EvidenceLookupPlanSelectionCounters,
}

impl EvidenceLookupSelectedPlan {
    pub(crate) fn new(
        catalog_digest: String,
        admitted_input_digest: String,
        spatial_touch_digest: String,
        stage_receipt_digest: String,
        stage: WorkloadEvidenceStage,
        rows: Vec<EvidenceLookupSelectedPlanRow>,
        counters: EvidenceLookupPlanSelectionCounters,
    ) -> Self {
        let selected_plan_digest = selected_plan_digest(
            &catalog_digest,
            &admitted_input_digest,
            &spatial_touch_digest,
            &stage_receipt_digest,
            stage,
            &rows,
            &counters,
        );
        Self {
            selected_plan_digest,
            catalog_digest,
            admitted_input_digest,
            spatial_touch_digest,
            stage_receipt_digest,
            stage,
            rows,
            counters,
        }
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }

    pub fn admitted_input_digest(&self) -> &str {
        &self.admitted_input_digest
    }

    pub fn spatial_touch_digest(&self) -> &str {
        &self.spatial_touch_digest
    }

    pub fn stage_receipt_digest(&self) -> &str {
        &self.stage_receipt_digest
    }

    pub const fn stage(&self) -> WorkloadEvidenceStage {
        self.stage
    }

    pub fn rows(&self) -> &[EvidenceLookupSelectedPlanRow] {
        &self.rows
    }

    pub const fn counters(&self) -> &EvidenceLookupPlanSelectionCounters {
        &self.counters
    }

    pub const fn claims_lookup_execution(&self) -> bool {
        false
    }

    pub const fn claims_index_construction(&self) -> bool {
        false
    }

    pub const fn claims_query_descriptor_authority(&self) -> bool {
        false
    }
}

fn selected_plan_digest(
    catalog_digest: &str,
    admitted_input_digest: &str,
    spatial_touch_digest: &str,
    stage_receipt_digest: &str,
    stage: WorkloadEvidenceStage,
    rows: &[EvidenceLookupSelectedPlanRow],
    counters: &EvidenceLookupPlanSelectionCounters,
) -> String {
    let mut parts = vec![
        "worth-spatial:evidence-lookup-selected-plan:v1".to_string(),
        format!("catalog:{catalog_digest}"),
        format!("admitted-input:{admitted_input_digest}"),
        format!("spatial-touch:{spatial_touch_digest}"),
        format!("stage-receipt:{stage_receipt_digest}"),
        format!("stage:{}", stage.human_name()),
        format!("candidate-families:{}", counters.candidate_family_count()),
        format!("selected-families:{}", counters.selected_family_count()),
        format!("unaffected-families:{}", counters.unaffected_family_count()),
        format!(
            "membership-probes:{}",
            counters.selected_family_membership_probe_count()
        ),
        format!(
            "topology-support-consumed:{}",
            counters.topology_support_rows_consumed_count()
        ),
        format!(
            "query-support-consumed:{}",
            counters.query_support_rows_consumed_count()
        ),
        format!(
            "raw-evidence-scans:{}",
            counters.raw_evidence_row_scan_count()
        ),
        format!(
            "broad-receipt-scans:{}",
            counters.broad_receipt_scan_count()
        ),
        format!(
            "caller-owned-evidence-work:{}",
            counters.caller_owned_evidence_work_count()
        ),
    ];
    parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
