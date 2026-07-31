use std::path::Path;

use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    AdmittedPhysicalRecordResidencyPolicy, FilesystemMediaAdmission, PhysicalRecordOpen,
    PhysicalRuntimeAdmission, PhysicalStore, PhysicalWorkProfileDeclaration,
    ServingPhysicalRuntime,
};
use worth_store_physical_backend::{
    FilesystemAccessPosture, MediaFaultDirective, MediaFaultSchedule, MediaOperationRole,
};

pub(super) fn serving_from_open_with_schedule(
    root: &Path,
    schedule: MediaFaultSchedule,
) -> ServingPhysicalRuntime {
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(root).unwrap()).unwrap();
    let media = match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("scheduled media should admit"),
    };
    let (format, _, access) = super::super::configuration();
    super::super::success(open_record_store!(media, |durability| {
        PhysicalRecordOpen::new(format, access, durability)
    }))
}

pub(super) fn serving_from_open_with_positioned_write_fault(
    root: &Path,
    profile: PhysicalWorkProfileDeclaration,
    directive: MediaFaultDirective,
) -> ServingPhysicalRuntime {
    serving_from_open_with_positioned_write_fault_at(root, profile, 1, directive)
}

pub(super) fn serving_from_open_with_positioned_write_fault_at(
    root: &Path,
    profile: PhysicalWorkProfileDeclaration,
    ordinal: u64,
    directive: MediaFaultDirective,
) -> ServingPhysicalRuntime {
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let schedule = authority
        .schedule(vec![authority.rule(
            MediaOperationRole::PositionedWrite,
            ordinal,
            directive,
        )])
        .unwrap();
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(root).unwrap()).unwrap();
    let media = match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("faulted media should admit"),
    };
    let (format, _, access) = super::super::configuration();
    super::super::success(open_record_store!(media, |durability| {
        PhysicalRecordOpen::new(format, access, durability).with_physical_work_profile(profile)
    },))
}

pub(super) fn serving_from_open_with_positioned_read_fault(
    root: &Path,
    ordinal: u64,
    directive: MediaFaultDirective,
) -> ServingPhysicalRuntime {
    serving_from_open_with_positioned_read_fault_policy(root, ordinal, directive, None)
}

pub(super) fn serving_from_open_with_positioned_read_fault_and_policy(
    root: &Path,
    ordinal: u64,
    directive: MediaFaultDirective,
    policy: AdmittedPhysicalRecordResidencyPolicy,
) -> ServingPhysicalRuntime {
    serving_from_open_with_positioned_read_fault_policy(root, ordinal, directive, Some(policy))
}

fn serving_from_open_with_positioned_read_fault_policy(
    root: &Path,
    ordinal: u64,
    directive: MediaFaultDirective,
    policy: Option<AdmittedPhysicalRecordResidencyPolicy>,
) -> ServingPhysicalRuntime {
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let schedule = authority
        .schedule(vec![authority.rule(
            MediaOperationRole::PositionedRead,
            ordinal,
            directive,
        )])
        .unwrap();
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(root).unwrap()).unwrap();
    let media = match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("faulted media should admit"),
    };
    let (format, _, access) = super::super::configuration();
    let durability = super::super::durability(&media);
    let open = PhysicalRecordOpen::new(format, access, durability);
    let open = match policy {
        Some(policy) => open.with_residency_policy(policy),
        None => open,
    };
    super::super::success(media.open_record_store(open))
}

pub(super) fn serving_from_open_with_paused_positioned_read_failure(
    root: &Path,
    ordinal: u64,
) -> (
    ServingPhysicalRuntime,
    worth_store_physical_backend::MediaPauseGate,
) {
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let gate = authority.pause_gate();
    let schedule = authority
        .schedule(vec![authority.rule(
            MediaOperationRole::PositionedRead,
            ordinal,
            MediaFaultDirective::PauseBeforeThenFailBefore {
                gate: gate.clone(),
                kind: std::io::ErrorKind::Other,
                raw_os_error: None,
            },
        )])
        .unwrap();
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(root).unwrap()).unwrap();
    let media = match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("paused failing media should admit"),
    };
    let (format, _, access) = super::super::configuration();
    (
        super::super::success(open_record_store!(media, |durability| {
            PhysicalRecordOpen::new(format, access, durability)
        })),
        gate,
    )
}

pub(super) fn serving_from_open_with_identified_positioned_read_fault(
    root: &Path,
    ordinal: u64,
    directive: MediaFaultDirective,
) -> ServingPhysicalRuntime {
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let schedule = authority
        .schedule(vec![authority
            .rule(MediaOperationRole::PositionedRead, ordinal, directive)
            .for_identified_operation_ordinal()])
        .unwrap();
    serving_from_open_with_schedule(root, schedule)
}

pub(super) fn serving_from_open_with_two_write_pauses(
    root: &Path,
) -> (
    ServingPhysicalRuntime,
    worth_store_physical_backend::MediaPauseGate,
    worth_store_physical_backend::MediaPauseGate,
) {
    serving_from_open_with_two_write_pauses_and_profile(
        root,
        PhysicalWorkProfileDeclaration::default(),
    )
}

pub(super) fn serving_from_open_with_two_write_pauses_and_profile(
    root: &Path,
    profile: PhysicalWorkProfileDeclaration,
) -> (
    ServingPhysicalRuntime,
    worth_store_physical_backend::MediaPauseGate,
    worth_store_physical_backend::MediaPauseGate,
) {
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let first = authority.pause_gate();
    let second = authority.pause_gate();
    let schedule = authority
        .schedule(vec![
            authority
                .rule(
                    MediaOperationRole::PositionedWrite,
                    1,
                    MediaFaultDirective::PauseBefore(first.clone()),
                )
                .for_identified_operation_ordinal(),
            authority
                .rule(
                    MediaOperationRole::PositionedWrite,
                    2,
                    MediaFaultDirective::PauseBefore(second.clone()),
                )
                .for_identified_operation_ordinal(),
        ])
        .unwrap();
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(root).unwrap()).unwrap();
    let media = match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("paused media should admit"),
    };
    let (format, _, access) = super::super::configuration();
    (
        super::super::success(open_record_store!(media, |durability| {
            PhysicalRecordOpen::new(format, access, durability).with_physical_work_profile(profile)
        },)),
        first,
        second,
    )
}

pub(super) fn serving_from_open_with_one_write_pause(
    root: &Path,
) -> (
    ServingPhysicalRuntime,
    worth_store_physical_backend::MediaPauseGate,
) {
    serving_from_open_with_one_write_pause_and_profile(
        root,
        PhysicalWorkProfileDeclaration::default(),
    )
}

pub(super) fn serving_from_open_with_one_write_pause_and_profile(
    root: &Path,
    profile: PhysicalWorkProfileDeclaration,
) -> (
    ServingPhysicalRuntime,
    worth_store_physical_backend::MediaPauseGate,
) {
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let gate = authority.pause_gate();
    let schedule = authority
        .schedule(vec![authority
            .rule(
                MediaOperationRole::PositionedWrite,
                1,
                MediaFaultDirective::PauseBefore(gate.clone()),
            )
            .for_identified_operation_ordinal()])
        .unwrap();
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(root).unwrap()).unwrap();
    let media = match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("paused media should admit"),
    };
    let (format, _, access) = super::super::configuration();
    (
        super::super::success(open_record_store!(media, |durability| {
            PhysicalRecordOpen::new(format, access, durability).with_physical_work_profile(profile)
        },)),
        gate,
    )
}
