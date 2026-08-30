use std::num::NonZeroU64;

use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    FilesystemAccessPosture, FilesystemMediaAdmission, PhysicalRuntimeAdmission, PhysicalStore,
    QualifiedRecoveryFilesystemMedia,
};
use worth_store_physical_format::{
    CheckpointRootBasis, CheckpointStreamEncoder, CheckpointWalSourceRange,
    PhysicalCheckpointIdentity, PhysicalCheckpointSource,
};
use worth_store_physical_integrity::{
    validate_checkpoint_stream_header, CheckpointStreamHeaderIntegrityValidation,
    CheckpointStreamHeaderScopeIdentity, PhysicalArtifactScope, PhysicalByteRange,
    UntrustedPhysicalArtifact,
};

use super::super::admitted_artifact::IntegrityAdmittedRecoveryArtifact;
use super::super::{RecoveryIntegrityIngressCounters, RecoveryIntegrityIngressRejection};

#[test]
fn checkpoint_record_binds_a_borrowed_range_of_the_c4_observation() {
    let parent = tempfile::tempdir().expect("test parent");
    let root = parent.path().join("checkpoint-source-binding");
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
    let identity = PhysicalCheckpointIdentity::new(store, NonZeroU64::new(7).unwrap());
    let source = PhysicalCheckpointSource::concurrent(
        identity,
        CheckpointWalSourceRange::new(10, 20).unwrap(),
        CheckpointRootBasis::new(3, 4),
        5,
    );
    let (_, header) = CheckpointStreamEncoder::begin(source);
    let mut checkpoint = header.clone();
    checkpoint.extend_from_slice(b"bounded trailing record bytes");
    let families = root.join("families");
    std::fs::create_dir_all(&families).unwrap();
    std::fs::write(families.join("checkpoint.current"), checkpoint).unwrap();

    let media = QualifiedRecoveryFilesystemMedia::qualify_existing(&root)
        .unwrap()
        .admit_persisted_store()
        .unwrap();
    let mut discovery = media.bounded_discovery(3, 4096).unwrap();
    let observed_a = discovery.read_current_checkpoint(4096).unwrap();
    let observed_b = discovery.read_current_checkpoint(4096).unwrap();
    let scope = PhysicalArtifactScope::checkpoint_stream_header(
        CheckpointStreamHeaderScopeIdentity::staged(store),
        PhysicalByteRange::new(0, header.len() as u64).unwrap(),
    );
    let validated = validate_header(&observed_a, &header, scope);
    let mut counters = RecoveryIntegrityIngressCounters::default();
    let admitted = IntegrityAdmittedRecoveryArtifact::bind_checkpoint_stream_header(
        &observed_a,
        PhysicalByteRange::new(0, header.len() as u64).unwrap(),
        validated,
        &mut counters,
    );
    let IntegrityAdmittedRecoveryArtifact::CheckpointStreamHeader(admitted) =
        admitted.into_outcome().unwrap()
    else {
        panic!("checkpoint header admission routed to the wrong family")
    };
    assert_eq!(
        admitted.project(&mut counters).checkpoint_identity,
        identity
    );

    let validated = validate_header(&observed_a, &header, scope);
    let copied_read = IntegrityAdmittedRecoveryArtifact::bind_checkpoint_stream_header(
        &observed_b,
        PhysicalByteRange::new(0, header.len() as u64).unwrap(),
        validated,
        &mut counters,
    );
    assert!(matches!(
        copied_read.into_outcome(),
        Err(RecoveryIntegrityIngressRejection::SourceIncarnationMismatch)
    ));

    let validated = validate_header(&observed_a, &header, scope);
    let shifted_range = IntegrityAdmittedRecoveryArtifact::bind_checkpoint_stream_header(
        &observed_a,
        PhysicalByteRange::new(1, header.len() as u64).unwrap(),
        validated,
        &mut counters,
    );
    assert!(matches!(
        shifted_range.into_outcome(),
        Err(RecoveryIntegrityIngressRejection::SourceIncarnationMismatch)
    ));
    assert_eq!(
        (
            counters.attempted,
            counters.admitted,
            counters.rejected_source_binding,
            counters.owner_projection_entries,
        ),
        (3, 1, 2, 1)
    );
    drop(discovery.finish());
}

fn validate_header<'media>(
    observed: &'media worth_store::physical_runtime::ObservedRecoveryArtifact,
    header: &[u8],
    scope: PhysicalArtifactScope,
) -> worth_store_physical_integrity::IntegrityValidatedCheckpointStreamHeader<'media> {
    let observed_header =
        &observed.bytes().expect("checkpoint observation is present")[..header.len()];
    let (validation, _) = validate_checkpoint_stream_header(
        UntrustedPhysicalArtifact::from_bounded_bytes(observed_header),
        scope,
    );
    let CheckpointStreamHeaderIntegrityValidation::Intact(validated) = validation else {
        panic!("canonical checkpoint header must validate")
    };
    validated
}
