use std::num::{NonZeroU32, NonZeroU64};
use std::time::{Duration, Instant};

use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    AdmittedPhysicalRecordFormat, AdmittedPhysicalRecordResidencyPolicy, FilesystemMediaAdmission,
    PhysicalOperationAllocationScope, PhysicalRecordInitialization, PhysicalRecordOpen,
    PhysicalRecordResidencyPolicy, PhysicalResidencyAllocationSnapshot,
    PhysicalResidencyCounterSnapshot, PhysicalResidencyDimension, PhysicalResidencyRetryPosture,
    PhysicalRuntimeAdmission, PhysicalSpeculativeWorkKind, PhysicalStore, PhysicalWorkEffectFate,
    RecordAppendBatch, RecordAppendDenial, RecordByteLimit, RecordPublicationStage,
    RecordReadDenial, RecordReadLimits, RecordServingTerminalPosture,
};
use worth_store_physical_backend::{
    FilesystemAccessPosture, MediaFaultDirective, MediaOperationRole,
};

use super::{configuration, media, success};

#[test]
fn public_read_and_append_pressure_retains_exact_pre_effect_basis() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("public-residency-pressure");
    let (format, placement, access) = configuration();
    let seeded = success(initialize_record_store!(media(&root), |durability| {
        PhysicalRecordInitialization::new(format, placement, access, durability)
    }));
    let published = seeded
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"first".as_slice(), b"second".as_slice()]).unwrap(),
            placement,
        )
        .unwrap();
    let first_record = published.record_id(0).unwrap();
    let second_record = published.record_id(1).unwrap();
    assert!(!seeded.close().residency().requires_inspection());

    let policy = one_page_operation_policy(format);
    let serving = success(open_record_store!(media(&root), |durability| {
        PhysicalRecordOpen::new(format, access, durability).with_residency_policy(policy)
    },));
    let store = serving.store_identity();
    let observation = serving.residency_observation();
    let generation = observation.store_generation();
    let counters: PhysicalResidencyCounterSnapshot = observation.counters();
    let allocations: PhysicalResidencyAllocationSnapshot = observation.allocations();
    assert_eq!(observation.store_identity(), store);
    assert_eq!(observation.admitted_policy(), policy);
    assert_eq!(allocations.store_identity(), store);
    assert_eq!(
        allocations
            .for_dimension(PhysicalResidencyDimension::MetadataBytes)
            .active_units(),
        counters.metadata_bytes(),
    );
    let page_bytes = u64::from(format.declaration().page_size().bytes());
    let read_limits = RecordReadLimits::new(RecordByteLimit::new(16).unwrap());

    let held = serving.records().open(first_record, read_limits).unwrap();
    let before_read_denial = serving.media_counters();
    let read_error = match serving.records().open(second_record, read_limits) {
        Err(error) => error,
        Ok(_) => panic!("the second read must not spend the held read allocation"),
    };
    assert_eq!(read_error.denial(), RecordReadDenial::PhysicalPressure);
    let read_pressure = read_error.pressure().unwrap();
    assert_eq!(read_pressure.basis().store_identity(), store);
    assert_eq!(read_pressure.basis().record(), Some(second_record));
    assert_eq!(read_pressure.store_generation(), generation);
    assert_eq!(
        read_pressure.scope(),
        PhysicalOperationAllocationScope::ForegroundRead
    );
    assert_eq!(
        read_pressure.dimension(),
        PhysicalResidencyDimension::OperationScope(
            PhysicalOperationAllocationScope::ForegroundRead,
        )
    );
    assert_eq!(read_pressure.requested(), page_bytes);
    assert_eq!(read_pressure.admitted(), page_bytes);
    assert_eq!(read_pressure.limit(), page_bytes);
    assert_eq!(
        read_pressure.retry_posture(),
        PhysicalResidencyRetryPosture::AfterAllocationRelease
    );
    assert!(!read_pressure.effect_may_have_started());
    assert_eq!(serving.media_counters(), before_read_denial);
    drop(held);

    let before_append_denial = serving.media_counters();
    let append_error = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"blocked".as_slice()]).unwrap(),
            placement,
        )
        .unwrap_err();
    assert_eq!(
        append_error.pressure_denial(),
        Some(RecordAppendDenial::PhysicalPressure)
    );
    let append_pressure = append_error.pressure().unwrap();
    assert_eq!(append_pressure.basis().store_identity(), store);
    assert_eq!(append_pressure.basis().record(), None);
    assert_eq!(append_pressure.store_generation(), generation);
    assert_eq!(
        append_pressure.scope(),
        PhysicalOperationAllocationScope::ForegroundWrite
    );
    assert_eq!(
        append_pressure.dimension(),
        PhysicalResidencyDimension::OperationScope(
            PhysicalOperationAllocationScope::ForegroundWrite,
        )
    );
    assert!(append_pressure.requested() > page_bytes);
    assert_eq!(append_pressure.admitted(), 0);
    assert_eq!(append_pressure.limit(), page_bytes);
    assert_eq!(
        append_pressure.retry_posture(),
        PhysicalResidencyRetryPosture::AfterConfigurationChange
    );
    assert!(!append_pressure.effect_may_have_started());
    assert_eq!(serving.media_counters(), before_append_denial);
    assert!(!serving.close().residency().requires_inspection());
}

