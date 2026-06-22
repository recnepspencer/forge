use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPersistentGraphIndexRequirementCounters {
    requirement_row_count: usize,
    persistent_store_owner_row_count: usize,
    blocked_allocation_count: usize,
    durable_artifact_count: usize,
}

impl ForgeQueryPersistentGraphIndexRequirementCounters {
    pub fn requirement_row_count(&self) -> usize {
        self.requirement_row_count
    }

    pub fn persistent_store_owner_row_count(&self) -> usize {
        self.persistent_store_owner_row_count
    }

    pub fn blocked_allocation_count(&self) -> usize {
        self.blocked_allocation_count
    }

    pub fn durable_artifact_count(&self) -> usize {
        self.durable_artifact_count
    }

    pub(crate) fn new(requirement_row_count: usize) -> Self {
        Self {
            requirement_row_count,
            persistent_store_owner_row_count: requirement_row_count,
            blocked_allocation_count: usize::from(requirement_row_count > 0),
            durable_artifact_count: 0,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        hash_parts(&[
            "forge_query_persistent_graph_index_requirement_counters_v1".to_string(),
            format!("requirement_rows:{}", self.requirement_row_count),
            format!(
                "persistent_store_owner_rows:{}",
                self.persistent_store_owner_row_count
            ),
            format!("blocked_allocations:{}", self.blocked_allocation_count),
            format!("durable_artifacts:{}", self.durable_artifact_count),
        ])
    }
}
