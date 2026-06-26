use forge_relational::facade::identity::EntityId;
use serde::Serialize;

use super::{LoopCycleMigrationError, LoopCycleReadStageCounters, LoopCycleReadStageReceipt};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LoopCycleBoundarySourceRow {
    shell_id: EntityId,
    boundary_component_count: usize,
    boundary_half_edge_count: usize,
    row_digest: String,
}

impl LoopCycleBoundarySourceRow {
    pub fn new(
        shell_id: EntityId,
        boundary_component_count: usize,
        boundary_half_edge_count: usize,
    ) -> Self {
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:loop-cycle-boundary-source-row:v1".to_string(),
            format!("shell:{shell_id:?}"),
            format!("boundary-components:{boundary_component_count}"),
            format!("boundary-half-edges:{boundary_half_edge_count}"),
        ]);
        Self {
            shell_id,
            boundary_component_count,
            boundary_half_edge_count,
            row_digest,
        }
    }

    pub const fn shell_id(&self) -> EntityId {
        self.shell_id
    }

    pub const fn boundary_component_count(&self) -> usize {
        self.boundary_component_count
    }

    pub const fn boundary_half_edge_count(&self) -> usize {
        self.boundary_half_edge_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopCycleTouchedBoundaryRows {
    selected_rows: Vec<LoopCycleBoundarySourceRow>,
    available_source_row_count: usize,
    source_rows_digest: String,
}

#[cfg(test)]
impl LoopCycleTouchedBoundaryRows {
    pub fn from_selected_rows(rows: Vec<LoopCycleBoundarySourceRow>) -> Self {
        let available_source_row_count = rows.len();
        Self::from_selected_rows_with_available_count(rows, available_source_row_count)
            .expect("selected rows are always a valid available row set")
    }

    pub fn from_selected_rows_with_available_count(
        rows: Vec<LoopCycleBoundarySourceRow>,
        available_source_row_count: usize,
    ) -> Result<Self, LoopCycleMigrationError> {
        if rows.len() > available_source_row_count {
            return Err(LoopCycleMigrationError::SelectedRowsExceedAvailableRows);
        }
        let source_rows_digest = source_rows_digest(&rows, available_source_row_count);
        Ok(Self {
            selected_rows: rows,
            available_source_row_count,
            source_rows_digest,
        })
    }

    pub fn selected_rows(&self) -> &[LoopCycleBoundarySourceRow] {
        &self.selected_rows
    }

    pub const fn available_source_row_count(&self) -> usize {
        self.available_source_row_count
    }

    pub fn selected_row_count(&self) -> usize {
        self.selected_rows.len()
    }

    pub fn source_rows_digest(&self) -> &str {
        &self.source_rows_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopCycleExecutionInput {
    selected_rows: Vec<LoopCycleBoundarySourceRow>,
    available_source_row_count: usize,
    selected_plan_digest: String,
    loop_cycle_selected_row_digest: String,
    source_rows_digest: String,
    read_stage_receipt_digest: String,
    touched_closure_loop_cycle_bound: usize,
    read_stage_counters: LoopCycleReadStageCounters,
    input_digest: String,
}

impl LoopCycleExecutionInput {
    pub fn from_selected_plan_and_read_stage(
        selected_plan: &DerivedInvalidationSelectedPlan,
        read_stage_receipt: LoopCycleReadStageReceipt,
    ) -> Result<Self, LoopCycleMigrationError> {
        let loop_cycle_selected_row_digest = selected_plan
            .selected_rows()
            .iter()
            .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::LoopCycles)
            .map(|row| row.row_digest().to_string())
            .ok_or(LoopCycleMigrationError::SelectedPlanMissingLoopCycleRow)?;
        if selected_plan.selected_plan_digest() != read_stage_receipt.selected_plan_digest()
            || selected_plan.touched_closure_digest() != read_stage_receipt.touched_closure_digest()
            || selected_plan.query_support_digest() != read_stage_receipt.query_support_digest()
            || selected_plan.legality_support_digest()
                != read_stage_receipt.legality_support_digest()
            || loop_cycle_selected_row_digest != read_stage_receipt.loop_cycle_selected_row_digest()
        {
            return Err(LoopCycleMigrationError::ReadStageReceiptNotBoundToSelectedPlan);
        }

        let selected_plan_digest = selected_plan.selected_plan_digest().to_string();
        let source_rows_digest = read_stage_receipt.read_source_digest().to_string();
        let read_stage_receipt_digest = read_stage_receipt.receipt_digest().to_string();
        let input_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:loop-cycle-execution-input:v2".to_string(),
            format!("selected-plan:{selected_plan_digest}"),
            format!("loop-cycle-selected-row:{loop_cycle_selected_row_digest}"),
            format!("read-stage:{read_stage_receipt_digest}"),
            format!("source-rows:{source_rows_digest}"),
        ]);
        Ok(Self {
            selected_rows: read_stage_receipt.selected_rows().to_vec(),
            available_source_row_count: read_stage_receipt.available_source_row_count(),
            selected_plan_digest,
            loop_cycle_selected_row_digest,
            source_rows_digest,
            read_stage_receipt_digest,
            touched_closure_loop_cycle_bound: read_stage_receipt.touched_closure_loop_cycle_bound(),
            read_stage_counters: *read_stage_receipt.read_stage_counters(),
            input_digest,
        })
    }

    #[cfg(test)]
    pub fn from_selected_plan(
        selected_plan: &DerivedInvalidationSelectedPlan,
        rows: LoopCycleTouchedBoundaryRows,
    ) -> Result<Self, LoopCycleMigrationError> {
        let loop_cycle_selected_row_digest = selected_plan
            .selected_rows()
            .iter()
            .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::LoopCycles)
            .map(|row| row.row_digest().to_string())
            .ok_or(LoopCycleMigrationError::SelectedPlanMissingLoopCycleRow)?;
        let selected_plan_digest = selected_plan.selected_plan_digest().to_string();
        let source_rows_digest = rows.source_rows_digest().to_string();
        let selected_row_count = rows.selected_rows.len();
        let input_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:loop-cycle-execution-input:v1".to_string(),
            format!("selected-plan:{selected_plan_digest}"),
            format!("loop-cycle-selected-row:{loop_cycle_selected_row_digest}"),
            format!("source-rows:{source_rows_digest}"),
        ]);
        Ok(Self {
            selected_rows: rows.selected_rows,
            available_source_row_count: rows.available_source_row_count,
            selected_plan_digest,
            loop_cycle_selected_row_digest,
            source_rows_digest,
            read_stage_receipt_digest: "test-only-raw-loop-cycle-input".to_string(),
            touched_closure_loop_cycle_bound: selected_row_bound(selected_plan),
            read_stage_counters: LoopCycleReadStageCounters::for_selected_rows(
                selected_row_count,
                rows.available_source_row_count,
            ),
            input_digest,
        })
    }

    pub fn selected_rows(&self) -> &[LoopCycleBoundarySourceRow] {
        &self.selected_rows
    }

    pub const fn available_source_row_count(&self) -> usize {
        self.available_source_row_count
    }

    pub fn selected_row_count(&self) -> usize {
        self.selected_rows.len()
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn loop_cycle_selected_row_digest(&self) -> &str {
        &self.loop_cycle_selected_row_digest
    }

    pub fn source_rows_digest(&self) -> &str {
        &self.source_rows_digest
    }

    pub fn read_stage_receipt_digest(&self) -> &str {
        &self.read_stage_receipt_digest
    }

    pub const fn touched_closure_loop_cycle_bound(&self) -> usize {
        self.touched_closure_loop_cycle_bound
    }

    pub const fn read_stage_counters(&self) -> &LoopCycleReadStageCounters {
        &self.read_stage_counters
    }

    pub fn input_digest(&self) -> &str {
        &self.input_digest
    }
}

#[cfg(test)]
fn selected_row_bound(selected_plan: &DerivedInvalidationSelectedPlan) -> usize {
    let counters = selected_plan.counters();
    counters.touched_entity_count()
        + counters.touched_relation_count()
        + counters.touched_relation_kind_count()
        + counters.touched_aspect_count()
        + counters.touched_scope_count()
}

#[cfg(test)]
fn source_rows_digest(
    rows: &[LoopCycleBoundarySourceRow],
    available_source_row_count: usize,
) -> String {
    let mut parts = vec![
        "worth-topo:loop-cycle-touched-boundary-rows:v1".to_string(),
        format!("selected-rows:{}", rows.len()),
        format!("available-source-rows:{available_source_row_count}"),
    ];
    parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
    super::super::super::catalog::catalog_digest(parts)
}
