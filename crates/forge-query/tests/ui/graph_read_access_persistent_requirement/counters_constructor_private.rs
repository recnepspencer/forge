use forge_query::facade::runtime::ForgeQueryPersistentGraphIndexRequirementCounters;

fn main() {
    let _ = ForgeQueryPersistentGraphIndexRequirementCounters {
        requirement_row_count: 0,
        persistent_store_owner_row_count: 0,
        blocked_allocation_count: 0,
        durable_artifact_count: 0,
    };
}
