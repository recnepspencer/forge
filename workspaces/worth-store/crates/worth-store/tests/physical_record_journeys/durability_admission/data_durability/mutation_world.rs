use std::path::Path;

use worth_proof::TransitionOutcome;
use worth_signal::facade::TemporalDuration;
use worth_store::physical_runtime::certification::MediaFaultDirective;
use worth_store::physical_runtime::{
    FilesystemMediaAdmission, PhysicalMutationDeadline, PhysicalMutationIdempotencyMaterial,
    PhysicalMutationRequest, PhysicalRuntimeAdmission, PhysicalStore, PhysicalWalAppendOutcome,
    PhysicalWalBarrierOutcome, RecordAppendBatch,
};
use worth_store_physical_backend::{
    CertificationMediaFaultActivation, FilesystemAccessPosture, MediaOperationRole,
};

pub(super) fn append(
    submission: &worth_store::physical_runtime::PhysicalRecordSubmission,
    placement: worth_store::physical_runtime::AdmittedRecordPlacementPolicy,
    material: PhysicalMutationIdempotencyMaterial,
    batch: RecordAppendBatch,
) -> worth_store::physical_runtime::WalAppendedPhysicalMutation {
    let key = submission.issue_idempotency_key(material).unwrap();
    let prepared = match submission
        .prepare_durable_append(
            batch,
            placement,
            PhysicalMutationRequest::platform_durable(
                key,
                PhysicalMutationDeadline::at(TemporalDuration::temporal_duration(1_000).unwrap()),
            ),
        )
        .into_raw()
    {
        TransitionOutcome::Success(prepared) => prepared,
        _ => panic!("the real durable preparation path must succeed"),
    };
    match submission.append_prepared_wal(prepared) {
        PhysicalWalAppendOutcome::Appended(appended) => appended,
        _ => panic!("the real WAL append path must succeed"),
    }
}

pub(super) fn synchronize(
    submission: &worth_store::physical_runtime::PhysicalRecordSubmission,
    appended: worth_store::physical_runtime::WalAppendedPhysicalMutation,
) -> worth_store::physical_runtime::WalDurablePhysicalMutation {
    match submission.synchronize_appended_wal(appended) {
        PhysicalWalBarrierOutcome::Durable(durable) => durable,
        _ => panic!("the real WAL barrier must mint durable authority"),
    }
}

pub(super) fn fault_scheduled_media(
    root: &Path,
    target: u64,
    directive: MediaFaultDirective,
) -> (
    worth_store::physical_runtime::MediaOwnedPhysicalRuntime,
    CertificationMediaFaultActivation,
) {
    let admission =
        FilesystemMediaAdmission::certification(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let activation = authority.one_shot_activation();
    let schedule = authority
        .schedule(vec![authority
            .rule(MediaOperationRole::PositionedWrite, target, directive)
            .for_next_identified_operation_after_activation(
                activation.clone(),
            )])
        .unwrap();
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(root).unwrap()).unwrap();
    let media = match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("fault-scheduled media admission must succeed"),
    };
    (media, activation)
}

pub(super) fn fault_scheduled_media_at_identified_ordinal(
    root: &Path,
    target: u64,
    directive: MediaFaultDirective,
) -> worth_store::physical_runtime::MediaOwnedPhysicalRuntime {
    let admission =
        FilesystemMediaAdmission::certification(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let schedule = authority
        .schedule(vec![authority
            .rule(MediaOperationRole::PositionedWrite, target, directive)
            .for_identified_operation_ordinal()])
        .unwrap();
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(root).unwrap()).unwrap();
    match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("fault-scheduled media admission must succeed"),
    }
}

pub(super) fn certification_media(
    root: &Path,
) -> worth_store::physical_runtime::MediaOwnedPhysicalRuntime {
    let admission =
        FilesystemMediaAdmission::certification(FilesystemAccessPosture::CoordinatedServiceAccount);
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(root).unwrap()).unwrap();
    match runtime.try_admit_filesystem_media(admission).into_raw() {
        TransitionOutcome::Success(media) => media,
        _ => panic!("certification media admission must succeed"),
    }
}
