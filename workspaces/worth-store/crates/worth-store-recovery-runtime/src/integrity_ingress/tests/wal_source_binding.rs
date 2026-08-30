use sha2::{Digest, Sha256};
use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    FilesystemAccessPosture, FilesystemMediaAdmission, PhysicalRuntimeAdmission, PhysicalStore,
    QualifiedRecoveryFilesystemMedia,
};
use worth_store_physical_format::{
    wal_frame::{encode_wal_frame_v1, WalFrameV1EncodeRequest, WalSegmentIdentity},
    CurrentPhysicalRecordPlacement, DurableInlineRecordPlacement, PersistedInlineSegmentAllocation,
    PersistedPhysicalDataFrameSubject, PersistedPhysicalRecoveryFrame,
    PersistedPhysicalRecoveryProjection, PersistedPhysicalRecoveryRootState,
    PersistedRecordIdentity, PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId,
    PhysicalRecordSlot, PhysicalSegmentId, RecordArtifactFile, RecordFrameCoordinate,
    RecordSegmentPageManifestEntry,
};
use worth_store_physical_integrity::{
    validate_wal_frame, PhysicalArtifactScope, PhysicalByteRange, UntrustedPhysicalArtifact,
    WalFrameIntegrityValidation,
};

use super::super::admitted_artifact::IntegrityAdmittedRecoveryArtifact;
use super::super::{
    RecoveryIntegrityIngressCounters, RecoveryIntegrityIngressObservationOutcome,
    RecoveryIntegrityIngressRejection,
};

#[test]
fn wal_binding_retains_the_exact_c4_entry_and_frame_range() {
    let parent = tempfile::tempdir().expect("test parent");
    let root = parent.path().join("wal-source-binding");
    let runtime =
        PhysicalStore::admit(PhysicalRuntimeAdmission::new(root.clone()).expect("declared root"))
            .expect("ordinary runtime admission");
    let admission =
        FilesystemMediaAdmission::production(FilesystemAccessPosture::CoordinatedServiceAccount);
    let media = match runtime.try_admit_filesystem_media(admission).into_raw() {
        TransitionOutcome::Success(media) => media,
        _ => panic!("ordinary media initialization failed"),
    };
    let store = media.store_identity();
    let _ = media.close();
    let identity = WalSegmentIdentity::new(1, 2).unwrap();
    let redo = canonical_redo_payload();
    let frame = encode_wal_frame_v1(
        WalFrameV1EncodeRequest::from_segment_identity(identity, 3, 4, b"c9-ingress-wal", &redo)
            .unwrap(),
    );
    let wal = root.join("families").join("wal");
    std::fs::create_dir_all(&wal).unwrap();
    let prefix = b"c4-prefix";
    let mut wal_bytes = prefix.to_vec();
    wal_bytes.extend_from_slice(&frame);
    std::fs::write(wal.join("a.wal"), &wal_bytes).unwrap();
    std::fs::write(wal.join("b.wal"), &wal_bytes).unwrap();

    let media = QualifiedRecoveryFilesystemMedia::qualify_existing(&root)
        .unwrap()
        .admit_persisted_store()
        .unwrap();
    let mut discovery = media.bounded_discovery(4, 4096).unwrap();
    let observed = discovery.read_wal_artifacts(2, 4096).unwrap();
    assert_eq!(observed.len(), 2);
    let frame_range = PhysicalByteRange::new(prefix.len() as u64, frame.len() as u64).unwrap();
    let scope = PhysicalArtifactScope::wal_frame(store, identity, frame_range);
    let validation = validate(&observed[0], frame_range, scope);
    let mut counters = RecoveryIntegrityIngressCounters::default();
    let admitted = IntegrityAdmittedRecoveryArtifact::bind_wal_frame(
        &observed[0],
        scope,
        frame_range,
        validation,
        &mut counters,
    );
    assert_eq!(
        admitted.observation().outcome(),
        RecoveryIntegrityIngressObservationOutcome::Admitted
    );
    let IntegrityAdmittedRecoveryArtifact::WalFrame(admitted) = admitted.into_outcome().unwrap()
    else {
        panic!("WAL admission routed to the wrong family")
    };
    let projection = admitted.project(&mut counters);
    assert_eq!(projection.segment_identity, identity);
    assert_eq!((projection.lsn_start, projection.lsn_end), (3, 4));
    assert_eq!(projection.source_entry_type, observed[0].entry_type());
    assert_eq!(projection.source_name, observed[0].name());
    assert_eq!(projection.redo.byte_count(), redo.len() as u64);
    assert_eq!(projection.redo.digest(), projection.payload_digest);
    let records = projection.redo.interpret(1, &mut counters).unwrap();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].lsn().get(), 3);

    let validation = validate(&observed[0], frame_range, scope);
    let substituted = IntegrityAdmittedRecoveryArtifact::bind_wal_frame(
        &observed[1],
        scope,
        frame_range,
        validation,
        &mut counters,
    );
    assert_eq!(
        substituted.observation().outcome(),
        RecoveryIntegrityIngressObservationOutcome::Rejected(
            RecoveryIntegrityIngressRejection::SourceIncarnationMismatch
        )
    );

    let validation = validate(&observed[0], frame_range, scope);
    let shifted = IntegrityAdmittedRecoveryArtifact::bind_wal_frame(
        &observed[0],
        scope,
        PhysicalByteRange::new(frame_range.offset() + 1, frame.len() as u64).unwrap(),
        validation,
        &mut counters,
    );
    assert_eq!(
        shifted.observation().outcome(),
        RecoveryIntegrityIngressObservationOutcome::Rejected(
            RecoveryIntegrityIngressRejection::ScopeMismatch
        )
    );
    assert_eq!(
        (
            counters.attempted,
            counters.admitted,
            counters.rejected_source_binding
        ),
        (3, 1, 2)
    );
    assert_eq!(counters.owner_projection_entries, 1);
    assert_eq!(counters.owner_decoder_entries, 1);
    drop(discovery.finish());
}

