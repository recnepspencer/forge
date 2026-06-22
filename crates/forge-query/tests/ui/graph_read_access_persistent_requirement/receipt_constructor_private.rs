use forge_query::facade::runtime::{
    ForgeQueryPersistentGraphIndexRequirementCounters,
    ForgeQueryPersistentGraphIndexRequirementReceipt,
};

fn main() {
    let _ = ForgeQueryPersistentGraphIndexRequirementReceipt {
        digest: String::new(),
        declaration_digest: String::new(),
        counters: ForgeQueryPersistentGraphIndexRequirementCounters {
            requirement_row_count: 0,
            persistent_store_owner_row_count: 0,
            blocked_allocation_count: 0,
            durable_artifact_count: 0,
        },
    };
}