#[test]
fn ordinary_writebehind_pressure_cleans_extent_residue_before_retry() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("writebehind-pressure-cleanup");
    let (format, placement, access) = configuration();
    let initialized = success(initialize_record_store!(media(&root), |durability| {
        PhysicalRecordInitialization::new(format, placement, access, durability)
    }));
    assert!(!initialized.close().residency().requires_inspection());

    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let authority = admission.fault_schedule_authority();
    let gate = authority.pause_gate();
    let schedule = authority
        .schedule(vec![authority
            .rule(
                MediaOperationRole::PositionedWrite,
                2,
                MediaFaultDirective::PauseBefore(gate.clone()),
            )
            .for_identified_operation_ordinal()])
        .unwrap();
    let runtime = PhysicalStore::admit(PhysicalRuntimeAdmission::new(&root).unwrap()).unwrap();
    let media = match runtime
        .try_admit_filesystem_media(admission.with_fault_schedule(schedule))
        .into_raw()
    {
        TransitionOutcome::Success(media) => media,
        _ => panic!("writebehind pressure media must admit"),
    };
    let serving = success(open_record_store!(media, |durability| {
        PhysicalRecordOpen::new(format, access, durability)
            .with_residency_policy(two_append_writebehind_policy(format))
    },));
    let payload = vec![73_u8; 1024 * 1024];
    let residency_before = serving.residency_observation().counters();
    let submission = serving.record_submission();
    let primary_batch = RecordAppendBatch::try_from_iter([payload.as_slice()]).unwrap();
    let primary = std::thread::spawn(move || submission.append_batch(primary_batch, placement));

    let deadline = Instant::now() + Duration::from_secs(5);
    while gate.reached_context().is_none() && Instant::now() < deadline {
        std::thread::yield_now();
    }
    let context = gate
        .reached_context()
        .expect("primary append must reach candidate writeback");
    assert_eq!(context.role(), MediaOperationRole::PositionedWrite);
    assert_eq!(context.identified_operation_ordinal(), Some(2));
    let residency_at_dispatch = serving.residency_observation().counters();
    let before_denial = serving.media_counters();

    let denial = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([payload.as_slice()]).unwrap(),
            placement,
        )
        .unwrap_err();
    let pressure = denial
        .pressure()
        .expect("writebehind pressure must project");
    assert_eq!(
        pressure.dimension(),
        PhysicalResidencyDimension::SpeculativeFrames(PhysicalSpeculativeWorkKind::WriteBehind)
    );
    assert_eq!(pressure.basis().store_identity(), serving.store_identity());
    assert!(pressure.basis().frame_coordinate().is_some());
    assert_eq!(pressure.requested(), 1);
    assert_eq!(pressure.admitted(), 1);
    assert_eq!(pressure.limit(), 1);
    assert_eq!(
        pressure.retry_posture(),
        PhysicalResidencyRetryPosture::AfterWritebackSettlement
    );
    assert!(!pressure.effect_may_have_started());
    assert!(serving.media_counters().deletions() > before_denial.deletions());
    assert!(serving.publication_residue().is_empty());
    assert!(!serving.observed_non_authoritative_residue());
    let residency_after_denial = serving.residency_observation().counters();

    gate.release();
    let primary = primary.join().unwrap().unwrap();
    assert_eq!(primary.record_ids().len(), 1);
    let primary_writebacks = assert_extent_candidate_trace(&primary);
    let residency_after_primary = serving.residency_observation().counters();
    let retry = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([payload.as_slice()]).unwrap(),
            placement,
        )
        .unwrap();
    assert_eq!(retry.record_ids().len(), 1);
    let retry_writebacks = assert_extent_candidate_trace(&retry);
    let counters = serving.residency_observation().counters();
    let successful_writebacks = primary_writebacks + retry_writebacks;
    assert_eq!(counters.writebacks(), successful_writebacks);
    let primary_candidate_publications = residency_at_dispatch
        .candidate_publications()
        .saturating_sub(residency_before.candidate_publications())
        .saturating_add(
            residency_after_primary
                .candidate_publications()
                .saturating_sub(residency_after_denial.candidate_publications()),
        );
    let denied_candidate_publications = residency_after_denial
        .candidate_publications()
        .saturating_sub(residency_at_dispatch.candidate_publications());
    let retry_candidate_publications = counters
        .candidate_publications()
        .saturating_sub(residency_after_primary.candidate_publications());
    assert_eq!(denied_candidate_publications, 1);
    assert!(primary_candidate_publications > primary_writebacks);
    assert!(retry_candidate_publications > retry_writebacks);
    assert_eq!(
        counters.candidate_publications(),
        primary_candidate_publications
            + retry_candidate_publications
            + denied_candidate_publications
    );
    let close = serving.close();
    assert_eq!(
        close.records().posture(),
        RecordServingTerminalPosture::NoInspectionRequired
    );
    assert!(!close.residency().requires_inspection());
}

