use forge_relational::facade::identity::EntityId;
use schema::facade::platform::authority::{WireInterpretationClass, WireInterpretationRecord};
use serde::Serialize;

use super::WireViewMigrationError;
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::migrated_products::wire_views::WireViewReadStageReceipt;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WireViewSourceRow {
    wire_id: EntityId,
    class: WireInterpretationClass,
    connected_component_count: usize,
    half_edge_count: usize,
    terminal_vertex_ids: Vec<EntityId>,
    branch_vertex_ids: Vec<EntityId>,
    row_digest: String,
}

impl WireViewSourceRow {
    pub fn from_interpretation(record: &WireInterpretationRecord) -> Self {
        Self::new(
            record.wire_id,
            record.class,
            record.connected_component_count,
            1,
            record.terminal_vertex_ids.clone(),
            record.branch_vertex_ids.clone(),
        )
    }

    pub(crate) fn new(
        wire_id: EntityId,
        class: WireInterpretationClass,
        connected_component_count: usize,
        half_edge_count: usize,
        terminal_vertex_ids: Vec<EntityId>,
        branch_vertex_ids: Vec<EntityId>,
    ) -> Self {
        let row_digest = wire_view_source_row_digest(
            wire_id,
            class,
            connected_component_count,
            half_edge_count,
            &terminal_vertex_ids,
            &branch_vertex_ids,
        );
        Self {
            wire_id,
            class,
            connected_component_count,
            half_edge_count,
            terminal_vertex_ids,
            branch_vertex_ids,
            row_digest,
        }
    }

    pub const fn wire_id(&self) -> EntityId {
        self.wire_id
    }

    pub const fn class(&self) -> WireInterpretationClass {
        self.class
    }

    pub const fn connected_component_count(&self) -> usize {
        self.connected_component_count
    }

    pub const fn half_edge_count(&self) -> usize {
        self.half_edge_count
    }

    pub fn terminal_vertex_ids(&self) -> &[EntityId] {
        &self.terminal_vertex_ids
    }

    pub fn branch_vertex_ids(&self) -> &[EntityId] {
        &self.branch_vertex_ids
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireViewExecutionInput {
    selected_rows: Vec<WireViewSourceRow>,
    available_source_row_count: usize,
    selected_plan_digest: String,
    wire_view_selected_row_digest: String,
    source_rows_digest: String,
    read_stage_receipt_digest: String,
    touched_closure_wire_view_bound: usize,
    read_stage_counters: super::WireViewReadStageCounters,
    input_digest: String,
}

impl WireViewExecutionInput {
    pub fn from_selected_plan_and_read_stage(
        selected_plan: &DerivedInvalidationSelectedPlan,
        read_stage_receipt: WireViewReadStageReceipt,
    ) -> Result<Self, WireViewMigrationError> {
        let wire_view_selected_row_digest = selected_plan
            .selected_rows()
            .iter()
            .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::WireViews)
            .map(|row| row.row_digest().to_string())
            .ok_or(WireViewMigrationError::SelectedPlanMissingWireViewRow)?;
        if selected_plan.selected_plan_digest() != read_stage_receipt.selected_plan_digest()
            || selected_plan.touched_closure_digest() != read_stage_receipt.touched_closure_digest()
            || selected_plan.query_support_digest() != read_stage_receipt.query_support_digest()
            || selected_plan.legality_support_digest()
                != read_stage_receipt.legality_support_digest()
            || wire_view_selected_row_digest != read_stage_receipt.wire_view_selected_row_digest()
        {
            return Err(WireViewMigrationError::ReadStageReceiptNotBoundToSelectedPlan);
        }

        let selected_plan_digest = selected_plan.selected_plan_digest().to_string();
        let source_rows_digest = read_stage_receipt.read_source_digest().to_string();
        let read_stage_receipt_digest = read_stage_receipt.receipt_digest().to_string();
        let input_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:wire-view-execution-input:v2".to_string(),
            format!("selected-plan:{selected_plan_digest}"),
            format!("wire-view-selected-row:{wire_view_selected_row_digest}"),
            format!("read-stage:{read_stage_receipt_digest}"),
            format!("source-rows:{source_rows_digest}"),
        ]);
        Ok(Self {
            selected_rows: read_stage_receipt.selected_rows().to_vec(),
            available_source_row_count: read_stage_receipt.available_source_row_count(),
            selected_plan_digest,
            wire_view_selected_row_digest,
            source_rows_digest,
            read_stage_receipt_digest,
            touched_closure_wire_view_bound: read_stage_receipt.touched_closure_wire_view_bound(),
            read_stage_counters: *read_stage_receipt.read_stage_counters(),
            input_digest,
        })
    }

    pub fn selected_rows(&self) -> &[WireViewSourceRow] {
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

    pub fn source_rows_digest(&self) -> &str {
        &self.source_rows_digest
    }

    pub fn read_stage_receipt_digest(&self) -> &str {
        &self.read_stage_receipt_digest
    }

    pub const fn touched_closure_wire_view_bound(&self) -> usize {
        self.touched_closure_wire_view_bound
    }

    pub const fn read_stage_counters(&self) -> &super::WireViewReadStageCounters {
        &self.read_stage_counters
    }

    pub fn input_digest(&self) -> &str {
        &self.input_digest
    }
}

fn wire_view_source_row_digest(
    wire_id: EntityId,
    class: WireInterpretationClass,
    connected_component_count: usize,
    half_edge_count: usize,
    terminal_vertex_ids: &[EntityId],
    branch_vertex_ids: &[EntityId],
) -> String {
    super::super::super::catalog::catalog_digest([
        "worth-topo:wire-view-source-row:v1".to_string(),
        format!("wire:{wire_id:?}"),
        format!("class:{class:?}"),
        format!("connected-components:{connected_component_count}"),
        format!("half-edges:{half_edge_count}"),
        format!("terminal-vertices:{terminal_vertex_ids:?}"),
        format!("branch-vertices:{branch_vertex_ids:?}"),
    ])
}
