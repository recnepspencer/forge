use crate::tests::support::*;

#[test]
fn geometry_profile_does_not_force_retention_pass_on_each_commit() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);

    runtime.performance_access().reset_counters();
    let created = create_entity_outcome(&mut runtime, "geometry-hot-retention");
    let entity = changed_entities(&created)[0];
    let counters_after_create = runtime.performance_access().counters();

    assert_eq!(counters_after_create.retention_entity_slots_scanned, 0);
    assert_eq!(counters_after_create.retention_relation_slots_scanned, 0);

    runtime.performance_access().reset_counters();
    let deleted = delete_entity(&mut runtime, entity);
    let counters_after_delete = runtime.performance_access().counters();

    assert_eq!(counters_after_delete.retention_entity_slots_scanned, 0);
    assert_eq!(counters_after_delete.retention_relation_slots_scanned, 0);

    assert!(runtime
        .visibility_authority()
        .release_snapshot(&created.snapshot));
    assert!(runtime
        .visibility_authority()
        .release_snapshot(&deleted.snapshot));

    let plan = runtime.retention().inspect_plan();
    assert!(plan.reclaimable_entities >= 1);
}
