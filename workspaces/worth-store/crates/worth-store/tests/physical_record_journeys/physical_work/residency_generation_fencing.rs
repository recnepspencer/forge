use std::cell::Cell;

use worth_store::physical_runtime::{
    AdmittedDirtyFrame, CertificationFrameReadFailure, CertificationFrameWorkFailure,
    PhysicalDirtyTransitionFailure, PhysicalResidencyAllocationBoundaryKind,
    PhysicalResidencyCertification, PhysicalWorkPreEffectDenial, PhysicalWritebackFailureCause,
};
use worth_store_buffer_pool::PhysicalResidencyDimension;
use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;
use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::{serving_from_initialization_with_work_profile, work_fixture};

const REPLACEMENT: [u8; 8] = [0x71, 0xc6, 0x93, 0x28, 0x45, 0xba, 0xd0, 0x1f];

#[test]
fn stale_generation_cannot_consume_an_already_hot_frame() {
    let root = tempfile::tempdir().unwrap();
    let (profile, _, _) = work_fixture();
    let serving = serving_from_initialization_with_work_profile(root.path(), profile);
    let current = serving.certification_physical_residency();
    let coordinate = coordinate();
    drop(current.pin_exact(coordinate).unwrap());
    let stale = serving.certification_stale_physical_residency();
    let residency_before = current.counters();
    let allocations_before = current.allocation_trace();
    let work_before = serving.physical_work_counters();
    let signal_before = serving.physical_signal_observation().unwrap();
    let scheduler_before = serving.physical_scheduler_capacity();
    let media_before = serving.media_counters();

    let failure = match stale.pin_exact(coordinate) {
        Err(failure) => failure,
        Ok(_) => panic!(
            "MUTANT_PREDICATE:stale-residency-generation-consumed: stale residency authority pinned a current hot frame"
        ),
    };

    assert_eq!(
        failure,
        CertificationFrameReadFailure::PhysicalWork(CertificationFrameWorkFailure::PreEffect(
            PhysicalWorkPreEffectDenial::StaleGeneration,
        ),),
        "MUTANT_PREDICATE:stale-residency-generation-consumed"
    );
    assert_eq!(
        current.counters(),
        residency_before,
        "MUTANT_PREDICATE:stale-residency-generation-consumed"
    );
    assert_eq!(
        current.allocation_trace(),
        allocations_before,
        "MUTANT_PREDICATE:stale-residency-generation-consumed"
    );
    assert_eq!(serving.physical_work_counters(), work_before);
    assert_eq!(
        serving.physical_signal_observation().unwrap(),
        signal_before
    );
    assert_eq!(serving.physical_scheduler_capacity(), scheduler_before);
    assert_eq!(serving.media_counters(), media_before);
    assert!(!serving.close().residency().requires_inspection());
}

#[test]
fn stale_generation_cannot_consume_current_lease_for_dirty_admission() {
    let root = tempfile::tempdir().unwrap();
    let (profile, _, _) = work_fixture();
    let serving = serving_from_initialization_with_work_profile(root.path(), profile);
    let current = serving.certification_physical_residency();
    let stale = serving.certification_stale_physical_residency();
    let lease = current.pin_exact(coordinate()).unwrap();
    let fill_called = Cell::new(false);
    let residency_before = current.counters();
    let allocations_before = current.allocation_trace();
    let work_before = serving.physical_work_counters();
    let signal_before = serving.physical_signal_observation().unwrap();
    let scheduler_before = serving.physical_scheduler_capacity();
    let media_before = serving.media_counters();

    let failure = stale
        .admit_dirty_frame(lease, |_, _| fill_called.set(true))
        .unwrap_err();

    assert_eq!(failure, PhysicalDirtyTransitionFailure::StaleOrForeignFrame);
    assert!(
        !fill_called.get(),
        "stale dirty admission invoked the mutation closure"
    );
    let residency_after = current.counters();
    assert_eq!(
        residency_after.pinned_frames() + 1,
        residency_before.pinned_frames(),
        "rejected dirty admission must release the consumed clean pin"
    );
    assert_eq!(
        residency_after.pin_leases() + 1,
        residency_before.pin_leases(),
        "rejected dirty admission must release the consumed clean lease"
    );
    assert_eq!(residency_after.hits(), residency_before.hits());
    assert_eq!(residency_after.faults(), residency_before.faults());
    assert_eq!(
        residency_after.source_loads(),
        residency_before.source_loads()
    );
    assert_eq!(
        residency_after.dirty_frames(),
        residency_before.dirty_frames()
    );
    assert_eq!(
        residency_after.dirty_transitions(),
        residency_before.dirty_transitions()
    );
    assert_eq!(
        residency_after.active_operation_bytes(),
        residency_before.active_operation_bytes()
    );
    assert_eq!(
        residency_after.copied_bytes(),
        residency_before.copied_bytes()
    );
    assert_eq!(
        residency_after.copy_operations(),
        residency_before.copy_operations()
    );
    assert_eq!(residency_after.denials(), residency_before.denials());
    let before_events = allocations_before.events().collect::<Vec<_>>();
    let allocations_after = current.allocation_trace();
    let after_events = allocations_after.events().collect::<Vec<_>>();
    assert_eq!(
        &after_events[..before_events.len()],
        before_events.as_slice(),
        "stale dirty denial changed the preexisting allocation trace"
    );
    let cleanup = &after_events[before_events.len()..];
    assert_eq!(cleanup.len(), 2, "stale dirty denial leaked cleanup events");
    assert_eq!(
        cleanup
            .iter()
            .map(|event| (event.kind(), event.dimension()))
            .collect::<Vec<_>>(),
        vec![
            (
                PhysicalResidencyAllocationBoundaryKind::Release,
                PhysicalResidencyDimension::PinLeases,
            ),
            (
                PhysicalResidencyAllocationBoundaryKind::Release,
                PhysicalResidencyDimension::PinnedFrames,
            ),
        ]
    );
    assert_eq!(serving.physical_work_counters(), work_before);
    assert_eq!(
        serving.physical_signal_observation().unwrap(),
        signal_before
    );
    assert_eq!(serving.physical_scheduler_capacity(), scheduler_before);
    assert_eq!(serving.media_counters(), media_before);
    assert!(!serving.close().residency().requires_inspection());
}