fn assert_extent_candidate_trace(
    published: &worth_store::physical_runtime::PublishedRecordBatch,
) -> u64 {
    let fates = published
        .physical_work()
        .effects()
        .iter()
        .filter(|effect| effect.stage() == RecordPublicationStage::CandidateDataWrite)
        .map(|effect| effect.settlement().unwrap().effect_fate())
        .collect::<Vec<_>>();
    assert_eq!(
        fates
            .iter()
            .filter(|fate| **fate == PhysicalWorkEffectFate::PublicationCompleted)
            .count(),
        1,
        "an extent append creates exactly one candidate artifact"
    );
    let writebacks = fates
        .iter()
        .filter(|fate| **fate == PhysicalWorkEffectFate::WriteCompleted)
        .count() as u64;
    assert!(writebacks > 0, "the hostile extent must require writeback");
    assert_eq!(fates.len() as u64, writebacks + 1);
    writebacks
}

fn one_page_operation_policy(
    format: AdmittedPhysicalRecordFormat,
) -> AdmittedPhysicalRecordResidencyPolicy {
    use PhysicalOperationAllocationScope as Scope;
    use PhysicalSpeculativeWorkKind as Kind;

    let page = u64::from(format.declaration().page_size().bytes());
    let metadata = 16 * 1024;
    let resident = page * 4;
    let mut builder = PhysicalRecordResidencyPolicy::builder()
        .total_bytes(nonzero(metadata + resident + page + page))
        .resident_bytes(nonzero(resident))
        .metadata_bytes(nonzero(metadata))
        .frame_entries(nonzero_count(4))
        .pinned_frames(nonzero_count(4))
        .pin_leases(nonzero_count(4))
        .dirty_frames(nonzero_count(2))
        .dirty_replacement_bytes(nonzero(page))
        .operation_bytes(nonzero(page));
    for scope in [
        Scope::ForegroundRead,
        Scope::ForegroundWrite,
        Scope::Recovery,
        Scope::Scrub,
        Scope::Maintenance,
        Scope::Verification,
        Scope::Blob,
    ] {
        builder = builder.scope_bytes(scope, nonzero(page));
    }
    for kind in [Kind::ReadAhead, Kind::Prefetch, Kind::WriteBehind] {
        builder = builder.speculative_frames(kind, nonzero_count(2));
    }
    builder.admit(format).into_result().unwrap()
}

fn two_append_writebehind_policy(
    format: AdmittedPhysicalRecordFormat,
) -> AdmittedPhysicalRecordResidencyPolicy {
    use PhysicalOperationAllocationScope as Scope;
    use PhysicalSpeculativeWorkKind as Kind;

    let page = u64::from(format.declaration().page_size().bytes());
    let operation = page * 416;
    let metadata = page * 2;
    let resident = page * 4;
    let mut builder = PhysicalRecordResidencyPolicy::builder()
        .total_bytes(nonzero(operation + metadata + resident))
        .resident_bytes(nonzero(resident))
        .metadata_bytes(nonzero(metadata))
        .frame_entries(nonzero_count(12))
        .pinned_frames(nonzero_count(4))
        .pin_leases(nonzero_count(6))
        .dirty_frames(nonzero_count(2))
        .dirty_replacement_bytes(nonzero(resident))
        .operation_bytes(nonzero(operation));
    for scope in [
        Scope::ForegroundRead,
        Scope::ForegroundWrite,
        Scope::Recovery,
        Scope::Scrub,
        Scope::Maintenance,
        Scope::Verification,
        Scope::Blob,
    ] {
        builder = builder.scope_bytes(scope, nonzero(operation));
    }
    for (kind, frames) in [
        (Kind::ReadAhead, 2),
        (Kind::Prefetch, 2),
        (Kind::WriteBehind, 1),
    ] {
        builder = builder.speculative_frames(kind, nonzero_count(frames));
    }
    builder.admit(format).into_result().unwrap()
}

fn nonzero(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).unwrap()
}

fn nonzero_count(value: u32) -> NonZeroU32 {
    NonZeroU32::new(value).unwrap()
}
