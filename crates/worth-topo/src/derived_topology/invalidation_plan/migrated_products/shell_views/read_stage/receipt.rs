use serde::Serialize;

use super::{ShellViewReadSource, ShellViewReadStageCounters};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::migrated_products::shell_views::{
    ShellViewBoundarySourceRow, ShellViewMigrationError,
};
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ShellViewReadStageReceipt {
    selected_plan_digest: String,
    touched_closure_digest: String,
    query_support_digest: String,
    legality_support_digest: String,
    shell_view_selected_row_digest: String,
    native_query_read_receipt_digest: String,
    selected_legality_receipt_digest: String,
    read_source_digest: String,
    touched_closure_shell_view_bound: usize,
    selected_source_row_count: usize,
    available_source_row_count: usize,
    read_stage_counters: ShellViewReadStageCounters,
    selected_rows: Vec<ShellViewBoundarySourceRow>,
    receipt_digest: String,
}

impl ShellViewReadStageReceipt {
    pub fn from_selected_plan_and_read_source(
        selected_plan: &DerivedInvalidationSelectedPlan,
        read_source: ShellViewReadSource,
    ) -> Result<Self, ShellViewMigrationError> {
        let selected_row = selected_plan
            .selected_rows()
            .iter()
            .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::ShellViews)
            .ok_or(ShellViewMigrationError::SelectedPlanMissingShellViewRow)?;
        let native_query_read_receipt_digest = selected_row
            .query_receipt_digest()
            .ok_or(ShellViewMigrationError::ReadStageReceiptMissingQueryReceipt)?
            .to_string();
        if !read_source
            .query_report_digests()
            .contains(&native_query_read_receipt_digest)
        {
            return Err(ShellViewMigrationError::ReadStageQueryReceiptNotBoundToSource);
        }
        let selected_legality_receipt_digest = selected_row
            .legality_receipt_digest()
            .ok_or(ShellViewMigrationError::ReadStageReceiptMissingLegalityReceipt)?
            .to_string();
        let touched_closure_shell_view_bound = touched_closure_shell_view_bound(selected_plan);
        let selected_source_row_count = read_source.selected_rows().len();
        if selected_source_row_count == 0 {
            return Err(ShellViewMigrationError::ReadStageTouchedClosureSelectedNoShellViewRows);
        }
        if selected_source_row_count > touched_closure_shell_view_bound {
            return Err(ShellViewMigrationError::ReadStageSelectedRowsExceedTouchedClosure);
        }

        let shell_view_selected_row_digest = selected_row.row_digest().to_string();
        let selected_rows = read_source.selected_rows().to_vec();
        let available_source_row_count = read_source.available_source_row_count();
        let read_stage_counters = *read_source.counters();
        let read_source_digest = read_source.read_source_digest().to_string();
        let mut receipt_parts = vec![
            "worth-topo:shell-view-read-stage-receipt:v1".to_string(),
            format!("selected-plan:{}", selected_plan.selected_plan_digest()),
            format!("touched-closure:{}", selected_plan.touched_closure_digest()),
            format!("query-support:{}", selected_plan.query_support_digest()),
            format!(
                "legality-support:{}",
                selected_plan.legality_support_digest()
            ),
            format!("selected-row:{shell_view_selected_row_digest}"),
            format!("native-query-read:{native_query_read_receipt_digest}"),
            format!("selected-legality:{selected_legality_receipt_digest}"),
            format!("read-source:{read_source_digest}"),
            format!("touched-bound:{touched_closure_shell_view_bound}"),
            format!("selected-source-rows:{selected_source_row_count}"),
            format!("available-source-rows:{available_source_row_count}"),
            format!(
                "read-stage-touched-anchors:{}",
                read_stage_counters.touched_anchor_count()
            ),
            format!(
                "read-stage-half-edge-lookups:{}",
                read_stage_counters.half_edge_lookup_count()
            ),
            format!(
                "read-stage-radial-relation-lookups:{}",
                read_stage_counters.radial_relation_lookup_count()
            ),
            format!(
                "read-stage-selected-radial-roots:{}",
                read_stage_counters.selected_radial_root_count()
            ),
            format!(
                "read-stage-touched-neighborhood-breadth:{}",
                read_stage_counters.touched_neighborhood_breadth_count()
            ),
            format!(
                "read-stage-unrelated-breadth:{}",
                read_stage_counters.unrelated_source_breadth_count()
            ),
            format!(
                "read-stage-whole-view-fallbacks:{}",
                read_stage_counters.whole_view_fallback_count()
            ),
        ];
        receipt_parts.extend(
            selected_rows
                .iter()
                .map(|row| format!("source-row:{}", row.row_digest())),
        );
        let receipt_digest = super::super::super::super::catalog::catalog_digest(receipt_parts);
        Ok(Self {
            selected_plan_digest: selected_plan.selected_plan_digest().to_string(),
            touched_closure_digest: selected_plan.touched_closure_digest().to_string(),
            query_support_digest: selected_plan.query_support_digest().to_string(),
            legality_support_digest: selected_plan.legality_support_digest().to_string(),
            shell_view_selected_row_digest,
            native_query_read_receipt_digest,
            selected_legality_receipt_digest,
            read_source_digest,
            touched_closure_shell_view_bound,
            selected_source_row_count,
            available_source_row_count,
            read_stage_counters,
            selected_rows,
            receipt_digest,
        })
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

    pub fn shell_view_selected_row_digest(&self) -> &str {
        &self.shell_view_selected_row_digest
    }

    pub fn native_query_read_receipt_digest(&self) -> &str {
        &self.native_query_read_receipt_digest
    }

    pub fn selected_legality_receipt_digest(&self) -> &str {
        &self.selected_legality_receipt_digest
    }

    pub fn read_source_digest(&self) -> &str {
        &self.read_source_digest
    }

    pub const fn touched_closure_shell_view_bound(&self) -> usize {
        self.touched_closure_shell_view_bound
    }

    pub const fn selected_source_row_count(&self) -> usize {
        self.selected_source_row_count
    }

    pub const fn available_source_row_count(&self) -> usize {
        self.available_source_row_count
    }

    pub const fn read_stage_counters(&self) -> &ShellViewReadStageCounters {
        &self.read_stage_counters
    }

    pub fn selected_rows(&self) -> &[ShellViewBoundarySourceRow] {
        &self.selected_rows
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }
}

fn touched_closure_shell_view_bound(selected_plan: &DerivedInvalidationSelectedPlan) -> usize {
    let counters = selected_plan.counters();
    counters.touched_entity_count()
        + counters.touched_relation_count()
        + counters.touched_relation_kind_count()
        + counters.touched_aspect_count()
        + counters.touched_scope_count()
}