#[test]
fn stale_generation_cannot_consume_current_dirty_writeback_authority() {
    let root = tempfile::tempdir().unwrap();
    let (profile, _, _) = work_fixture();
    let serving = serving_from_initialization_with_work_profile(root.path(), profile);
    let current = serving.certification_physical_residency();
    let stale = serving.certification_stale_physical_residency();
    let dirty = dirty_frame(&current);
    let residency_before = current.counters();
    let allocations_before = current.allocation_trace();
    let writeback_before = serving.residency_observation().writebacks();
    let work_before = serving.physical_work_counters();
    let signal_before = serving.physical_signal_observation().unwrap();
    let scheduler_before = serving.physical_scheduler_capacity();
    let media_before = serving.media_counters();

    let failure = match stale.prepare_writeback(
        dirty,
        ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
    ) {
        Err(failure) => failure,
        Ok(_) => panic!(
            "MUTANT_PREDICATE:stale-residency-generation-consumed: stale residency authority prepared current dirty writeback"
        ),
    };

    assert_eq!(
        failure.cause(),
        PhysicalWritebackFailureCause::PreEffect(PhysicalWorkPreEffectDenial::StaleGeneration,),
        "MUTANT_PREDICATE:stale-residency-generation-consumed"
    );
    assert_eq!(current.counters(), residency_before);
    assert_eq!(current.allocation_trace(), allocations_before);
    assert_eq!(
        serving.residency_observation().writebacks(),
        writeback_before
    );
    assert_eq!(serving.physical_work_counters(), work_before);
    assert_eq!(
        serving.physical_signal_observation().unwrap(),
        signal_before
    );
    assert_eq!(serving.physical_scheduler_capacity(), scheduler_before);
    assert_eq!(serving.media_counters(), media_before);
    failure.into_dirty().discard().unwrap();
    assert!(!serving.close().residency().requires_inspection());
}

#[test]
fn foreign_dirty_authority_cannot_claim_same_coordinate_in_another_live_store() {
    let first_root = tempfile::tempdir().unwrap();
    let second_root = tempfile::tempdir().unwrap();
    let (profile, _, _) = work_fixture();
    let first = serving_from_initialization_with_work_profile(first_root.path(), profile.clone());
    let second = serving_from_initialization_with_work_profile(second_root.path(), profile);
    let first_residency = first.certification_physical_residency();
    let second_residency = second.certification_physical_residency();
    let first_dirty = dirty_frame(&first_residency);
    let second_dirty = dirty_frame(&second_residency);
    let residency_before = second_residency.counters();
    let allocations_before = second_residency.allocation_trace();
    let writeback_before = second.residency_observation().writebacks();
    let work_before = second.physical_work_counters();
    let signal_before = second.physical_signal_observation().unwrap();
    let scheduler_before = second.physical_scheduler_capacity();
    let media_before = second.media_counters();

    let failure = match second_residency.prepare_writeback(
        first_dirty,
        ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
    ) {
        Err(failure) => failure,
        Ok(_) => panic!(
            "MUTANT_PREDICATE:foreign-dirty-frame-claimed: foreign dirty authority claimed the second Store frame"
        ),
    };

    assert_eq!(
        failure.cause(),
        PhysicalWritebackFailureCause::StaleOrForeignDirtyFrame,
        "MUTANT_PREDICATE:foreign-dirty-frame-claimed"
    );
    assert_eq!(
        second_residency.counters(),
        residency_before,
        "MUTANT_PREDICATE:foreign-dirty-frame-claimed"
    );
    assert_eq!(
        second_residency.allocation_trace(),
        allocations_before,
        "MUTANT_PREDICATE:foreign-dirty-frame-claimed"
    );
    assert_eq!(
        second.residency_observation().writebacks(),
        writeback_before,
        "MUTANT_PREDICATE:foreign-dirty-frame-claimed"
    );
    assert_eq!(second.physical_work_counters(), work_before);
    assert_eq!(second.physical_signal_observation().unwrap(), signal_before);
    assert_eq!(second.physical_scheduler_capacity(), scheduler_before);
    assert_eq!(second.media_counters(), media_before);
    failure.into_dirty().discard().unwrap();
    second_dirty.discard().unwrap();
    assert!(!first.close().residency().requires_inspection());
    assert!(!second.close().residency().requires_inspection());
}

fn dirty_frame(residency: &PhysicalResidencyCertification) -> AdmittedDirtyFrame {
    residency
        .admit_dirty_frame(residency.pin_exact(coordinate()).unwrap(), |_, target| {
            target.copy_from_slice(&REPLACEMENT);
        })
        .unwrap()
}

fn coordinate() -> RecordFrameCoordinate {
    RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 8, 8).unwrap()
}
