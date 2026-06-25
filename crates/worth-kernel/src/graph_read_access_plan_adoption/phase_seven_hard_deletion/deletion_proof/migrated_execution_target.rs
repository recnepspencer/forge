#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MigratedExecutionTarget {
    pub(crate) label: &'static str,
    pub(crate) source_path: &'static str,
    pub(crate) owner: &'static str,
    pub(crate) removal_trigger: &'static str,
}

impl MigratedExecutionTarget {
    const fn deleted(
        label: &'static str,
        source_path: &'static str,
        owner: &'static str,
        removal_trigger: &'static str,
    ) -> Self {
        Self {
            label,
            source_path,
            owner,
            removal_trigger,
        }
    }
}

const MIGRATED_EXECUTION_TARGETS: &[MigratedExecutionTarget] = &[
    MigratedExecutionTarget::deleted(
        "old_query_adoption_graph_read_access_module",
        "crates/worth-kernel/src/query_adoption/graph_read_access",
        "worth-kernel.graph_read_access_plan_adoption",
        "deleted once Milestone 6 inventory and Milestone 8 plan adoption owned graph-read access",
    ),
    MigratedExecutionTarget::deleted(
        "old_construction_graph_read_access_adoption_entrypoint",
        "crates/worth-kernel/src/query_adoption/graph_read_access/mod.rs",
        "worth-kernel.graph_read_access_plan_adoption",
        "deleted once Query access plan adoption became the only construction graph-read route",
    ),
];

pub(crate) const fn current_migrated_execution_targets() -> &'static [MigratedExecutionTarget] {
    MIGRATED_EXECUTION_TARGETS
}
