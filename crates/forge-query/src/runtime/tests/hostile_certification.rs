use super::support::*;

#[test]
fn hostile_certification_artifact_matches_serialized_replay_and_repeated_runs() {
    let first = execute_runtime_hostile_schedule();
    let replay = replay_runtime_hostile_schedule();
    let second = execute_runtime_hostile_schedule();

    assert_eq!(first, replay);
    assert_eq!(first, second);
    assert_eq!(first.digest().as_str(), replay.digest().as_str());
    assert_eq!(first.digest().as_str(), second.digest().as_str());
}

#[test]
fn hostile_certification_counters_stay_at_exact_zero() {
    let artifact = execute_runtime_hostile_schedule();
    let counters = artifact.counters();

    assert_eq!(counters.committed_read_hot_path_lock_count(), 0);
    assert_eq!(counters.reader_derived_evaluation_count(), 0);
    assert_eq!(counters.orphaned_snapshot_generation_count(), 0);
    assert_eq!(counters.unretired_read_pin_count(), 0);
    assert_eq!(counters.journal_gap_count(), 0);
    assert_eq!(counters.delivery_residue_count(), 0);
}
