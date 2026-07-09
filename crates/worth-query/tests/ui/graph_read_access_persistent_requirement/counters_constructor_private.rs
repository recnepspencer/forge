use worth_query::facade::runtime::WorthQueryPersistentGraphIndexRequirementCounters;

fn main() {
    let _ = WorthQueryPersistentGraphIndexRequirementCounters {
        requirement_row_count: 0,
        persistent_store_owner_row_count: 0,
        blocked_allocation_count: 0,
        durable_artifact_count: 0,
    };
}
