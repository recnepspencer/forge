use worth_spatial::facade::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;

fn main() {
    let _ = EvidenceLookupConsumedWorkloadHandoff {
        stage_receipt_identity: fake(),
        workload_stage_index_identity: fake(),
        selected_lookup_plan_digest: fake(),
        lookup_execution_receipt_digest: fake(),
        lookup_product_output_digest: fake(),
        topology_derived_receipt_state: fake(),
        covered_family_identities: fake(),
        counters: fake(),
        milestone_twelve_seed: fake(),
    };
}

fn fake<T>() -> T {
    unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
}
