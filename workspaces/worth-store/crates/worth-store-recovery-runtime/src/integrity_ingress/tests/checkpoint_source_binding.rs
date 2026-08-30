use std::num::NonZeroU64;

use sha2::{Digest, Sha256};
use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    FilesystemAccessPosture, FilesystemMediaAdmission, PhysicalRuntimeAdmission, PhysicalStore,
    QualifiedRecoveryFilesystemMedia,
};
use worth_store_physical_format::{
    CheckpointBindingCompactionHeader, CheckpointRootBasis, CheckpointStreamEncoder,
    CheckpointWalSourceRange, PhysicalCheckpointIdentity, PhysicalCheckpointSource,
};
use worth_store_physical_integrity::{
    validate_checkpoint_binding, validate_checkpoint_binding_compaction,
    validate_checkpoint_footer, validate_checkpoint_stream_header,
    CheckpointBindingCompactionIntegrityValidation, CheckpointBindingIntegrityValidation,
    CheckpointFooterIntegrityValidation, CheckpointFooterValidationBasis,
    CheckpointStreamHeaderIntegrityValidation, CheckpointStreamHeaderScopeIdentity,
    PhysicalArtifactScope, PhysicalByteRange, UntrustedPhysicalArtifact,
};

use super::super::admitted_artifact::IntegrityAdmittedRecoveryArtifact;
use super::super::families::checkpoint::{
    CheckpointStreamProjectionDenial, IntegrityAdmittedCheckpointStream,
};
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
    let (encoder, header) = CheckpointStreamEncoder::begin(source);
    let compaction_header = CheckpointBindingCompactionHeader::new(1, 20).unwrap();
    let (mut compaction, compaction_record) = encoder.begin_binding_compaction(compaction_header);
    let binding_payload = b"typed-checkpoint-binding";
    let binding_record = compaction.encode_binding_record(binding_payload).unwrap();
    let (_, footer) = compaction.finish();
    let mut checkpoint = header.clone();
    checkpoint.extend_from_slice(&compaction_record);
    checkpoint.extend_from_slice(&binding_record);
    checkpoint.extend_from_slice(&footer);
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
    let header_range = PhysicalByteRange::new(0, header.len() as u64).unwrap();
    let scope = PhysicalArtifactScope::checkpoint_stream_header(
        CheckpointStreamHeaderScopeIdentity::staged(store),
        header_range,
    );
    let validation = validate_header(&observed_a, header_range, scope);
    let mut counters = RecoveryIntegrityIngressCounters::default();
    let admitted = IntegrityAdmittedRecoveryArtifact::bind_checkpoint_stream_header(
        &observed_a,
        scope,
        header_range,
        validation,
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

    let validation = validate_header(&observed_a, header_range, scope);
    let copied_read = IntegrityAdmittedRecoveryArtifact::bind_checkpoint_stream_header(
        &observed_b,
        scope,
        header_range,
        validation,
        &mut counters,
    );
    assert!(matches!(
        copied_read.into_outcome(),
        Err(RecoveryIntegrityIngressRejection::SourceIncarnationMismatch)
    ));

    let validation = validate_header(&observed_a, header_range, scope);
    let shifted_range = IntegrityAdmittedRecoveryArtifact::bind_checkpoint_stream_header(
        &observed_a,
        scope,
        PhysicalByteRange::new(header_range.offset() + 1, header.len() as u64).unwrap(),
        validation,
        &mut counters,
    );
    assert!(matches!(
        shifted_range.into_outcome(),
        Err(RecoveryIntegrityIngressRejection::ScopeMismatch)
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

    let binding_offset = header.len() + compaction_record.len();
    let binding_range =
        PhysicalByteRange::new(binding_offset as u64, binding_record.len() as u64).unwrap();
    let binding_scope = PhysicalArtifactScope::checkpoint_binding(identity, binding_range);
    let bytes = observed_a.bytes().unwrap();
    let binding_validation = validate_checkpoint_binding(
        UntrustedPhysicalArtifact::from_bounded_bytes(
            &bytes[binding_offset..binding_offset + binding_record.len()],
        ),
        binding_scope,
    )
    .0;
    let mut binding_counters = RecoveryIntegrityIngressCounters::default();
    let binding = IntegrityAdmittedRecoveryArtifact::bind_checkpoint_binding(
        &observed_a,
        binding_scope,
        binding_range,
        binding_validation,
        &mut binding_counters,
    );
    let IntegrityAdmittedRecoveryArtifact::CheckpointBinding(binding) =
        binding.into_outcome().unwrap()
    else {
        panic!("checkpoint binding admission routed to the wrong family")
    };
    let projection = binding.project(&mut binding_counters);
    assert_eq!(
        projection.binding.byte_count(),
        binding_payload.len() as u64
    );
    let expected_binding_digest: [u8; 32] = Sha256::digest(binding_payload).into();
    assert_eq!(projection.binding.digest(), expected_binding_digest);
    assert_eq!(
        (
            binding_counters.admitted,
            binding_counters.owner_projection_entries
        ),
        (1, 1)
    );

    let compaction_offset = header.len();
    let compaction_range =
        PhysicalByteRange::new(compaction_offset as u64, compaction_record.len() as u64).unwrap();
    let compaction_scope =
        PhysicalArtifactScope::checkpoint_binding_compaction(identity, compaction_range);
    let footer_offset = binding_offset + binding_record.len();
    let footer_range = PhysicalByteRange::new(footer_offset as u64, footer.len() as u64).unwrap();
    let footer_scope = PhysicalArtifactScope::checkpoint_footer(identity, footer_range);

    let header_validation = validate_header(&observed_a, header_range, scope);
    let CheckpointStreamHeaderIntegrityValidation::Intact(header_validated) = header_validation
    else {
        panic!("checkpoint header must remain intact")
    };
    let compaction_validation = validate_checkpoint_binding_compaction(
        UntrustedPhysicalArtifact::from_bounded_bytes(
            &bytes[compaction_offset..compaction_offset + compaction_record.len()],
        ),
        compaction_scope,
    )
    .0;
    let CheckpointBindingCompactionIntegrityValidation::Intact(compaction_validated) =
        compaction_validation
    else {
        panic!("checkpoint compaction must remain intact")
    };
    let binding_validation = validate_checkpoint_binding(
        UntrustedPhysicalArtifact::from_bounded_bytes(
            &bytes[binding_offset..binding_offset + binding_record.len()],
        ),
        binding_scope,
    )
    .0;
    let CheckpointBindingIntegrityValidation::Intact(binding_validated) = binding_validation else {
        panic!("checkpoint binding must remain intact")
    };
    let footer_validation = validate_checkpoint_footer(
        UntrustedPhysicalArtifact::from_bounded_bytes(&bytes[footer_offset..]),
        footer_scope,
        CheckpointFooterValidationBasis::new(
            &header_validated,
            &[],
            &compaction_validated,
            std::slice::from_ref(&binding_validated),
        ),
    )
    .0;
    let CheckpointFooterIntegrityValidation::Intact(footer_validated) = footer_validation else {
        panic!("checkpoint footer must remain intact")
    };

    let mut stream_counters = RecoveryIntegrityIngressCounters::default();
    let header = IntegrityAdmittedRecoveryArtifact::bind_checkpoint_stream_header(
        &observed_a,
        scope,
        header_range,
        CheckpointStreamHeaderIntegrityValidation::Intact(header_validated),
        &mut stream_counters,
    )
    .into_outcome()
    .unwrap();
    let compaction = IntegrityAdmittedRecoveryArtifact::bind_checkpoint_binding_compaction(
        &observed_a,
        compaction_scope,
        compaction_range,
        CheckpointBindingCompactionIntegrityValidation::Intact(compaction_validated),
        &mut stream_counters,
    )
    .into_outcome()
    .unwrap();
    let binding = IntegrityAdmittedRecoveryArtifact::bind_checkpoint_binding(
        &observed_a,
        binding_scope,
        binding_range,
        CheckpointBindingIntegrityValidation::Intact(binding_validated),
        &mut stream_counters,
    )
    .into_outcome()
    .unwrap();
    let footer = IntegrityAdmittedRecoveryArtifact::bind_checkpoint_footer(
        &observed_a,
        footer_scope,
        footer_range,
        CheckpointFooterIntegrityValidation::Intact(footer_validated),
        &mut stream_counters,
    )
    .into_outcome()
    .unwrap();
    let (
        IntegrityAdmittedRecoveryArtifact::CheckpointStreamHeader(header),
        IntegrityAdmittedRecoveryArtifact::CheckpointBindingCompaction(compaction),
        IntegrityAdmittedRecoveryArtifact::CheckpointBinding(binding),
        IntegrityAdmittedRecoveryArtifact::CheckpointFooter(footer),
    ) = (header, compaction, binding, footer)
    else {
        panic!("checkpoint record admission routed to a wrong family")
    };
    let stream = IntegrityAdmittedCheckpointStream::assemble(
        header,
        Vec::new(),
        compaction,
        vec![binding],
        footer,
    )
    .unwrap();
    let projected: Result<_, CheckpointStreamProjectionDenial> =
        stream.project(0, 1, &mut stream_counters);
    let verified = projected.unwrap();
    assert_eq!(verified.source(), source);
    assert_eq!(verified.binding_records().len(), 1);
    assert_eq!(
        (
            stream_counters.admitted,
            stream_counters.owner_projection_entries,
            stream_counters.owner_decoder_entries,
        ),
        (4, 1, 1)
    );
    drop(discovery.finish());
}

fn validate_header<'media>(
    observed: &'media worth_store::physical_runtime::ObservedRecoveryArtifact,
    range: PhysicalByteRange,
    scope: PhysicalArtifactScope,
) -> CheckpointStreamHeaderIntegrityValidation<'media> {
    let bytes = observed.bytes().expect("checkpoint observation is present");
    let start = range.offset() as usize;
    let end = range.end_exclusive() as usize;
    validate_checkpoint_stream_header(
        UntrustedPhysicalArtifact::from_bounded_bytes(&bytes[start..end]),
        scope,
    )
    .0
}
