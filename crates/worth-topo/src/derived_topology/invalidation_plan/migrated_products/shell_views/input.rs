use serde::Serialize;

use super::{ShellViewMigrationError, ShellViewReadStageCounters, ShellViewReadStageReceipt};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;
use crate::projection::read_views::TopologyShellBoundaryNeighborhoodView;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ShellViewBoundarySourceRow {
    touched_shell_identity: String,
    touched_source_identity: String,
    source_half_edge_identity: String,
    source_edge_identity: String,
    radial_target_half_edge_identity: String,
    current_target_edge_identity: String,
    source_radial_next_relation_identity: String,
    ring_half_edge_count: usize,
    boundary_half_edge: bool,
    non_manifold_edge: bool,
    row_digest: String,
}

impl ShellViewBoundarySourceRow {
    pub fn new(
        touched_shell_identity: impl Into<String>,
        touched_source_identity: impl Into<String>,
        source_half_edge_identity: impl Into<String>,
        source_edge_identity: impl Into<String>,
        radial_target_half_edge_identity: impl Into<String>,
        current_target_edge_identity: impl Into<String>,
        source_radial_next_relation_identity: impl Into<String>,
        ring_half_edge_count: usize,
        boundary_half_edge: bool,
        non_manifold_edge: bool,
    ) -> Self {
        let touched_shell_identity = touched_shell_identity.into();
        let touched_source_identity = touched_source_identity.into();
        let source_half_edge_identity = source_half_edge_identity.into();
        let source_edge_identity = source_edge_identity.into();
        let radial_target_half_edge_identity = radial_target_half_edge_identity.into();
        let current_target_edge_identity = current_target_edge_identity.into();
        let source_radial_next_relation_identity = source_radial_next_relation_identity.into();
        let row_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:shell-view-boundary-source-row:v1".to_string(),
            format!("touched-shell:{touched_shell_identity}"),
            format!("touched-source:{touched_source_identity}"),
            format!("source-half-edge:{source_half_edge_identity}"),
            format!("source-edge:{source_edge_identity}"),
            format!("radial-target:{radial_target_half_edge_identity}"),
            format!("current-target-edge:{current_target_edge_identity}"),
            format!("radial-relation:{source_radial_next_relation_identity}"),
            format!("ring-half-edges:{ring_half_edge_count}"),
            format!("boundary-half-edge:{boundary_half_edge}"),
            format!("non-manifold-edge:{non_manifold_edge}"),
        ]);
        Self {
            touched_shell_identity,
            touched_source_identity,
            source_half_edge_identity,
            source_edge_identity,
            radial_target_half_edge_identity,
            current_target_edge_identity,
            source_radial_next_relation_identity,
            ring_half_edge_count,
            boundary_half_edge,
            non_manifold_edge,
            row_digest,
        }
    }

    pub fn touched_shell_identity(&self) -> &str {
        &self.touched_shell_identity
    }

    pub fn touched_source_identity(&self) -> &str {
        &self.touched_source_identity
    }

    pub fn source_half_edge_identity(&self) -> &str {
        &self.source_half_edge_identity
    }

    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }

    pub fn radial_target_half_edge_identity(&self) -> &str {
        &self.radial_target_half_edge_identity
    }

    pub fn current_target_edge_identity(&self) -> &str {
        &self.current_target_edge_identity
    }

    pub fn source_radial_next_relation_identity(&self) -> &str {
        &self.source_radial_next_relation_identity
    }

    pub const fn ring_half_edge_count(&self) -> usize {
        self.ring_half_edge_count
    }

    pub const fn boundary_half_edge(&self) -> bool {
        self.boundary_half_edge
    }

    pub const fn non_manifold_edge(&self) -> bool {
        self.non_manifold_edge
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub(crate) fn from_query_shell_boundary_view(
        view: &TopologyShellBoundaryNeighborhoodView,
    ) -> Self {
        let ring_half_edge_count = view.same_edge_half_edge_identities().len();
        let touched_source_identity = view.source_half_edge_identity();
        Self::new(
            view.touched_shell_identity(),
            touched_source_identity,
            view.source_half_edge_identity(),
            view.source_edge_identity(),
            view.current_target_half_edge_identity(),
            view.current_target_edge_identity(),
            view.source_radial_next_relation_identity(),
            ring_half_edge_count,
            view.current_target_half_edge_identity() == view.source_half_edge_identity(),
            ring_half_edge_count > 2,
        )
    }
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellViewTouchedBoundaryRows {
    selected_rows: Vec<ShellViewBoundarySourceRow>,
    available_source_row_count: usize,
    source_rows_digest: String,
}

#[cfg(test)]
impl ShellViewTouchedBoundaryRows {
    pub fn from_selected_rows(rows: Vec<ShellViewBoundarySourceRow>) -> Self {
        let available_source_row_count = rows.len();
        Self::from_selected_rows_with_available_count(rows, available_source_row_count)
            .expect("selected rows are always a valid available row set")
    }

    pub fn from_selected_rows_with_available_count(
        rows: Vec<ShellViewBoundarySourceRow>,
        available_source_row_count: usize,
    ) -> Result<Self, ShellViewMigrationError> {
        if rows.len() > available_source_row_count {
            return Err(ShellViewMigrationError::SelectedRowsExceedAvailableRows);
        }
        let source_rows_digest = source_rows_digest(&rows, available_source_row_count);
        Ok(Self {
            selected_rows: rows,
            available_source_row_count,
            source_rows_digest,
        })
    }

    pub fn selected_rows(&self) -> &[ShellViewBoundarySourceRow] {
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
pub struct ShellViewExecutionInput {
    selected_rows: Vec<ShellViewBoundarySourceRow>,
    available_source_row_count: usize,
    selected_plan_digest: String,
    shell_view_selected_row_digest: String,
    source_rows_digest: String,
    read_stage_receipt_digest: String,
    touched_closure_shell_view_bound: usize,
    read_stage_counters: ShellViewReadStageCounters,
    input_digest: String,
}

impl ShellViewExecutionInput {
    pub fn from_selected_plan_and_read_stage(
        selected_plan: &DerivedInvalidationSelectedPlan,
        read_stage_receipt: ShellViewReadStageReceipt,
    ) -> Result<Self, ShellViewMigrationError> {
        let shell_view_selected_row_digest = selected_plan
            .selected_rows()
            .iter()
            .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::ShellViews)
            .map(|row| row.row_digest().to_string())
            .ok_or(ShellViewMigrationError::SelectedPlanMissingShellViewRow)?;
        if selected_plan.selected_plan_digest() != read_stage_receipt.selected_plan_digest()
            || selected_plan.touched_closure_digest() != read_stage_receipt.touched_closure_digest()
            || selected_plan.query_support_digest() != read_stage_receipt.query_support_digest()
            || selected_plan.legality_support_digest()
                != read_stage_receipt.legality_support_digest()
            || shell_view_selected_row_digest != read_stage_receipt.shell_view_selected_row_digest()
        {
            return Err(ShellViewMigrationError::ReadStageReceiptNotBoundToSelectedPlan);
        }

        let selected_plan_digest = selected_plan.selected_plan_digest().to_string();
        let source_rows_digest = read_stage_receipt.read_source_digest().to_string();
        let read_stage_receipt_digest = read_stage_receipt.receipt_digest().to_string();
        let input_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:shell-view-execution-input:v2".to_string(),
            format!("selected-plan:{selected_plan_digest}"),
            format!("shell-view-selected-row:{shell_view_selected_row_digest}"),
            format!("read-stage:{read_stage_receipt_digest}"),
            format!("source-rows:{source_rows_digest}"),
        ]);
        Ok(Self {
            selected_rows: read_stage_receipt.selected_rows().to_vec(),
            available_source_row_count: read_stage_receipt.available_source_row_count(),
            selected_plan_digest,
            shell_view_selected_row_digest,
            source_rows_digest,
            read_stage_receipt_digest,
            touched_closure_shell_view_bound: read_stage_receipt.touched_closure_shell_view_bound(),
            read_stage_counters: *read_stage_receipt.read_stage_counters(),
            input_digest,
        })
    }

    #[cfg(test)]
    pub fn from_selected_plan(
        selected_plan: &DerivedInvalidationSelectedPlan,
        rows: ShellViewTouchedBoundaryRows,
    ) -> Result<Self, ShellViewMigrationError> {
        let shell_view_selected_row_digest = selected_plan
            .selected_rows()
            .iter()
            .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::ShellViews)
            .map(|row| row.row_digest().to_string())
            .ok_or(ShellViewMigrationError::SelectedPlanMissingShellViewRow)?;
        let selected_plan_digest = selected_plan.selected_plan_digest().to_string();
        let source_rows_digest = rows.source_rows_digest().to_string();
        let selected_row_count = rows.selected_rows.len();
        let input_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:shell-view-execution-input:v1".to_string(),
            format!("selected-plan:{selected_plan_digest}"),
            format!("shell-view-selected-row:{shell_view_selected_row_digest}"),
            format!("source-rows:{source_rows_digest}"),
        ]);
        Ok(Self {
            selected_rows: rows.selected_rows,
            available_source_row_count: rows.available_source_row_count,
            selected_plan_digest,
            shell_view_selected_row_digest,
            source_rows_digest,
            read_stage_receipt_digest: "test-only-raw-shell-view-input".to_string(),
            touched_closure_shell_view_bound: selected_row_bound(selected_plan),
            read_stage_counters: ShellViewReadStageCounters::for_selected_rows(
                selected_row_count,
                rows.available_source_row_count,
            ),
            input_digest,
        })
    }

    pub fn selected_rows(&self) -> &[ShellViewBoundarySourceRow] {
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

    pub fn shell_view_selected_row_digest(&self) -> &str {
        &self.shell_view_selected_row_digest
    }

    pub fn source_rows_digest(&self) -> &str {
        &self.source_rows_digest
    }

    pub fn read_stage_receipt_digest(&self) -> &str {
        &self.read_stage_receipt_digest
    }

    pub const fn touched_closure_shell_view_bound(&self) -> usize {
        self.touched_closure_shell_view_bound
    }

    pub const fn read_stage_counters(&self) -> &ShellViewReadStageCounters {
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
    rows: &[ShellViewBoundarySourceRow],
    available_source_row_count: usize,
) -> String {
    let mut parts = vec![
        "worth-topo:shell-view-touched-source-rows:v1".to_string(),
        format!("selected-rows:{}", rows.len()),
        format!("available-source-rows:{available_source_row_count}"),
    ];
    parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
    super::super::super::catalog::catalog_digest(parts)
}
