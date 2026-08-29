#[test]
fn native_fault_subset_uses_the_production_protocol_world_and_independent_oracle() {
    let evidence = crate::phase6_native_lifecycle::verify_native_fault_contract();

    assert!(evidence.qualified_schedules >= 15);
    assert!(evidence.state_event_pairs > 100);
    assert!(evidence.exact_capacity_preserved_sequence);
    assert!(evidence.over_capacity_stopped_before_retention);
    assert!(evidence.invalid_ime_range_stopped_before_retention);
}
