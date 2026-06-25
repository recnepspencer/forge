use serde::Serialize;

use super::MaterializedGraphDerivedProductOutput;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaterializedGraphMigrationCounters {
    selected_entity_count: usize,
    selected_relation_count: usize,
    available_entity_count: usize,
    available_relation_count: usize,
    topology_entity_count: usize,
    topology_relation_count: usize,
    execution_work_count: usize,
    whole_view_fallback_count: usize,
    non_materialized_placeholder_execution_count: usize,
    old_authority_residue_count: usize,
    counters_digest: String,
}

impl MaterializedGraphMigrationCounters {
    pub(crate) fn new(
        output: &MaterializedGraphDerivedProductOutput,
        execution_work_count: usize,
        whole_view_fallback_count: usize,
        non_materialized_placeholder_execution_count: usize,
        old_authority_residue_count: usize,
    ) -> Self {
        let selected_entity_count = output.selected_entity_count();
        let selected_relation_count = output.selected_relation_count();
        let available_entity_count = output.available_entity_count();
        let available_relation_count = output.available_relation_count();
        let topology_entity_count = output.topology_entity_count();
        let topology_relation_count = output.topology_relation_count();
        let counters_digest = super::super::super::catalog::catalog_digest([
            "worth-topo:materialized-graph-migration-counters:v1".to_string(),
            format!("selected-entities:{selected_entity_count}"),
            format!("selected-relations:{selected_relation_count}"),
            format!("available-entities:{available_entity_count}"),
            format!("available-relations:{available_relation_count}"),
            format!("topology-entities:{topology_entity_count}"),
            format!("topology-relations:{topology_relation_count}"),
            format!("execution-work:{execution_work_count}"),
            format!("whole-view-fallbacks:{whole_view_fallback_count}"),
            format!("non-materialized-placeholders:{non_materialized_placeholder_execution_count}"),
            format!("old-authority-residue:{old_authority_residue_count}"),
        ]);
        Self {
            selected_entity_count,
            selected_relation_count,
            available_entity_count,
            available_relation_count,
            topology_entity_count,
            topology_relation_count,
            execution_work_count,
            whole_view_fallback_count,
            non_materialized_placeholder_execution_count,
            old_authority_residue_count,
            counters_digest,
        }
    }

    pub const fn selected_entity_count(&self) -> usize {
        self.selected_entity_count
    }

    pub const fn selected_relation_count(&self) -> usize {
        self.selected_relation_count
    }

    pub const fn available_entity_count(&self) -> usize {
        self.available_entity_count
    }

    pub const fn available_relation_count(&self) -> usize {
        self.available_relation_count
    }

    pub const fn topology_entity_count(&self) -> usize {
        self.topology_entity_count
    }

    pub const fn topology_relation_count(&self) -> usize {
        self.topology_relation_count
    }

    pub const fn execution_work_count(&self) -> usize {
        self.execution_work_count
    }

    pub const fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }

    pub const fn non_materialized_placeholder_execution_count(&self) -> usize {
        self.non_materialized_placeholder_execution_count
    }

    pub const fn old_authority_residue_count(&self) -> usize {
        self.old_authority_residue_count
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }
}
