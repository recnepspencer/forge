use serde::Serialize;

use super::TraversalViewsDerivedProductOutput;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TraversalViewsMigrationCounters {
    touched_closure_traversal_bound: usize,
    selected_traversal_count: usize,
    available_traversal_count: usize,
    execution_work_count: usize,
    whole_view_fallback_count: usize,
    non_traversal_placeholder_execution_count: usize,
    old_authority_residue_count: usize,
    counters_digest: String,
}

impl TraversalViewsMigrationCounters {
    pub(crate) fn new(
        output: &TraversalViewsDerivedProductOutput,
        execution_work_count: usize,
        whole_view_fallback_count: usize,
        non_traversal_placeholder_execution_count: usize,
        old_authority_residue_count: usize,
    ) -> Self {
        let touched_closure_traversal_bound = output.touched_closure_traversal_bound();
        let selected_traversal_count = output.selected_traversal_count();
        let available_traversal_count = output.available_traversal_count();
        let counters_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:traversal-views-migration-counters:v1".to_string(),
            format!("touched-bound:{touched_closure_traversal_bound}"),
            format!("selected-traversals:{selected_traversal_count}"),
            format!("available-traversals:{available_traversal_count}"),
            format!("execution-work:{execution_work_count}"),
            format!("whole-view-fallbacks:{whole_view_fallback_count}"),
            format!("non-traversal-placeholders:{non_traversal_placeholder_execution_count}"),
            format!("old-authority-residue:{old_authority_residue_count}"),
        ]);
        Self {
            touched_closure_traversal_bound,
            selected_traversal_count,
            available_traversal_count,
            execution_work_count,
            whole_view_fallback_count,
            non_traversal_placeholder_execution_count,
            old_authority_residue_count,
            counters_digest,
        }
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

    pub const fn execution_work_count(&self) -> usize {
        self.execution_work_count
    }

    pub const fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }

    pub const fn non_traversal_placeholder_execution_count(&self) -> usize {
        self.non_traversal_placeholder_execution_count
    }

    pub const fn old_authority_residue_count(&self) -> usize {
        self.old_authority_residue_count
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }
}
