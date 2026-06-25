use serde::Serialize;

use super::row::{
    DerivedInvalidationDenialKind, DerivedInvalidationDenialRow,
    DerivedInvalidationPlannedDisposition, DerivedInvalidationResidueRow,
    DerivedInvalidationSelectedRow,
};
use crate::topology_operators::TopologyTouchedGraphCounters;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationSelectionCounters {
    candidate_product_count: usize,
    matched_product_count: usize,
    invalidated_product_count: usize,
    unaffected_product_count: usize,
    denied_product_count: usize,
    residue_product_count: usize,
    incremental_update_count: usize,
    bounded_rebuild_count: usize,
    whole_view_fallback_count: usize,
    touched_entity_count: usize,
    touched_relation_count: usize,
    touched_relation_kind_count: usize,
    touched_aspect_count: usize,
    touched_scope_count: usize,
    caller_owned_graph_work_count: usize,
    counters_digest: String,
}

impl DerivedInvalidationSelectionCounters {
    pub(super) fn from_rows(
        touched_counters: TopologyTouchedGraphCounters,
        candidate_product_count: usize,
        matched_product_count: usize,
        selected_rows: &[DerivedInvalidationSelectedRow],
        unaffected_product_count: usize,
        denial_rows: &[DerivedInvalidationDenialRow],
        residue_rows: &[DerivedInvalidationResidueRow],
    ) -> Self {
        let incremental_update_count = selected_rows
            .iter()
            .filter(|row| {
                row.planned_disposition()
                    == DerivedInvalidationPlannedDisposition::IncrementalUpdate
            })
            .count();
        let bounded_rebuild_count = selected_rows
            .iter()
            .filter(|row| {
                row.planned_disposition() == DerivedInvalidationPlannedDisposition::BoundedRebuild
            })
            .count();
        let denied_product_count = denial_rows
            .iter()
            .map(DerivedInvalidationDenialRow::family_identity)
            .collect::<std::collections::BTreeSet<_>>()
            .len();
        let whole_view_fallback_count = 0;
        let caller_owned_graph_work_count = 0;
        let invalidated_product_count = selected_rows.len() + denied_product_count;
        let counters_digest = super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-selection-counters:v1".to_string(),
            format!("candidate-products:{candidate_product_count}"),
            format!("matched-products:{matched_product_count}"),
            format!("invalidated-products:{invalidated_product_count}"),
            format!("unaffected-products:{unaffected_product_count}"),
            format!("denied-products:{denied_product_count}"),
            format!("residue-products:{}", residue_rows.len()),
            format!("incremental-updates:{incremental_update_count}"),
            format!("bounded-rebuilds:{bounded_rebuild_count}"),
            format!("whole-view-fallbacks:{whole_view_fallback_count}"),
            format!("touched-entities:{}", touched_counters.entity_count()),
            format!("touched-relations:{}", touched_counters.relation_count()),
            format!(
                "touched-relation-kinds:{}",
                touched_counters.relation_kind_count()
            ),
            format!(
                "touched-aspects:{}",
                touched_counters.touched_aspect_count()
            ),
            format!("touched-scopes:{}", touched_counters.topology_scope_count()),
            format!("caller-owned-graph-work:{caller_owned_graph_work_count}"),
            format!(
                "query-denials:{}",
                denial_count(
                    denial_rows,
                    DerivedInvalidationDenialKind::MissingQuerySupport
                )
            ),
            format!(
                "legality-denials:{}",
                denial_count(
                    denial_rows,
                    DerivedInvalidationDenialKind::MissingLegalitySupport
                )
            ),
        ]);
        Self {
            candidate_product_count,
            matched_product_count,
            invalidated_product_count,
            unaffected_product_count,
            denied_product_count,
            residue_product_count: residue_rows.len(),
            incremental_update_count,
            bounded_rebuild_count,
            whole_view_fallback_count,
            touched_entity_count: touched_counters.entity_count(),
            touched_relation_count: touched_counters.relation_count(),
            touched_relation_kind_count: touched_counters.relation_kind_count(),
            touched_aspect_count: touched_counters.touched_aspect_count(),
            touched_scope_count: touched_counters.topology_scope_count(),
            caller_owned_graph_work_count,
            counters_digest,
        }
    }

    pub const fn candidate_product_count(&self) -> usize {
        self.candidate_product_count
    }

    pub const fn matched_product_count(&self) -> usize {
        self.matched_product_count
    }

    pub const fn invalidated_product_count(&self) -> usize {
        self.invalidated_product_count
    }

    pub const fn unaffected_product_count(&self) -> usize {
        self.unaffected_product_count
    }

    pub const fn denied_product_count(&self) -> usize {
        self.denied_product_count
    }

    pub const fn residue_product_count(&self) -> usize {
        self.residue_product_count
    }

    pub const fn incremental_update_count(&self) -> usize {
        self.incremental_update_count
    }

    pub const fn bounded_rebuild_count(&self) -> usize {
        self.bounded_rebuild_count
    }

    pub const fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }

    pub const fn touched_entity_count(&self) -> usize {
        self.touched_entity_count
    }

    pub const fn touched_relation_count(&self) -> usize {
        self.touched_relation_count
    }

    pub const fn touched_relation_kind_count(&self) -> usize {
        self.touched_relation_kind_count
    }

    pub const fn touched_aspect_count(&self) -> usize {
        self.touched_aspect_count
    }

    pub const fn touched_scope_count(&self) -> usize {
        self.touched_scope_count
    }

    pub const fn caller_owned_graph_work_count(&self) -> usize {
        self.caller_owned_graph_work_count
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }
}

fn denial_count(
    denial_rows: &[DerivedInvalidationDenialRow],
    kind: DerivedInvalidationDenialKind,
) -> usize {
    denial_rows.iter().filter(|row| row.kind() == kind).count()
}
