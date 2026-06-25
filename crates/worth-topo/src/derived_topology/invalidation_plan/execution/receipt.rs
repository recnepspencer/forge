use serde::Serialize;

use super::rows::{
    DerivedInvalidationDeniedProductExecutionRow, DerivedInvalidationExecutedProductRow,
    DerivedInvalidationResidueExecutionRow, DerivedInvalidationUnaffectedProductExecutionRow,
};
use super::{
    DerivedInvalidationExecutionCounters, DerivedInvalidationExecutionError,
    DerivedInvalidationExecutionErrorKind, DerivedInvalidationProductExecutor,
    PlannedDerivedInvalidationProductExecutor,
};
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationExecutionAdmission, DerivedInvalidationSelectedPlan,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationExecutionReceipt {
    phase_four_seed_digest: String,
    selected_plan_digest: String,
    touched_closure_digest: String,
    query_support_digest: String,
    legality_support_digest: String,
    executed_rows: Vec<DerivedInvalidationExecutedProductRow>,
    unaffected_rows: Vec<DerivedInvalidationUnaffectedProductExecutionRow>,
    denied_rows: Vec<DerivedInvalidationDeniedProductExecutionRow>,
    residue_rows: Vec<DerivedInvalidationResidueExecutionRow>,
    counters: DerivedInvalidationExecutionCounters,
    execution_receipt_digest: String,
}

impl DerivedInvalidationExecutionReceipt {
    pub fn execute_selected_plan(
        selected_plan: &DerivedInvalidationSelectedPlan,
    ) -> Result<Self, DerivedInvalidationExecutionError> {
        Self::execute_selected_plan_with_executor(
            selected_plan,
            &PlannedDerivedInvalidationProductExecutor,
        )
    }

    pub(crate) fn execute_selected_plan_with_executor(
        selected_plan: &DerivedInvalidationSelectedPlan,
        executor: &impl DerivedInvalidationProductExecutor,
    ) -> Result<Self, DerivedInvalidationExecutionError> {
        if selected_plan.execution_admission() == DerivedInvalidationExecutionAdmission::Denied
            && !selected_plan.selected_rows().is_empty()
        {
            return Err(DerivedInvalidationExecutionError::new(
                DerivedInvalidationExecutionErrorKind::DeniedPlanCarriedExecutableRows,
            ));
        }

        let executed_rows = if selected_plan.execution_admission().is_admitted() {
            execute_selected_rows(selected_plan, executor)?
        } else {
            Vec::new()
        };
        let unaffected_rows = selected_plan
            .unaffected_rows()
            .iter()
            .map(DerivedInvalidationUnaffectedProductExecutionRow::from_unaffected_row)
            .collect::<Vec<_>>();
        let denied_rows = selected_plan
            .denied_rows()
            .iter()
            .map(DerivedInvalidationDeniedProductExecutionRow::from_denial_row)
            .collect::<Vec<_>>();
        let residue_rows = selected_plan
            .residue_rows()
            .iter()
            .map(DerivedInvalidationResidueExecutionRow::from_residue_row)
            .collect::<Vec<_>>();
        let counters = DerivedInvalidationExecutionCounters::from_rows(
            &executed_rows,
            &unaffected_rows,
            &denied_rows,
            &residue_rows,
        );
        let execution_receipt_digest = execution_receipt_digest(
            selected_plan,
            &executed_rows,
            &unaffected_rows,
            &denied_rows,
            &residue_rows,
            &counters,
        );
        Ok(Self {
            phase_four_seed_digest: selected_plan.phase_four_seed().seed_digest().to_string(),
            selected_plan_digest: selected_plan.selected_plan_digest().to_string(),
            touched_closure_digest: selected_plan.touched_closure_digest().to_string(),
            query_support_digest: selected_plan.query_support_digest().to_string(),
            legality_support_digest: selected_plan.legality_support_digest().to_string(),
            executed_rows,
            unaffected_rows,
            denied_rows,
            residue_rows,
            counters,
            execution_receipt_digest,
        })
    }

    pub fn phase_four_seed_digest(&self) -> &str {
        &self.phase_four_seed_digest
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn touched_closure_digest(&self) -> &str {
        &self.touched_closure_digest
    }

    pub fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub fn legality_support_digest(&self) -> &str {
        &self.legality_support_digest
    }

    pub fn executed_rows(&self) -> &[DerivedInvalidationExecutedProductRow] {
        &self.executed_rows
    }

    pub fn unaffected_rows(&self) -> &[DerivedInvalidationUnaffectedProductExecutionRow] {
        &self.unaffected_rows
    }

    pub fn denied_rows(&self) -> &[DerivedInvalidationDeniedProductExecutionRow] {
        &self.denied_rows
    }

    pub fn residue_rows(&self) -> &[DerivedInvalidationResidueExecutionRow] {
        &self.residue_rows
    }

    pub const fn counters(&self) -> &DerivedInvalidationExecutionCounters {
        &self.counters
    }

    pub fn execution_receipt_digest(&self) -> &str {
        &self.execution_receipt_digest
    }
}

fn execute_selected_rows(
    selected_plan: &DerivedInvalidationSelectedPlan,
    executor: &impl DerivedInvalidationProductExecutor,
) -> Result<Vec<DerivedInvalidationExecutedProductRow>, DerivedInvalidationExecutionError> {
    selected_plan
        .selected_rows()
        .iter()
        .map(|row| {
            let report = executor.execute_selected_row(row)?;
            if report.source_selected_row_digest() != row.row_digest() {
                return Err(DerivedInvalidationExecutionError::new(
                    DerivedInvalidationExecutionErrorKind::ExecutionReportSourceRowMismatch,
                ));
            }
            Ok(DerivedInvalidationExecutedProductRow::from_selected_row_report(row, &report))
        })
        .collect()
}

fn execution_receipt_digest(
    selected_plan: &DerivedInvalidationSelectedPlan,
    executed_rows: &[DerivedInvalidationExecutedProductRow],
    unaffected_rows: &[DerivedInvalidationUnaffectedProductExecutionRow],
    denied_rows: &[DerivedInvalidationDeniedProductExecutionRow],
    residue_rows: &[DerivedInvalidationResidueExecutionRow],
    counters: &DerivedInvalidationExecutionCounters,
) -> String {
    let mut parts = vec![
        "worth-topo:derived-invalidation-execution-receipt:v1".to_string(),
        format!(
            "phase-four-seed:{}",
            selected_plan.phase_four_seed().seed_digest()
        ),
        format!("selected-plan:{}", selected_plan.selected_plan_digest()),
        format!("touched-closure:{}", selected_plan.touched_closure_digest()),
        format!("query-support:{}", selected_plan.query_support_digest()),
        format!(
            "legality-support:{}",
            selected_plan.legality_support_digest()
        ),
        format!("counters:{}", counters.counters_digest()),
    ];
    parts.extend(
        executed_rows
            .iter()
            .map(|row| format!("executed:{}", row.row_digest())),
    );
    parts.extend(
        unaffected_rows
            .iter()
            .map(|row| format!("unaffected:{}", row.row_digest())),
    );
    parts.extend(
        denied_rows
            .iter()
            .map(|row| format!("denied:{}", row.row_digest())),
    );
    parts.extend(
        residue_rows
            .iter()
            .map(|row| format!("residue:{}", row.row_digest())),
    );
    super::super::catalog::catalog_digest(parts)
}
