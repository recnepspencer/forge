use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    ExternalPhysicalRecordLocator, PhysicalLocatorReadmissionDenial, PhysicalRecordInitialization,
    PhysicalRecordOpen, PhysicalWorkEffectFate, RecordAppendBatch, RecordAppendDenial,
    RecordAppendError, RecordByteLimit, RecordReadDenial, RecordReadLimits,
    RecordServingTerminalPosture,
};
use worth_store_physical_backend::MediaOperationRole;
use worth_store_physical_format::{
    PhysicalFreeSpaceMembershipBlock, RecordAllocationClass, RecordFreeSpaceManifestEntry,
};

use super::{media, scenario_configuration::dense_configuration, success};

#[test]
fn locator_readmission_and_free_space_truth_survive_reopen() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, access) = dense_configuration(4);
    let serving = success(initialize_record_store!(media(&root), |durability| {
        PhysicalRecordInitialization::new(format, placement, access, durability)
    }));
    let published = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"stable".as_slice()]).unwrap(),
            placement,
        )
        .unwrap();
    let locator = ExternalPhysicalRecordLocator::new(
        serving.store_identity(),
        published.record_id(0).unwrap(),
    );
    serving.close();
    let free = super::manifest_fixture::decode_free_space_tree(&root, 2, format.declaration(), 128);
    assert_eq!(free.header.next_segment(), 2);
    assert_eq!(free.header.next_page(), 2);
    assert_eq!(free.entries.len(), 2);
    let reopened = success(open_record_store!(media(&root), |durability| {
        PhysicalRecordOpen::new(format, access, durability)
    }));
    assert_eq!(
        reopened
            .records()
            .readmit_locator(locator)
            .into_result()
            .unwrap(),
        published.record_id(0).unwrap()
    );
    let before = reopened.media_counters();
    let mut foreign = locator.encode();
    foreign[0] ^= 0xff;
    assert_eq!(
        reopened
            .records()
            .readmit_locator(ExternalPhysicalRecordLocator::decode(foreign).unwrap())
            .into_result(),
        Err(PhysicalLocatorReadmissionDenial::StoreIdentityMismatch)
    );
    let mut missing = locator.encode();
    missing[32..40].copy_from_slice(&999_u64.to_le_bytes());
    assert_eq!(
        reopened
            .records()
            .readmit_locator(ExternalPhysicalRecordLocator::decode(missing).unwrap())
            .into_result(),
        Err(PhysicalLocatorReadmissionDenial::RecordNotFound)
    );
    let after = reopened.media_counters();
    assert_eq!(
        after.completed_bytes_for(MediaOperationRole::PositionedRead),
        before.completed_bytes_for(MediaOperationRole::PositionedRead)
    );
    reopened.close();
    let free_path = root.join("families/records/free-space/free-space-0000000000000002.manifest");
    let mut damaged = std::fs::read(&free_path).unwrap();
    let final_byte = damaged.len() - 1;
    damaged[final_byte] ^= 1;
    std::fs::write(&free_path, damaged).unwrap();
    let outcome = open_record_store!(media(&root), |durability| PhysicalRecordOpen::new(
        format, access, durability
    ))
    .into_raw();
    let TransitionOutcome::Denied(denial) = outcome else {
        panic!("damaged free-space truth must deny open")
    };
    assert_eq!(
        denial.reason(),
        worth_store::physical_runtime::RecordBootstrapDenial::FreeSpaceManifestDamaged
    );
    denial.into_runtime().close();
}

#[test]
fn locator_readmission_damage_revokes_the_shared_serving_authority() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, access) = dense_configuration(4);
    let serving = success(initialize_record_store!(media(&root), |durability| {
        PhysicalRecordInitialization::new(format, placement, access, durability)
    }));
    let published = serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"locator truth".as_slice()]).unwrap(),
            placement,
        )
        .unwrap();
    let locator = ExternalPhysicalRecordLocator::new(
        serving.store_identity(),
        published.record_id(0).unwrap(),
    );
    serving.close();

    let block_path =
        root.join("families/records/roots/root-0000000000000002-block-0000000000000001.manifest");
    let mut damaged = std::fs::read(&block_path).unwrap();
    let final_byte = damaged.len() - 1;
    damaged[final_byte] ^= 1;
    std::fs::write(&block_path, damaged).unwrap();

    let reopened = success(open_record_store!(media(&root), |durability| {
        PhysicalRecordOpen::new(format, access, durability)
    }));
    let invalidations_before = reopened
        .physical_signal_observation()
        .unwrap()
        .aspect_invalidation_count();
    let error = match reopened.records().open_external(
        locator,
        RecordReadLimits::new(RecordByteLimit::new(64).unwrap()),
    ) {
        Ok(_) => panic!("damaged locator truth must not construct a read session"),
        Err(error) => error,
    };
    assert_eq!(error.denial(), RecordReadDenial::ArtifactDamaged);
    assert_eq!(
        reopened
            .physical_signal_observation()
            .unwrap()
            .aspect_invalidation_count(),
        invalidations_before + 1,
        "semantic rejection after a completed range read must invalidate its exact projection"
    );
    let failed_identity = error
        .observation()
        .last_physical_work()
        .expect("semantic damage retains the rejected physical read identity");
    let failed = reopened
        .physical_work_observer()
        .causal()
        .records()
        .iter()
        .find(|record| record.identity() == failed_identity)
        .copied()
        .expect("semantic damage joins to a causal physical settlement");
    assert_eq!(failed.effect_fate(), PhysicalWorkEffectFate::ReadCompleted);
    assert!(failed.backend_operation().is_some());
    assert_mutation_fenced(&reopened, placement);
    assert_eq!(
        reopened.close().records().posture(),
        RecordServingTerminalPosture::InspectionRequired
    );

    let reopened = success(open_record_store!(media(&root), |durability| {
        PhysicalRecordOpen::new(format, access, durability)
    }));
    assert_eq!(
        reopened.records().readmit_locator(locator).into_result(),
        Err(PhysicalLocatorReadmissionDenial::CurrentRootUnavailable)
    );
    assert_mutation_fenced(&reopened, placement);
    assert_eq!(
        reopened.close().records().posture(),
        RecordServingTerminalPosture::InspectionRequired
    );
}