fn canonical_redo_payload() -> Vec<u8> {
    let authority = PhysicalGenerationAuthority::for_canonical_physical_format();
    let segment = PhysicalSegmentId::from_raw(1).unwrap();
    let page_cell = authority
        .page_cell(segment, PhysicalPageId::from_raw(1).unwrap())
        .with_page_generation(PhysicalGeneration::from_raw(1).unwrap());
    let frame_bytes = vec![1; 8];
    let coordinate = RecordFrameCoordinate::new(
        RecordArtifactFile::Segment {
            segment: 1,
            generation: 1,
        },
        0,
        frame_bytes.len() as u32,
    )
    .unwrap();
    let frame = PersistedPhysicalRecoveryFrame::new(
        PersistedPhysicalDataFrameSubject::InlinePage(page_cell),
        coordinate,
        &frame_bytes,
    )
    .unwrap();
    let record = PersistedRecordIdentity::new([1; 16], 1).unwrap();
    let slot = authority
        .slot_cell(
            segment,
            page_cell.page_id(),
            PhysicalRecordSlot::from_raw(1).unwrap(),
        )
        .with_slot_generation(PhysicalGeneration::from_raw(1).unwrap());
    let segment_cell = authority
        .segment_cell(segment)
        .with_segment_generation(PhysicalGeneration::from_raw(1).unwrap());
    let placement =
        DurableInlineRecordPlacement::new(record, segment_cell, page_cell, slot, 4, 4).unwrap();
    let routing = RecordSegmentPageManifestEntry::new(page_cell, segment_cell, 1, 0).unwrap();
    let projection = PersistedPhysicalRecoveryProjection::new(
        1,
        PersistedPhysicalRecoveryRootState::new(
            4096,
            1,
            4,
            vec![PersistedInlineSegmentAllocation::new(segment_cell, 4, 1).unwrap()],
            Some(record),
            Some(segment_cell),
        )
        .unwrap(),
        vec![record],
        vec![frame],
        vec![CurrentPhysicalRecordPlacement::Inline(placement)],
        vec![routing],
        Vec::new(),
    )
    .unwrap();
    let mut target = Vec::new();
    target.push(1);
    target.extend_from_slice(&1_u64.to_le_bytes());
    target.extend_from_slice(&1_u64.to_le_bytes());
    target.extend_from_slice(&1_u64.to_le_bytes());
    target.push(5);
    target.extend_from_slice(&1_u64.to_le_bytes());
    target.extend_from_slice(&1_u64.to_le_bytes());
    target.extend_from_slice(&0_u64.to_le_bytes());
    target.extend_from_slice(&(frame_bytes.len() as u32).to_le_bytes());
    let mut encoded = Vec::new();
    encode_field(&mut encoded, b"store.physical.wal.canonical-redo.v3");
    encoded.extend_from_slice(&1_u64.to_le_bytes());
    encoded.extend_from_slice(&0_u32.to_le_bytes());
    encoded.extend_from_slice(&3_u64.to_le_bytes());
    encoded.extend_from_slice(&1_u64.to_le_bytes());
    encode_field(&mut encoded, &target);
    let digest: [u8; 32] = Sha256::digest(&frame_bytes).into();
    encoded.extend_from_slice(&digest);
    encode_field(&mut encoded, b"redo");
    encode_field(&mut encoded, &projection.encode());
    encoded
}

fn encode_field(target: &mut Vec<u8>, bytes: &[u8]) {
    target.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
    target.extend_from_slice(bytes);
}

fn validate<'media>(
    observed: &'media worth_store::physical_runtime::ObservedWalArtifact,
    range: PhysicalByteRange,
    scope: PhysicalArtifactScope,
) -> WalFrameIntegrityValidation<'media> {
    let bytes = observed
        .bytes()
        .expect("regular WAL entry has bounded bytes");
    let start = range.offset() as usize;
    let end = range.end_exclusive() as usize;
    let input = UntrustedPhysicalArtifact::from_bounded_bytes(&bytes[start..end]);
    validate_wal_frame(input, scope).0
}
