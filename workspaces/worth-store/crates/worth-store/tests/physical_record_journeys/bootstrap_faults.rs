use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    FilesystemMediaAdmission, PhysicalRecordInitialization, PhysicalRecordOpen,
    PhysicalRuntimeAdmission, PhysicalStore,
};
use worth_store_physical_backend::{
    FilesystemAccessPosture, MediaFaultDirective, MediaOperationRole,
};

use super::{configuration, media, success};

#[test]
fn incomplete_bootstrap_publication_never_returns_reusable_authority() {
    let baseline_parent = tempfile::tempdir().unwrap();
    let baseline_root = baseline_parent.path().join("baseline");
    let baseline = media(&baseline_root);
    let before = baseline.media_counters();
    let (format, placement, access) = configuration();
    let serving = success(
        baseline
            .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access)),
    );
    let after = serving.media_counters();
    serving.close();

    let effect_roles = [
        MediaOperationRole::CreateDirectory,
        MediaOperationRole::CreateNew,
        MediaOperationRole::PositionedWrite,
        MediaOperationRole::SynchronizeFileState,
        MediaOperationRole::SynchronizeDirectoryPublication,
        MediaOperationRole::AtomicReplace,
    ];
    let mut cut_index = 0;
    for role in effect_roles {
        let first = before.attempts_for(role) + 1;
        let last = after.attempts_for(role);
        for ordinal in first..=last {
            let mutation_started = role != MediaOperationRole::CreateDirectory || ordinal != first;
            let catalog_may_exist =
                role == MediaOperationRole::SynchronizeDirectoryPublication && ordinal == last;
            exercise_cut(
                cut_index,
                role,
                ordinal,
                MediaFaultDirective::FailBefore {
                    kind: std::io::ErrorKind::Other,
                    raw_os_error: None,
                },
                mutation_started,
                catalog_may_exist,
            );
            cut_index += 1;
        }
    }

    for role in [
        MediaOperationRole::AtomicReplace,
        MediaOperationRole::SynchronizeDirectoryPublication,
    ] {
        exercise_cut(
            cut_index,
            role,
            after.attempts_for(role),
            MediaFaultDirective::IndeterminateAfterEffect,
            true,
            true,
        );
        cut_index += 1;
    }
}

fn exercise_cut(
    index: usize,
    role: MediaOperationRole,
    ordinal: u64,
    directive: MediaFaultDirective,
    mutation_started: bool,
    catalog_may_exist: bool,
) {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join(format!("store-{index}"));
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let schedule = authority
        .schedule(vec![authority.rule(role, ordinal, directive)])
        .unwrap();
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(&root).unwrap()).unwrap();
    let faulted_media = match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("cut point must be after media admission"),
    };
    let (format, placement, access) = configuration();
    let outcome = faulted_media
        .initialize_record_store(PhysicalRecordInitialization::new(format, placement, access))
        .into_raw();
    if !mutation_started {
        let TransitionOutcome::Denied(denial) = outcome else {
            panic!("a pre-effect cut must return reusable media authority");
        };
        denial.into_runtime().close();
        assert!(!root.join("families/records").exists());
        return;
    }
    assert!(matches!(outcome, TransitionOutcome::Failed(_)));

    let observed = media(&root)
        .open_record_store(PhysicalRecordOpen::new(format, access))
        .into_raw();
    match observed {
        TransitionOutcome::Success(serving) if catalog_may_exist => {
            serving.close();
        }
        TransitionOutcome::Denied(denial) => {
            denial.into_runtime().close();
        }
        _ => panic!("fresh observation must never manufacture bootstrap authority"),
    }
}