fn assert_mutation_fenced(
    serving: &worth_store::physical_runtime::ServingPhysicalRuntime,
    placement: worth_store::physical_runtime::AdmittedRecordPlacementPolicy,
) {
    assert!(matches!(
        serving.record_submission().append_batch(
            RecordAppendBatch::try_from_iter([b"must stay sealed".as_slice()]).unwrap(),
            placement,
        ),
        Err(RecordAppendError::Denied(
            RecordAppendDenial::ServingRequiresInspection
        ))
    ));
}

#[test]
fn validly_framed_extra_free_space_claim_is_not_accepted_as_truth() {
    let parent = tempfile::tempdir().unwrap();
    let root = parent.path().join("store");
    let (format, placement, access) = dense_configuration(4);
    let serving = success(initialize_record_store!(media(&root), |durability| {
        PhysicalRecordInitialization::new(format, placement, access, durability)
    }));
    serving
        .record_submission()
        .append_batch(
            RecordAppendBatch::try_from_iter([b"stable".as_slice()]).unwrap(),
            placement,
        )
        .unwrap();
    serving.close();

    let free = super::manifest_fixture::decode_free_space_tree(&root, 2, format.declaration(), 128);
    let mut entries = free.entries;
    entries.push(
        RecordFreeSpaceManifestEntry::new(RecordAllocationClass::InlinePage, 999, 1, 1, 1).unwrap(),
    );
    entries.sort_by_key(|entry| worth_store_physical_format::FreeSpaceKey::from(*entry));
    overwrite_free_space_root_block(&root, format.declaration(), &free.header, entries);

    let reopened = success(open_record_store!(media(&root), |durability| {
        PhysicalRecordOpen::new(format, access, durability)
    }));
    assert!(matches!(
        reopened.record_submission().append_batch(
            RecordAppendBatch::try_from_iter([b"must inspect the tree".as_slice()]).unwrap(),
            placement,
        ),
        Err(RecordAppendError::Denied(
            RecordAppendDenial::PublishedLayoutDamaged
        ))
    ));
    reopened.close();
}

#[test]
fn altered_free_range_and_generation_cannot_be_readmitted_as_authority() {
    for (case, count_delta, generation_delta) in [("range", 1, 0), ("generation", 0, 1)] {
        let parent = tempfile::tempdir().unwrap();
        let root = parent.path().join(case);
        let (format, placement, access) = dense_configuration(4);
        let serving = success(initialize_record_store!(media(&root), |durability| {
            PhysicalRecordInitialization::new(format, placement, access, durability)
        }));
        serving
            .record_submission()
            .append_batch(
                RecordAppendBatch::try_from_iter([b"stable".as_slice()]).unwrap(),
                placement,
            )
            .unwrap();
        serving.close();

        let free =
            super::manifest_fixture::decode_free_space_tree(&root, 2, format.declaration(), 128);
        let mut entries = free.entries;
        let inline = entries
            .iter_mut()
            .find(|entry| entry.class() == RecordAllocationClass::InlinePage)
            .unwrap();
        *inline = RecordFreeSpaceManifestEntry::new(
            inline.class(),
            inline.owner(),
            inline.first_unallocated(),
            inline.unallocated_count() + count_delta,
            inline.generation() + generation_delta,
        )
        .unwrap();
        overwrite_free_space_root_block(&root, format.declaration(), &free.header, entries);

        let reopened = success(open_record_store!(media(&root), |durability| {
            PhysicalRecordOpen::new(format, access, durability)
        }));
        assert!(
            matches!(
                reopened.record_submission().append_batch(
                    RecordAppendBatch::try_from_iter([b"tree damage".as_slice()]).unwrap(),
                    placement,
                ),
                Err(RecordAppendError::Denied(
                    RecordAppendDenial::PublishedLayoutDamaged
                ))
            ),
            "altered {case} authority must deny allocation planning"
        );
        reopened.close();
    }
}

fn overwrite_free_space_root_block(
    root: &std::path::Path,
    format: worth_store_physical_format::PhysicalRecordFormatDeclaration,
    header: &worth_store_physical_format::DurableFreeSpaceManifestHeader,
    entries: Vec<RecordFreeSpaceManifestEntry>,
) {
    let reference = header.root().expect("the specimen has a free-space root");
    assert_eq!(
        reference.level(),
        0,
        "the focused specimen must have one leaf"
    );
    let block = PhysicalFreeSpaceMembershipBlock::leaf(
        header.tree_identity(),
        reference.generation(),
        reference.block(),
        entries,
        header.node_capacity(),
    )
    .unwrap();
    std::fs::write(
        root.join(format!(
            "families/records/free-space/free-space-{:016x}-block-{:016x}.manifest",
            reference.generation(),
            reference.block(),
        )),
        block.encode(format),
    )
    .unwrap();
}
