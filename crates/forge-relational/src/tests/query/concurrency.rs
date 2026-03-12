use std::sync::Arc;

use crate::tests::support::*;

#[test]
fn concurrent_snapshot_and_version_reads_match_serial_truth() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::AiWorkflow);
    let created = create_entity_outcome(&mut runtime, "before");
    let entity = changed_entities(&created)[0];
    let explicit_snapshot = runtime.snapshot_access().snapshot();
    let updated = update_entity(&mut runtime, entity, "after");
    let serial_snapshot_name = {
        let read = runtime.visibility_reads().read_snapshot(&explicit_snapshot).unwrap();
        read_entity_name(read.get_entity(entity).unwrap()).unwrap().to_string()
    };
    let serial_version_name = {
        let read = runtime.visibility_reads().read_version(created.version_id);
        read_entity_name(read.get_entity(entity).unwrap()).unwrap().to_string()
    };
    let serial_latest_name = {
        let read = runtime.visibility_reads().read_snapshot(&updated.snapshot).unwrap();
        read_entity_name(read.get_entity(entity).unwrap()).unwrap().to_string()
    };
    let runtime = Arc::new(runtime);

    std::thread::scope(|scope| {
        let mut snapshot_threads = Vec::new();
        for _ in 0..8 {
            let runtime = Arc::clone(&runtime);
            let explicit_snapshot = explicit_snapshot.clone();
            let published_snapshot = updated.snapshot.clone();
            snapshot_threads.push(scope.spawn(move || {
                let snapshot_read = runtime.visibility_reads().read_snapshot(&explicit_snapshot).unwrap();
                let version_read = runtime.visibility_reads().read_version(created.version_id);
                let latest_read = runtime.visibility_reads().read_snapshot(&published_snapshot).unwrap();
                (
                    read_entity_name(snapshot_read.get_entity(entity).unwrap())
                        .unwrap()
                        .to_string(),
                    read_entity_name(version_read.get_entity(entity).unwrap())
                        .unwrap()
                        .to_string(),
                    read_entity_name(latest_read.get_entity(entity).unwrap())
                        .unwrap()
                        .to_string(),
                )
            }));
        }

        for thread in snapshot_threads {
            let (snapshot_name, version_name, latest_name) = thread.join().unwrap();
            assert_eq!(snapshot_name, serial_snapshot_name);
            assert_eq!(version_name, serial_version_name);
            assert_eq!(latest_name, serial_latest_name);
        }
    });
}

#[test]
fn concurrent_read_pressure_keeps_cache_diagnostics_coherent() {
    let mut runtime = runtime_with_test_schema_profile(RelationalRuntimeProfile::GeometryKernel);
    let created = create_entity_outcome(&mut runtime, "baseline");
    let entity = changed_entities(&created)[0];
    let explicit_snapshot = runtime.snapshot_access().snapshot();
    let updated = update_entity(&mut runtime, entity, "mutated");
    let _ = create_entity_outcome(&mut runtime, "churn-1");
    let _ = create_entity_outcome(&mut runtime, "churn-2");
    let _ = create_entity_outcome(&mut runtime, "churn-3");
    runtime.reset_complexity_counters();
    let runtime = Arc::new(runtime);

    std::thread::scope(|scope| {
        let mut readers = Vec::new();
        for _ in 0..6 {
            let runtime = Arc::clone(&runtime);
            let explicit_snapshot = explicit_snapshot.clone();
            let published_snapshot = updated.snapshot.clone();
            readers.push(scope.spawn(move || {
                let snapshot_diag = runtime
                    .visibility_reads().inspect_snapshot_read_path(&explicit_snapshot)
                    .expect("explicit snapshot diagnostics");
                let published_diag = runtime
                    .visibility_reads().inspect_snapshot_read_path(&published_snapshot)
                    .expect("published snapshot diagnostics");
                let historical = runtime.visibility_reads().read_version(created.version_id);
                let historical_name = read_entity_name(historical.get_entity(entity).unwrap())
                    .unwrap()
                    .to_string();
                (
                    snapshot_diag.entries.len(),
                    published_diag.entries.len(),
                    historical_name,
                )
            }));
        }

        for reader in readers {
            let (snapshot_entries, published_entries, historical_name) = reader.join().unwrap();
            assert!(snapshot_entries > 0);
            assert!(published_entries > 0);
            assert_eq!(historical_name, "baseline");
        }
    });

    let counters = runtime.complexity_counters();
    assert!(counters.visibility_cache_hits > 0);
}
