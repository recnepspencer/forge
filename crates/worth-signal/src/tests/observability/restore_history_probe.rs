use crate::tests::domains::fintech::{
    compile_financial_locality_world_with_policy, restore_lifecycle_definition,
};
use crate::tests::performance_profiles::throughput_definition::{profiles, PERFORMANCE_SEED};

#[test]
fn public_restore_snapshot_does_not_import_destination_compile_history() {
    let definition = restore_lifecycle_definition(PERFORMANCE_SEED);
    let idle = profiles()
        .into_iter()
        .find(|profile| profile.name == "throughput_idle")
        .expect("idle profile");
    let forensic = profiles()
        .into_iter()
        .find(|profile| profile.name == "introspective")
        .expect("forensic profile");
    let mut source = compile_financial_locality_world_with_policy(definition.clone(), idle.policy)
        .expect("idle source compiles");
    let snapshot = source
        .locality_capture_runtime_snapshot()
        .expect("idle snapshot");
    let mut destination = compile_financial_locality_world_with_policy(definition, forensic.policy)
        .expect("forensic destination compiles");
    let dest_before = destination.locality_optional_observation_inventory();
    destination
        .locality_restore_runtime_snapshot_keeping_destination_policy(&snapshot)
        .expect("public restore with destination policy");
    let inventory = destination.locality_optional_observation_inventory();
    let snapshot_sequences = snapshot
        .diagnostics
        .lineage_records
        .iter()
        .map(|record| record.sequence)
        .collect::<Vec<_>>();
    for sequence in dest_before.lineage_sequences {
        if !snapshot_sequences.contains(&sequence) {
            assert!(
                !inventory.lineage_sequences.contains(&sequence),
                "destination compile lineage {sequence} must not survive public restore"
            );
        }
    }
}
