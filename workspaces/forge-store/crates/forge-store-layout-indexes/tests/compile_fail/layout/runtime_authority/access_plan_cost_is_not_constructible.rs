use forge_store_layout_indexes::{AccessPlanCostClass, AccessPlanCostEstimate};

fn forge() -> AccessPlanCostEstimate {
    AccessPlanCostEstimate {
        class: AccessPlanCostClass::BTreePointLookup,
        operation_counters: panic!(),
        estimated_memory_bytes: 0,
        estimated_page_reads: 0,
        estimated_chunk_reads: 0,
        estimated_range_touches: 0,
        estimated_byte_reads: 0,
        exact_coverage: None,
    }
}

fn main() {}
