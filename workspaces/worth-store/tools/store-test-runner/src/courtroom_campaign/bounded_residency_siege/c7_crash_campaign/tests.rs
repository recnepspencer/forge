use super::{
    checkpoint, crash_arguments, run_crash, run_producer, BuiltCourtroomExecutables,
    C7CaseProcessRole, C7DurabilityCrashSeam, DurabilityCheckpointOrder,
};
use crate::courtroom_campaign::bounded_residency_siege::world::BoundedResidencySiegeWorld;

#[test]
fn every_case_process_role_is_globally_unique() {
    let roles = C7DurabilityCrashSeam::ALL
        .into_iter()
        .flat_map(|seam| {
            C7CaseProcessRole::ALL
                .into_iter()
                .map(move |role| role.qualified(seam))
        })
        .collect::<std::collections::BTreeSet<_>>();

    if roles.len() != 40 {
        panic!("MUTANT_PREDICATE:c7-case-process-role-collapsed");
    }
}

#[test]
fn selected_schedule_and_seam_reach_the_c7_child_exactly() {
    let order = DurabilityCheckpointOrder::TargetSealedBeforeCheckpoint;
    let seam = C7DurabilityCrashSeam::DuringDataWritePrefix;
    let arguments = crash_arguments(
        std::path::Path::new("store"),
        std::path::Path::new("configuration"),
        seam,
        order,
    );
    let values = arguments
        .iter()
        .map(|argument| argument.to_string_lossy())
        .collect::<Vec<_>>();
    if !values
        .windows(2)
        .any(|pair| pair[0] == "--schedule-plan" && pair[1] == order.encoded())
        || !values
            .windows(2)
            .any(|pair| pair[0] == "--crash-seam" && pair[1] == seam.label())
    {
        panic!("MUTANT_PREDICATE:c7-selected-schedule-not-propagated");
    }
}

#[test]
fn bounded_checkpoint_memory_reaches_the_selected_termination_point() {
    let workspace = crate::workspace_root();
    let world =
        BoundedResidencySiegeWorld::create(None).expect("real C7 crash world must construct");
    let binaries = BuiltCourtroomExecutables::build(&workspace)
        .expect("current courtroom binaries must build");
    crate::mutation_campaign::emit_nested_executable(binaries.writer().path());
    run_producer(&world, &binaries).expect("C7 seed producer must complete");
    let seam = C7DurabilityCrashSeam::BeforeWalAppend;
    let (crash, marker) = run_crash(
        &world,
        &binaries,
        seam,
        DurabilityCheckpointOrder::CheckpointBeforeTarget,
    )
    .unwrap_or_else(|failure| {
        if failure.contains("schedule checkpoint did not complete")
            && failure.contains("ResidencyUnavailable")
        {
            panic!("MUTANT_PREDICATE:c7-bounded-checkpoint-memory-unadmitted {failure}");
        }
        panic!("MUTANT_PREDICATE:c7-wal-write-role-mismatch {failure}");
    });
    checkpoint::verify(&crash, &marker, seam)
        .unwrap_or_else(|failure| panic!("MUTANT_PREDICATE:c7-wal-write-role-mismatch {failure}"));
}

#[test]
fn wal_barrier_boundary_reaches_the_selected_operation() {
    assert_reaches_selected_operation(
        C7DurabilityCrashSeam::AfterWalBarrierBeforeDataDispatch,
        "c7-wal-barrier-role-mismatch",
    );
}

#[test]
fn data_write_boundary_reaches_the_second_positioned_write() {
    assert_reaches_selected_operation(
        C7DurabilityCrashSeam::DuringDataWritePrefix,
        "c7-data-write-relative-selection-mismatch",
    );
}

#[test]
fn checkpoint_verification_rejects_the_wrong_media_role() {
    let result = checkpoint::verify_signature(
        "C7_COURTROOM_CRASH_CHECKPOINT before-wal-append MediaEffect 41 append:1:1:128:1",
        41,
        C7DurabilityCrashSeam::BeforeWalAppend,
    );
    if result.is_ok() {
        panic!("MUTANT_PREDICATE:c7-marker-media-role-unchecked");
    }
}

#[test]
fn checkpoint_verification_rejects_the_wrong_relative_match() {
    let result = checkpoint::verify_signature(
        "C7_COURTROOM_CRASH_CHECKPOINT during-data-write-prefix MediaEffect 41 positioned_write:9:7:128:1",
        41,
        C7DurabilityCrashSeam::DuringDataWritePrefix,
    );
    if result.is_ok() {
        panic!("MUTANT_PREDICATE:c7-marker-relative-match-unchecked");
    }
}

fn assert_reaches_selected_operation(seam: C7DurabilityCrashSeam, predicate: &str) {
    let workspace = crate::workspace_root();
    let world =
        BoundedResidencySiegeWorld::create(None).expect("real C7 crash world must construct");
    let binaries = BuiltCourtroomExecutables::build(&workspace)
        .expect("current courtroom binaries must build");
    crate::mutation_campaign::emit_nested_executable(binaries.writer().path());
    run_producer(&world, &binaries).expect("C7 seed producer must complete");
    let (crash, marker) = run_crash(
        &world,
        &binaries,
        seam,
        DurabilityCheckpointOrder::CheckpointBeforeTarget,
    )
    .unwrap_or_else(|failure| panic!("MUTANT_PREDICATE:{predicate} {failure}"));
    checkpoint::verify(&crash, &marker, seam)
        .unwrap_or_else(|failure| panic!("MUTANT_PREDICATE:{predicate} {failure}"));
}
