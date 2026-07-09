use worth_query::facade::runtime::{
    WorthQueryPersistentGraphIndexRequirementCounters,
    WorthQueryPersistentGraphIndexRequirementReceipt,
};

fn main() {
    let _ = WorthQueryPersistentGraphIndexRequirementReceipt {
        digest: String::new(),
        declaration_digest: String::new(),
        counters: WorthQueryPersistentGraphIndexRequirementCounters {
            requirement_row_count: 0,
            persistent_store_owner_row_count: 0,
            blocked_allocation_count: 0,
            durable_artifact_count: 0,
        },
    };
}
