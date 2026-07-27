#[test]
fn physical_adapters_receive_observation_without_pool_control() {
    let cases = trybuild::TestCases::new();
    for supported in [
        "physical_integrity_supported.rs",
        "physical_isolation_supported.rs",
        "blob_streaming_supported.rs",
        "operation_scopes_supported.rs",
    ] {
        cases.pass(format!("tests/physical_adapter_authority/{supported}"));
    }
    for denied in [
        "pool_construction_is_sealed.rs",
        "eviction_authority_is_sealed.rs",
        "dirty_mutation_authority_is_sealed.rs",
        "generation_forgery_is_rejected.rs",
        "semantic_residency_inference_is_rejected.rs",
        "certification_authority_is_absent.rs",
    ] {
        cases.compile_fail(format!("tests/physical_adapter_authority/{denied}"));
    }
}
