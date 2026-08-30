use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    FilesystemAccessPosture, FilesystemMediaAdmission, PhysicalRuntimeAdmission, PhysicalStore,
    QualifiedRecoveryFilesystemMedia,
};
use worth_store_physical_format::wal_frame::{
    encode_wal_frame_v1, WalFrameV1EncodeRequest, WalSegmentIdentity,
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
    let frame = encode_wal_frame_v1(
        WalFrameV1EncodeRequest::from_segment_identity(
            identity,
            3,
            4,
            b"c9-ingress-wal",
            b"typed-redo-payload",
        )
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
    assert_eq!(
        projection.redo.byte_count(),
        b"typed-redo-payload".len() as u64
    );
    assert_eq!(projection.redo.digest(), projection.payload_digest);

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
    drop(discovery.finish());
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
