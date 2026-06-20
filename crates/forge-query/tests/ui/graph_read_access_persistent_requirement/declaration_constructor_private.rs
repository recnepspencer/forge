use forge_query::facade::runtime::{
    ForgeQueryGraphReadRequiredCapabilityOwner,
    ForgeQueryPersistentGraphIndexRequirementCounters,
    ForgeQueryPersistentGraphIndexRequirementDeclaration,
};

fn main() {
    let _ = ForgeQueryPersistentGraphIndexRequirementDeclaration {
        digest: String::new(),
        read_graph_digest: String::new(),
        access_shape_digest: String::new(),
        selectivity_shape_digest: String::new(),
        requirement_set_digest: String::new(),
        inventory_match_report_digest: String::new(),
        estimated_index_bytes: 0,
        estimated_result_bytes: 0,
        required_owner: ForgeQueryGraphReadRequiredCapabilityOwner::PersistentStore,
        requirement_rows: Vec::new(),
        counters: ForgeQueryPersistentGraphIndexRequirementCounters {
            requirement_row_count: 0,
            persistent_store_owner_row_count: 0,
            blocked_allocation_count: 0,
            durable_artifact_count: 0,
        },
    };
}
