use serde::Serialize;

use super::{VertexDiskReadSource, VertexDiskReadStageCounters};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::migrated_products::vertex_disks::{
    VertexDiskBoundarySourceRow, VertexDiskMigrationError,
};
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VertexDiskReadStageReceipt {
    selected_plan_digest: String,
    touched_closure_digest: String,
    query_support_digest: String,
    legality_support_digest: String,
    vertex_disk_selected_row_digest: String,
    native_query_read_receipt_digest: String,
    selected_legality_receipt_digest: String,
    read_source_digest: String,
    touched_closure_vertex_disk_bound: usize,
    selected_source_row_count: usize,
    available_source_row_count: usize,
    read_stage_counters: VertexDiskReadStageCounters,
    selected_rows: Vec<VertexDiskBoundarySourceRow>,
    receipt_digest: String,
}

impl VertexDiskReadStageReceipt {
    pub fn from_selected_plan_and_read_source(
        selected_plan: &DerivedInvalidationSelectedPlan,
        read_source: VertexDiskReadSource,
    ) -> Result<Self, VertexDiskMigrationError> {
        let selected_row = selected_plan
            .selected_rows()
            .iter()
            .find(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::VertexDisks)
            .ok_or(VertexDiskMigrationError::SelectedPlanMissingVertexDiskRow)?;
        let native_query_read_receipt_digest = selected_row
            .query_receipt_digest()
            .ok_or(VertexDiskMigrationError::ReadStageReceiptMissingQueryReceipt)?
            .to_string();
        if !read_source
            .query_report_digests()
            .contains(&native_query_read_receipt_digest)
        {
            return Err(VertexDiskMigrationError::ReadStageQueryReceiptNotBoundToSource);
        }
        let selected_legality_receipt_digest = selected_row
            .legality_receipt_digest()
            .ok_or(VertexDiskMigrationError::ReadStageReceiptMissingLegalityReceipt)?
            .to_string();
        let touched_closure_vertex_disk_bound = touched_closure_vertex_disk_bound(selected_plan);
        let selected_source_row_count = read_source.selected_rows().len();
        if selected_source_row_count == 0 {
            return Err(VertexDiskMigrationError::ReadStageTouchedClosureSelectedNoVertexDiskRows);
        }
        if selected_source_row_count > touched_closure_vertex_disk_bound {
            return Err(VertexDiskMigrationError::ReadStageSelectedRowsExceedTouchedClosure);
        }

        let vertex_disk_selected_row_digest = selected_row.row_digest().to_string();
        let selected_rows = read_source.selected_rows().to_vec();
        let available_source_row_count = read_source.available_source_row_count();
        let read_stage_counters = *read_source.counters();
        let read_source_digest = read_source.read_source_digest().to_string();
        let mut receipt_parts = vec![
            "worth-topo:vertex-disk-read-stage-receipt:v1".to_string(),
            format!("selected-plan:{}", selected_plan.selected_plan_digest()),
            format!("touched-closure:{}", selected_plan.touched_closure_digest()),
            format!("query-support:{}", selected_plan.query_support_digest()),
            format!(
                "legality-support:{}",
                selected_plan.legality_support_digest()
            ),
            format!("selected-row:{vertex_disk_selected_row_digest}"),
            format!("native-query-read:{native_query_read_receipt_digest}"),
            format!("selected-legality:{selected_legality_receipt_digest}"),
            format!("read-source:{read_source_digest}"),
            format!("touched-bound:{touched_closure_vertex_disk_bound}"),
            format!("selected-source-rows:{selected_source_row_count}"),
            format!("available-source-rows:{available_source_row_count}"),
            format!(
                "read-stage-touched-vertices:{}",
                read_stage_counters.touched_vertex_count()
            ),
            format!(
                "read-stage-touched-half-edge-lookups:{}",
                read_stage_counters.touched_half_edge_lookup_count()
            ),
            format!(
                "read-stage-selected-vertex-disk-roots:{}",
                read_stage_counters.selected_vertex_disk_root_count()
            ),
            format!(
                "read-stage-touched-incident-half-edges:{}",
                read_stage_counters.touched_incident_half_edge_count()
            ),
            format!(
                "read-stage-touched-incident-edges:{}",
                read_stage_counters.touched_incident_edge_count()
            ),
            format!(
                "read-stage-unrelated-vertex-disk-breadth:{}",
                read_stage_counters.unrelated_vertex_disk_breadth_count()
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
            vertex_disk_selected_row_digest,
            native_query_read_receipt_digest,
            selected_legality_receipt_digest,
            read_source_digest,
            touched_closure_vertex_disk_bound,
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

    pub fn vertex_disk_selected_row_digest(&self) -> &str {
        &self.vertex_disk_selected_row_digest
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

    pub const fn touched_closure_vertex_disk_bound(&self) -> usize {
        self.touched_closure_vertex_disk_bound
    }

    pub const fn selected_source_row_count(&self) -> usize {
        self.selected_source_row_count
    }

    pub const fn available_source_row_count(&self) -> usize {
        self.available_source_row_count
    }

    pub const fn read_stage_counters(&self) -> &VertexDiskReadStageCounters {
        &self.read_stage_counters
    }

    pub fn selected_rows(&self) -> &[VertexDiskBoundarySourceRow] {
        &self.selected_rows
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }
}

fn touched_closure_vertex_disk_bound(selected_plan: &DerivedInvalidationSelectedPlan) -> usize {
    let counters = selected_plan.counters();
    counters.touched_entity_count()
        + counters.touched_relation_count()
        + counters.touched_relation_kind_count()
        + counters.touched_aspect_count()
        + counters.touched_scope_count()
}
