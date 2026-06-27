use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
use crate::workload_platform::evidence_lookup_execution::{
    EvidenceLookupExecutionOutcome, EvidenceLookupExecutionReceipt,
};
use crate::workload_platform::evidence_lookup_plan_selection::{
    EvidenceLookupPlanQuerySurface, EvidenceLookupPlanRowOutcome,
    EvidenceLookupPlanTopologyPostureState, EvidenceLookupSelectedPlan,
};

use super::advisory_reason::EvidenceLookupDiagnosticAdvisoryReason;
use super::counters::EvidenceLookupDiagnosticCounters;
use super::denial_reason::EvidenceLookupDiagnosticDenialReason;
use super::error::{EvidenceLookupDiagnosticsError, EvidenceLookupDiagnosticsErrorKind};
use super::row::{
    EvidenceLookupDiagnosticRow, EvidenceLookupDiagnosticRowParts, EvidenceLookupDiagnosticWitness,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupDiagnosticCloseout {
    rows: Vec<EvidenceLookupDiagnosticRow>,
    counters: EvidenceLookupDiagnosticCounters,
    diagnostic_digest: String,
}

impl EvidenceLookupDiagnosticCloseout {
    pub(crate) fn from_rows(
        rows: Vec<EvidenceLookupDiagnosticRow>,
        selected_plan: &EvidenceLookupSelectedPlan,
        execution_receipt: &EvidenceLookupExecutionReceipt,
    ) -> Result<Self, EvidenceLookupDiagnosticsError> {
        if rows.is_empty() {
            return Err(EvidenceLookupDiagnosticsError::new(
                EvidenceLookupDiagnosticsErrorKind::EmptyDiagnosticRows,
                "evidence lookup diagnostics require at least one row",
            ));
        }
        let counters = EvidenceLookupDiagnosticCounters::from_projection_proof(
            &rows,
            selected_plan,
            execution_receipt,
        );
        let diagnostic_digest = diagnostic_digest(&rows, &counters);
        Ok(Self {
            rows,
            counters,
            diagnostic_digest,
        })
    }

    pub fn rows(&self) -> &[EvidenceLookupDiagnosticRow] {
        &self.rows
    }

    pub const fn counters(&self) -> &EvidenceLookupDiagnosticCounters {
        &self.counters
    }

    pub fn diagnostic_digest(&self) -> &str {
        &self.diagnostic_digest
    }

    pub fn require_family_stage_witness(
        &self,
        family_identity: &str,
        stage: WorkloadEvidenceStage,
    ) -> Result<&EvidenceLookupDiagnosticRow, EvidenceLookupDiagnosticsError> {
        self.rows
            .iter()
            .find(|row| row.family_identity() == family_identity && row.stage() == stage)
            .ok_or_else(|| {
                EvidenceLookupDiagnosticsError::new(
                    EvidenceLookupDiagnosticsErrorKind::MissingFamilyStageWitness,
                    format!(
                        "missing diagnostic witness for family `{family_identity}` at stage `{}`",
                        stage.human_name()
                    ),
                )
            })
    }
}

pub fn derive_evidence_lookup_diagnostics(
    selected_plan: &EvidenceLookupSelectedPlan,
    execution_receipt: &EvidenceLookupExecutionReceipt,
) -> Result<EvidenceLookupDiagnosticCloseout, EvidenceLookupDiagnosticsError> {
    let rows = selected_plan
        .rows()
        .iter()
        .map(|row| {
            let query_surface_contract = execution_receipt
                .query_surface_contract_for_family(row.family_identity())
                .cloned()
                .or_else(|| row.query_surface_contract().cloned());
            EvidenceLookupDiagnosticRow::from_parts(EvidenceLookupDiagnosticRowParts {
                family_identity: row.family_identity().to_string(),
                family_declaration_digest: row.family_declaration_digest().to_string(),
                stage: selected_plan.stage(),
                spatial_touch_digest: selected_plan.spatial_touch_digest().to_string(),
                stage_receipt_digest: selected_plan.stage_receipt_digest().to_string(),
                selected_plan_digest: selected_plan.selected_plan_digest().to_string(),
                selected_plan_row_digest: row.row_digest().to_string(),
                execution_receipt_digest: execution_receipt.execution_receipt_digest().to_string(),
                evidence_classes: row.evidence_classes().clone(),
                topology_posture: row.topology_posture().clone(),
                query_posture: row.query_posture().clone(),
                query_surface_contract,
                witness: witness_for_row(selected_plan, execution_receipt, row),
            })
        })
        .collect::<Vec<_>>();
    EvidenceLookupDiagnosticCloseout::from_rows(rows, selected_plan, execution_receipt)
}

fn witness_for_row(
    selected_plan: &EvidenceLookupSelectedPlan,
    execution_receipt: &EvidenceLookupExecutionReceipt,
    row: &crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlanRow,
) -> EvidenceLookupDiagnosticWitness {
    if selected_plan.stage_receipt_digest() != execution_receipt.stage_receipt_digest() {
        return EvidenceLookupDiagnosticWitness::Denied(
            EvidenceLookupDiagnosticDenialReason::WrongStageReceiptIdentity,
        );
    }
    if selected_plan.spatial_touch_digest() != execution_receipt.spatial_touch_digest() {
        return EvidenceLookupDiagnosticWitness::Denied(
            EvidenceLookupDiagnosticDenialReason::WrongSpatialTouchDigest,
        );
    }
    if selected_plan.selected_plan_digest() != execution_receipt.selected_plan_digest() {
        return EvidenceLookupDiagnosticWitness::Denied(
            EvidenceLookupDiagnosticDenialReason::ProductSwapDetected,
        );
    }
    if matches!(
        row.topology_posture().state(),
        EvidenceLookupPlanTopologyPostureState::RequiredButMissing { .. }
    ) {
        return EvidenceLookupDiagnosticWitness::Denied(
            EvidenceLookupDiagnosticDenialReason::RequiredTopologySupport,
        );
    }
    if row.outcome() == EvidenceLookupPlanRowOutcome::RequiredQueryPosture
        || execution_receipt.outcome() == EvidenceLookupExecutionOutcome::RequiredQuerySupport
    {
        return EvidenceLookupDiagnosticWitness::Denied(
            EvidenceLookupDiagnosticDenialReason::RequiredQuerySupport,
        );
    }
    if execution_receipt.outcome()
        == EvidenceLookupExecutionOutcome::MissingProjectionConsumptionFact
        && row.query_posture().surface()
            == EvidenceLookupPlanQuerySurface::ProjectionConsumptionReceipt
    {
        return EvidenceLookupDiagnosticWitness::Denied(
            EvidenceLookupDiagnosticDenialReason::MissingProjectionConsumptionFact,
        );
    }
    if row.outcome() == EvidenceLookupPlanRowOutcome::Denied
        || execution_receipt.outcome() == EvidenceLookupExecutionOutcome::DeniedBeforeExecution
    {
        return EvidenceLookupDiagnosticWitness::Denied(
            EvidenceLookupDiagnosticDenialReason::ExecutionDeniedBeforeLookup,
        );
    }
    if row.outcome() == EvidenceLookupPlanRowOutcome::CappedResidue
        || execution_receipt.outcome() == EvidenceLookupExecutionOutcome::CappedResidue
    {
        return EvidenceLookupDiagnosticWitness::Advisory(
            EvidenceLookupDiagnosticAdvisoryReason::CappedResidue,
        );
    }
    if row.outcome() == EvidenceLookupPlanRowOutcome::Unaffected {
        return EvidenceLookupDiagnosticWitness::Advisory(
            EvidenceLookupDiagnosticAdvisoryReason::UnaffectedFamily,
        );
    }
    EvidenceLookupDiagnosticWitness::Success
}

fn diagnostic_digest(
    rows: &[EvidenceLookupDiagnosticRow],
    counters: &EvidenceLookupDiagnosticCounters,
) -> String {
    let mut parts = vec![
        "worth-spatial:evidence-lookup-diagnostic-closeout:v1".to_string(),
        format!("rows:{}", counters.row_count()),
        format!("success:{}", counters.success_row_count()),
        format!("advisory:{}", counters.advisory_row_count()),
        format!("denial:{}", counters.denial_row_count()),
        format!(
            "hidden-lookup-scans:{}",
            counters.hidden_lookup_scan_count()
        ),
        format!(
            "hidden-broad-receipt-scans:{}",
            counters.hidden_broad_receipt_scan_count()
        ),
    ];
    parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
