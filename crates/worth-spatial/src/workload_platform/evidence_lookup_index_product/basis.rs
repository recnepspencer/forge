use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::evidence_ledger::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceRow, WorkloadEvidenceStage,
};
use crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupEvidenceClass;
use crate::workload_platform::evidence_lookup_plan_selection::{
    EvidenceLookupPlanRowOutcome, EvidenceLookupSelectedPlan,
};

#[cfg(test)]
use super::counters::EvidenceLookupIndexProductCounters;
use super::query_support::{
    selected_query_support_digest, selected_query_support_digests,
};
#[cfg(test)]
use super::query_support::query_support_row_count;
use super::topology_support::{
    selected_topology_support_digest, selected_topology_support_digests,
};
#[cfg(test)]
use super::topology_support::topology_receipt_ref_count;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EvidenceLookupLedgerBasis {
    basis_digest: String,
    selected_plan_digest: String,
    spatial_touch_digest: String,
    stage_receipt_digest: String,
    topology_support_digest: String,
    query_support_digest: String,
    rows: Vec<WorkloadEvidenceRow>,
    selected_scope_row_limit: usize,
    total_ledger_row_count: usize,
}

impl EvidenceLookupLedgerBasis {
    pub(crate) fn from_selected_plan(
        selected_plan: &EvidenceLookupSelectedPlan,
        ledger: &CompleteWorkloadEvidenceLedger,
    ) -> Self {
        let required_stages = required_stages(selected_plan);
        let mut rows = required_stages
            .iter()
            .filter_map(|stage| ledger.row_for_stage(*stage).cloned())
            .collect::<Vec<_>>();
        rows.sort_by_key(|row| row.stage().index_slot());
        let selected_scope_row_limit = rows.len();
        let total_ledger_row_count = ledger.counters().rows();
        let topology_support_digest = selected_topology_support_digest(selected_plan.rows());
        let query_support_digest = selected_query_support_digest(selected_plan.rows());
        let basis_digest = basis_digest(selected_plan, &rows, selected_scope_row_limit);
        Self {
            basis_digest,
            selected_plan_digest: selected_plan.selected_plan_digest().to_string(),
            spatial_touch_digest: selected_plan.spatial_touch_digest().to_string(),
            stage_receipt_digest: selected_plan.stage_receipt_digest().to_string(),
            topology_support_digest,
            query_support_digest,
            rows,
            selected_scope_row_limit,
            total_ledger_row_count,
        }
    }

    #[cfg(test)]
    pub(crate) fn from_complete_ledger_scope(
        selected_plan: &EvidenceLookupSelectedPlan,
        ledger: &CompleteWorkloadEvidenceLedger,
    ) -> Self {
        let rows = ledger.rows().to_vec();
        let selected_scope_row_limit = required_stages(selected_plan).len();
        let total_ledger_row_count = ledger.counters().rows();
        let topology_support_digest = selected_topology_support_digest(selected_plan.rows());
        let query_support_digest = selected_query_support_digest(selected_plan.rows());
        let basis_digest = basis_digest(selected_plan, &rows, selected_scope_row_limit);
        Self {
            basis_digest,
            selected_plan_digest: selected_plan.selected_plan_digest().to_string(),
            spatial_touch_digest: selected_plan.spatial_touch_digest().to_string(),
            stage_receipt_digest: selected_plan.stage_receipt_digest().to_string(),
            topology_support_digest,
            query_support_digest,
            rows,
            selected_scope_row_limit,
            total_ledger_row_count,
        }
    }

    pub(crate) fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    #[cfg(test)]
    pub(crate) fn rows(&self) -> &[WorkloadEvidenceRow] {
        &self.rows
    }

    pub(crate) fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub(crate) fn spatial_touch_digest(&self) -> &str {
        &self.spatial_touch_digest
    }

    pub(crate) fn stage_receipt_digest(&self) -> &str {
        &self.stage_receipt_digest
    }

    pub(crate) fn topology_support_digest(&self) -> &str {
        &self.topology_support_digest
    }

    pub(crate) fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub(crate) fn exceeds_selected_scope(&self) -> bool {
        self.rows.len() > self.selected_scope_row_limit
    }

    #[cfg(test)]
    pub(crate) fn counters(
        &self,
        selected_plan: &EvidenceLookupSelectedPlan,
    ) -> EvidenceLookupIndexProductCounters {
        EvidenceLookupIndexProductCounters::new(
            self.rows.len(),
            self.total_ledger_row_count,
            indexed_family_count(selected_plan),
            topology_receipt_ref_count(selected_plan.rows()),
            query_support_row_count(selected_plan.rows()),
            resident_byte_count(&self.rows),
        )
    }
}

fn basis_digest(
    selected_plan: &EvidenceLookupSelectedPlan,
    rows: &[WorkloadEvidenceRow],
    selected_scope_row_limit: usize,
) -> String {
    let mut parts = vec![
        "worth-spatial:evidence-lookup-ledger-basis:v1".to_string(),
        format!("selected-plan:{}", selected_plan.selected_plan_digest()),
        format!("spatial-touch:{}", selected_plan.spatial_touch_digest()),
        format!("stage-receipt:{}", selected_plan.stage_receipt_digest()),
        format!("stage:{}", selected_plan.stage().human_name()),
        format!("scope-limit:{selected_scope_row_limit}"),
    ];
    parts.extend(
        selected_topology_support_digests(selected_plan.rows())
            .into_iter()
            .map(|digest| format!("topology:{digest}")),
    );
    parts.extend(
        selected_query_support_digests(selected_plan.rows())
            .into_iter()
            .map(|digest| format!("query:{digest}")),
    );
    parts.extend(rows.iter().map(|row| {
        format!(
            "row:{}:{}",
            row.stage().human_name(),
            row.evidence_identity()
        )
    }));
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

fn required_stages(selected_plan: &EvidenceLookupSelectedPlan) -> Vec<WorkloadEvidenceStage> {
    let mut stages = Vec::new();
    for row in selected_plan.rows() {
        if row.outcome() != EvidenceLookupPlanRowOutcome::Selected {
            continue;
        }
        if row
            .evidence_classes()
            .classes()
            .contains(&EvidenceLookupEvidenceClass::SpatialTouchEvidence)
        {
            for stage in WorkloadEvidenceStage::AUTHORITY_STAGES {
                push_stage_once(&mut stages, stage);
            }
        }
        if row
            .evidence_classes()
            .classes()
            .contains(&EvidenceLookupEvidenceClass::BooleanStageReceipt)
        {
            push_stage_once(&mut stages, selected_plan.stage());
        }
    }
    stages
}

#[cfg(test)]
fn indexed_family_count(selected_plan: &EvidenceLookupSelectedPlan) -> usize {
    selected_plan
        .rows()
        .iter()
        .filter(|row| {
            row.strategy()
                .is_some_and(|strategy| strategy.is_indexed_lookup_plan())
        })
        .count()
}

#[cfg(test)]
fn resident_byte_count(rows: &[WorkloadEvidenceRow]) -> usize {
    rows.iter()
        .map(|row| {
            std::mem::size_of::<WorkloadEvidenceRow>()
                + row.evidence_identity().len()
                + row
                    .upstream_stage_binding()
                    .map_or(0, |binding| binding.upstream_evidence_identity().len())
        })
        .sum()
}

fn push_stage_once(stages: &mut Vec<WorkloadEvidenceStage>, stage: WorkloadEvidenceStage) {
    if !stages.contains(&stage) {
        stages.push(stage);
    }
}
