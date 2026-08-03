use super::{
    binary_binding::BuiltCourtroomExecutables, execution, oracle, world::BoundedResidencySiegeWorld,
};

#[test]
fn process_allocation_observer_falsifies_complete_store_materialization() {
    let workspace = crate::workspace_root();
    let world =
        BoundedResidencySiegeWorld::create(None).expect("real hostile world must construct");
    let binaries = BuiltCourtroomExecutables::build(&workspace)
        .expect("current courtroom binaries must build");
    crate::mutation_campaign::emit_nested_executable(binaries.writer().path());
    let child = execution::observe_serving_for_mutation(&world, &binaries)
        .expect("real producer and serving processes must complete");

    if let Err(failure) = oracle::verify_process_allocation(&child) {
        panic!("C5_PREDICATE:whole-store-allocation {failure}");
    }
}

#[test]
fn physical_work_topology_falsifies_unsettled_metadata_read() {
    let workspace = crate::workspace_root();
    let world =
        BoundedResidencySiegeWorld::create(None).expect("real hostile world must construct");
    let binaries = BuiltCourtroomExecutables::build(&workspace)
        .expect("current courtroom binaries must build");
    crate::mutation_campaign::emit_nested_executable(binaries.writer().path());
    let child = match execution::observe_serving_for_mutation(&world, &binaries) {
        Ok(child) => child,
        Err(failure) if failure.contains("physical work/media topology drifted") => {
            panic!("C5_PREDICATE:physical-work-metadata-topology-bypass {failure}")
        }
        Err(failure) => panic!("unexpected producer or serving failure: {failure}"),
    };

    if let Err(failure) = oracle::verify_work_reconciliation(&child) {
        panic!("C5_PREDICATE:physical-work-metadata-topology-bypass {failure}");
    }
}
