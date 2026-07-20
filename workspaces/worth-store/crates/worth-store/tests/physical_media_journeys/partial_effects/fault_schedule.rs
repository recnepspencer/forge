use worth_store::physical_runtime::certification::{
    CertificationMediaFaultAuthority, MediaFaultDirective, MediaPauseGate,
};
use worth_store::physical_runtime::FilesystemMediaAdmission;
use worth_store_physical_backend::{FilesystemAccessPosture, MediaOperationRole};

pub(super) fn fault_admission(case: &str) -> (FilesystemMediaAdmission, Option<MediaPauseGate>) {
    let admission =
        FilesystemMediaAdmission::certification(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let (role, ordinal, directive, gate) = match case {
        "pass-through" => return (admission, None),
        "before-root-creation" => fail_before(MediaOperationRole::CreateDirectory, 1),
        "short-identity-prefix" => (
            MediaOperationRole::PositionedWrite,
            1,
            MediaFaultDirective::AllowPrefix { bytes: 17 },
            None,
        ),
        "directory-barrier-indeterminate" => {
            fail_barrier(MediaOperationRole::SynchronizeDirectoryPublication, 1)
        }
        "abrupt-after-replacement" => pause_after(&authority, MediaOperationRole::AtomicReplace, 1),
        "after-fixed-directories" => (
            MediaOperationRole::CreateDirectory,
            4,
            MediaFaultDirective::IndeterminateAfterEffect,
            None,
        ),
        "before-staged-identity-create" => fail_before(MediaOperationRole::CreateNew, 1),
        "after-staged-identity-create" => fail_before(MediaOperationRole::PositionedWrite, 1),
        "after-complete-identity-write" => fail_before(MediaOperationRole::SynchronizeFileState, 1),
        "file-barrier-denial" => fail_barrier(MediaOperationRole::SynchronizeFileState, 1),
        "after-identity-file-sync" => fail_before(MediaOperationRole::AtomicReplace, 1),
        "after-directory-sync-before-observation" => pause_after(
            &authority,
            MediaOperationRole::SynchronizeRootParentPublication,
            1,
        ),
        "qualification-positioned-write" => (
            MediaOperationRole::PositionedWrite,
            2,
            MediaFaultDirective::AllowPrefix { bytes: 31 },
            None,
        ),
        "qualification-append" => fail_before(MediaOperationRole::Append, 1),
        "qualification-truncate" => fail_before(MediaOperationRole::Truncate, 1),
        "qualification-allocation" => fail_before(MediaOperationRole::Allocate, 1),
        "qualification-metadata" => fail_before(MediaOperationRole::ReadMetadata, 1),
        "qualification-list" => fail_before(MediaOperationRole::ListDirectory, 1),
        "cleanup-delete" => fail_before(MediaOperationRole::Delete, 1),
        "cleanup-directory-barrier" => {
            fail_barrier(MediaOperationRole::SynchronizeDirectoryPublication, 3)
        }
        "before-lock-release" => {
            let gate = authority.pause_gate();
            let schedule = authority
                .schedule(Vec::new())
                .unwrap()
                .pause_before_lease_release(gate.clone());
            return (admission.with_fault_schedule(schedule), Some(gate));
        }
        "after-lock-release" => {
            pause_after(&authority, MediaOperationRole::ReleaseMutationLease, 1)
        }
        _ => panic!("unknown fault case: {case}"),
    };
    let schedule = authority
        .schedule(vec![authority.rule(role, ordinal, directive)])
        .unwrap();
    (admission.with_fault_schedule(schedule), gate)
}

fn fail_before(
    role: MediaOperationRole,
    ordinal: u64,
) -> (
    MediaOperationRole,
    u64,
    MediaFaultDirective,
    Option<MediaPauseGate>,
) {
    (
        role,
        ordinal,
        MediaFaultDirective::FailBefore {
            kind: std::io::ErrorKind::PermissionDenied,
            raw_os_error: None,
        },
        None,
    )
}

fn fail_barrier(
    role: MediaOperationRole,
    ordinal: u64,
) -> (
    MediaOperationRole,
    u64,
    MediaFaultDirective,
    Option<MediaPauseGate>,
) {
    (
        role,
        ordinal,
        MediaFaultDirective::FailBarrier {
            kind: std::io::ErrorKind::Other,
            raw_os_error: None,
        },
        None,
    )
}

fn pause_after(
    authority: &CertificationMediaFaultAuthority,
    role: MediaOperationRole,
    ordinal: u64,
) -> (
    MediaOperationRole,
    u64,
    MediaFaultDirective,
    Option<MediaPauseGate>,
) {
    let gate = authority.pause_gate();
    (
        role,
        ordinal,
        MediaFaultDirective::PauseAfter(gate.clone()),
        Some(gate),
    )
}
