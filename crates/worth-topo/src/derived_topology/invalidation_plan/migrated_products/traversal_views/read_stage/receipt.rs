use serde::Serialize;

use super::{TraversalViewsReadSource, TraversalViewsSourceRow};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::migrated_products::traversal_views::TraversalViewsMigrationError;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TraversalViewsReadStageReceipt {
    selected_plan_digest: String,
    touched_closure_digest: String,
    query_support_digest: String,
    legality_support_digest: String,
    traversal_views_selected_row_digest: String,
    native_query_read_receipt_digest: String,
    selected_legality_receipt_digest: String,
    read_source_digest: String,
    touched_closure_traversal_bound: usize,
    selected_traversal_count: usize,
    available_traversal_count: usize,
    selected_rows: Vec<TraversalViewsSourceRow>,
    receipt_digest: String,
}

impl TraversalViewsReadStageReceipt {
    pub fn from_selected_plan_and_read_source(
        selected_plan: &DerivedInvalidationSelectedPlan,
        read_source: TraversalViewsReadSource,
    ) -> Result<Self, TraversalViewsMigrationError> {
        let selected_traversal_views_row = selected_plan
            .selected_rows()
            .iter()
            .find(|row| {
                row.family_identity() == DerivedTopologyProductFamilyIdentity::TraversalViews
            })
            .ok_or(TraversalViewsMigrationError::SelectedPlanMissingTraversalViewsRow)?;
        let native_query_read_receipt_digest = selected_traversal_views_row
            .query_receipt_digest()
            .ok_or(TraversalViewsMigrationError::ReadStageReceiptMissingQueryReceipt)?
            .to_string();
        let selected_legality_receipt_digest = selected_traversal_views_row
            .legality_receipt_digest()
            .ok_or(TraversalViewsMigrationError::ReadStageReceiptMissingLegalityReceipt)?
            .to_string();
        let traversal_views_selected_row_digest =
            selected_traversal_views_row.row_digest().to_string();
        let selected_traversal_count = read_source.selected_rows().len();
        let available_traversal_count = read_source.available_traversal_count();
        let touched_closure_traversal_bound = touched_closure_traversal_bound(selected_plan);
        if selected_traversal_count > touched_closure_traversal_bound {
            return Err(TraversalViewsMigrationError::ReadStageSelectedRowsExceedTouchedClosure);
        }
        let read_source_digest = read_source.source_digest().to_string();
        let selected_rows = read_source.selected_rows().to_vec();
        let mut receipt_parts = vec![
            "worth-topo:traversal-views-read-stage-receipt:v1".to_string(),
            format!("selected-plan:{}", selected_plan.selected_plan_digest()),
            format!("touched-closure:{}", selected_plan.touched_closure_digest()),
            format!("query-support:{}", selected_plan.query_support_digest()),
            format!(
                "legality-support:{}",
                selected_plan.legality_support_digest()
            ),
            format!("selected-row:{traversal_views_selected_row_digest}"),
            format!("native-query-read:{native_query_read_receipt_digest}"),
            format!("selected-legality:{selected_legality_receipt_digest}"),
            format!("read-source:{read_source_digest}"),
            format!("touched-bound:{touched_closure_traversal_bound}"),
            format!("selected-traversals:{selected_traversal_count}"),
            format!("available-traversals:{available_traversal_count}"),
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
            traversal_views_selected_row_digest,
            native_query_read_receipt_digest,
            selected_legality_receipt_digest,
            read_source_digest,
            touched_closure_traversal_bound,
            selected_traversal_count,
            available_traversal_count,
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

    pub fn traversal_views_selected_row_digest(&self) -> &str {
        &self.traversal_views_selected_row_digest
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

    pub const fn touched_closure_traversal_bound(&self) -> usize {
        self.touched_closure_traversal_bound
    }

    pub const fn selected_traversal_count(&self) -> usize {
        self.selected_traversal_count
    }

    pub const fn available_traversal_count(&self) -> usize {
        self.available_traversal_count
    }

    pub fn selected_rows(&self) -> &[TraversalViewsSourceRow] {
        &self.selected_rows
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }

    #[cfg(test)]
    pub(crate) fn with_selected_plan_digest_for_tests(
        mut self,
        selected_plan_digest: &'static str,
    ) -> Self {
        self.selected_plan_digest = selected_plan_digest.to_string();
        self
    }

    #[cfg(test)]
    pub(crate) fn with_native_query_read_receipt_digest_for_tests(
        mut self,
        digest: &'static str,
    ) -> Self {
        self.native_query_read_receipt_digest = digest.to_string();
        self
    }
}

fn touched_closure_traversal_bound(selected_plan: &DerivedInvalidationSelectedPlan) -> usize {
    let counters = selected_plan.counters();
    counters.touched_entity_count()
        + counters.touched_relation_count()
        + counters.touched_relation_kind_count()
        + counters.touched_aspect_count()
        + counters.touched_scope_count()
}
