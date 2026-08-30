use std::num::NonZeroU64;

use sha2::{Digest, Sha256};
use worth_proof::TransitionOutcome;
use worth_store::physical_runtime::{
    FilesystemAccessPosture, FilesystemMediaAdmission, ObservedRecoveryArtifact,
    PhysicalRuntimeAdmission, PhysicalStore, QualifiedRecoveryFilesystemMedia,
};
use worth_store_physical_format::{
    CheckpointBindingCompactionHeader, CheckpointDirtyFrameBasis, CheckpointRootBasis,
    CheckpointStreamEncoder, CheckpointWalSourceRange, PhysicalCheckpointIdentity,
    PhysicalCheckpointSource, RecordArtifactFile, RecordFrameCoordinate,
};
use worth_store_physical_integrity::{
    validate_checkpoint_binding, validate_checkpoint_binding_compaction,
    validate_checkpoint_dirty_basis, validate_checkpoint_footer, validate_checkpoint_stream_header,
    CheckpointBindingCompactionIntegrityValidation, CheckpointBindingIntegrityValidation,
    CheckpointDirtyBasisIntegrityValidation, CheckpointFooterIntegrityValidation,
    CheckpointFooterValidationBasis, CheckpointStreamHeaderIntegrityValidation,
    CheckpointStreamHeaderScopeIdentity, PhysicalArtifactScope, PhysicalByteRange,
    UntrustedPhysicalArtifact,
};

use super::super::{
    IntegrityAdmittedCheckpointProjection, IntegrityAdmittedCheckpointStream,
    IntegrityAdmittedRecoveryArtifact, RecoveryIntegrityIngressCounters,
    RecoveryIntegrityIngressRejection,
};

#[derive(Clone, Copy)]
struct CheckpointRecordLayout {
    header_scope: PhysicalArtifactScope,
    header_range: PhysicalByteRange,
    dirty_scope: PhysicalArtifactScope,
    dirty_range: PhysicalByteRange,
    compaction_scope: PhysicalArtifactScope,
    compaction_range: PhysicalByteRange,
    binding_scope: PhysicalArtifactScope,
    binding_range: PhysicalByteRange,
    footer_scope: PhysicalArtifactScope,
    footer_range: PhysicalByteRange,
}

#[test]
fn complete_checkpoint_projection_uses_only_the_admitted_record_set() {
    let parent = tempfile::tempdir().expect("test parent");
    let root = parent.path().join("checkpoint-stream-projection");
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
    let (mut encoder, header) = CheckpointStreamEncoder::begin(source);
    let dirty_basis = CheckpointDirtyFrameBasis::new(
        RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 0, 64).unwrap(),
        11,
    );
    let dirty_record = encoder.encode_dirty_basis(dirty_basis);
    let compaction_header = CheckpointBindingCompactionHeader::new(1, 20).unwrap();
    let (mut compaction, compaction_record) = encoder.begin_binding_compaction(compaction_header);
    let binding_payload = b"typed-checkpoint-binding";
    let binding_record = compaction.encode_binding_record(binding_payload).unwrap();
    let (_, footer) = compaction.finish();
    let layout = checkpoint_layout(
        store,
        identity,
        header.len(),
        dirty_record.len(),
        compaction_record.len(),
        binding_record.len(),
        footer.len(),
    );
    let mut checkpoint = header;
    checkpoint.extend_from_slice(&dirty_record);
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

    let mut substitution_counters = RecoveryIntegrityIngressCounters::default();
    let [header, dirty, compaction, binding, footer] =
        admit_checkpoint_records(&observed_b, &observed_a, layout, &mut substitution_counters);
    let (
        IntegrityAdmittedRecoveryArtifact::CheckpointStreamHeader(header),
        IntegrityAdmittedRecoveryArtifact::CheckpointDirtyBasis(dirty),
        IntegrityAdmittedRecoveryArtifact::CheckpointBindingCompaction(compaction),
        IntegrityAdmittedRecoveryArtifact::CheckpointBinding(binding),
        IntegrityAdmittedRecoveryArtifact::CheckpointFooter(footer),
    ) = (header, dirty, compaction, binding, footer)
    else {
        panic!("checkpoint record admission routed to a wrong family")
    };
    assert!(matches!(
        IntegrityAdmittedCheckpointStream::assemble(
            header,
            vec![dirty],
            compaction,
            vec![binding],
            footer,
        ),
        Err(RecoveryIntegrityIngressRejection::SourceIncarnationMismatch)
    ));
    assert_eq!(
        (
            substitution_counters.owner_projection_entries,
            substitution_counters.owner_decoder_entries,
        ),
        (0, 0)
    );

    let mut counters = RecoveryIntegrityIngressCounters::default();
    let [header, dirty, compaction, binding, footer] =
        admit_checkpoint_records(&observed_a, &observed_a, layout, &mut counters);
    let (
        IntegrityAdmittedRecoveryArtifact::CheckpointStreamHeader(header),
        IntegrityAdmittedRecoveryArtifact::CheckpointDirtyBasis(dirty),
        IntegrityAdmittedRecoveryArtifact::CheckpointBindingCompaction(compaction),
        IntegrityAdmittedRecoveryArtifact::CheckpointBinding(binding),
        IntegrityAdmittedRecoveryArtifact::CheckpointFooter(footer),
    ) = (header, dirty, compaction, binding, footer)
    else {
        panic!("checkpoint record admission routed to a wrong family")
    };
    let stream = IntegrityAdmittedCheckpointStream::assemble(
        header,
        vec![dirty],
        compaction,
        vec![binding],
        footer,
    )
    .unwrap();
    let projected: IntegrityAdmittedCheckpointProjection<'_> = stream.project(&mut counters);
    assert_eq!(projected.source(), source);
    assert_eq!(projected.checkpoint_identity(), identity);
    assert_eq!(projected.dirty_bases(), &[dirty_basis]);
    assert_eq!(projected.compaction_generation(), 1);
    assert_eq!(projected.wal_cutoff_lsn_exclusive(), 20);
    assert_eq!(projected.bindings().len(), 1);
    assert_eq!(projected.footer().identity(), identity);
    assert_eq!(
        projected.encoded_bytes(),
        observed_a.bytes().unwrap().len() as u64
    );
    assert_eq!(
        projected.bindings()[0].binding.digest(),
        <[u8; 32]>::from(Sha256::digest(binding_payload))
    );
    assert_eq!(
        (
            counters.admitted,
            counters.owner_projection_entries,
            counters.owner_decoder_entries,
        ),
        (5, 5, 1)
    );
    drop(discovery.finish());
}

fn checkpoint_layout(
    store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    identity: PhysicalCheckpointIdentity,
    header_bytes: usize,
    dirty_bytes: usize,
    compaction_bytes: usize,
    binding_bytes: usize,
    footer_bytes: usize,
) -> CheckpointRecordLayout {
    let header_range = PhysicalByteRange::new(0, header_bytes as u64).unwrap();
    let dirty_range = PhysicalByteRange::new(header_bytes as u64, dirty_bytes as u64).unwrap();
    let compaction_range =
        PhysicalByteRange::new((header_bytes + dirty_bytes) as u64, compaction_bytes as u64)
            .unwrap();
    let binding_offset = header_bytes + dirty_bytes + compaction_bytes;
    let binding_range =
        PhysicalByteRange::new(binding_offset as u64, binding_bytes as u64).unwrap();
    let footer_range =
        PhysicalByteRange::new((binding_offset + binding_bytes) as u64, footer_bytes as u64)
            .unwrap();
    CheckpointRecordLayout {
        header_scope: PhysicalArtifactScope::checkpoint_stream_header(
            CheckpointStreamHeaderScopeIdentity::staged(store),
            header_range,
        ),
        header_range,
        dirty_scope: PhysicalArtifactScope::checkpoint_dirty_basis(identity, dirty_range),
        dirty_range,
        compaction_scope: PhysicalArtifactScope::checkpoint_binding_compaction(
            identity,
            compaction_range,
        ),
        compaction_range,
        binding_scope: PhysicalArtifactScope::checkpoint_binding(identity, binding_range),
        binding_range,
        footer_scope: PhysicalArtifactScope::checkpoint_footer(identity, footer_range),
        footer_range,
    }
}

fn admit_checkpoint_records<'media>(
    header_observed: &'media ObservedRecoveryArtifact,
    body_observed: &'media ObservedRecoveryArtifact,
    layout: CheckpointRecordLayout,
    counters: &mut RecoveryIntegrityIngressCounters,
) -> [IntegrityAdmittedRecoveryArtifact<'media>; 5] {
    let header_bytes = header_observed.bytes().unwrap();
    let header_validation = validate_checkpoint_stream_header(
        UntrustedPhysicalArtifact::from_bounded_bytes(
            &header_bytes[..layout.header_range.length() as usize],
        ),
        layout.header_scope,
    )
    .0;
    let CheckpointStreamHeaderIntegrityValidation::Intact(header_validated) = header_validation
    else {
        panic!("checkpoint header must remain intact")
    };
    let body = body_observed.bytes().unwrap();
    let dirty_validation = validate_checkpoint_dirty_basis(
        bounded_range(body, layout.dirty_range),
        layout.dirty_scope,
    )
    .0;
    let CheckpointDirtyBasisIntegrityValidation::Intact(dirty_validated) = dirty_validation else {
        panic!("checkpoint dirty basis must remain intact")
    };
    let compaction_validation = validate_checkpoint_binding_compaction(
        bounded_range(body, layout.compaction_range),
        layout.compaction_scope,
    )
    .0;
    let CheckpointBindingCompactionIntegrityValidation::Intact(compaction_validated) =
        compaction_validation
    else {
        panic!("checkpoint compaction must remain intact")
    };
    let binding_validation = validate_checkpoint_binding(
        bounded_range(body, layout.binding_range),
        layout.binding_scope,
    )
    .0;
    let CheckpointBindingIntegrityValidation::Intact(binding_validated) = binding_validation else {
        panic!("checkpoint binding must remain intact")
    };
    let footer_validation = validate_checkpoint_footer(
        bounded_range(body, layout.footer_range),
        layout.footer_scope,
        CheckpointFooterValidationBasis::new(
            &header_validated,
            std::slice::from_ref(&dirty_validated),
            &compaction_validated,
            std::slice::from_ref(&binding_validated),
        ),
    )
    .0;
    let CheckpointFooterIntegrityValidation::Intact(footer_validated) = footer_validation else {
        panic!("checkpoint footer must remain intact")
    };
    [
        IntegrityAdmittedRecoveryArtifact::bind_checkpoint_stream_header(
            header_observed,
            layout.header_scope,
            layout.header_range,
            CheckpointStreamHeaderIntegrityValidation::Intact(header_validated),
            counters,
        )
        .into_outcome()
        .unwrap(),
        IntegrityAdmittedRecoveryArtifact::bind_checkpoint_dirty_basis(
            body_observed,
            layout.dirty_scope,
            layout.dirty_range,
            CheckpointDirtyBasisIntegrityValidation::Intact(dirty_validated),
            counters,
        )
        .into_outcome()
        .unwrap(),
        IntegrityAdmittedRecoveryArtifact::bind_checkpoint_binding_compaction(
            body_observed,
            layout.compaction_scope,
            layout.compaction_range,
            CheckpointBindingCompactionIntegrityValidation::Intact(compaction_validated),
            counters,
        )
        .into_outcome()
        .unwrap(),
        IntegrityAdmittedRecoveryArtifact::bind_checkpoint_binding(
            body_observed,
            layout.binding_scope,
            layout.binding_range,
            CheckpointBindingIntegrityValidation::Intact(binding_validated),
            counters,
        )
        .into_outcome()
        .unwrap(),
        IntegrityAdmittedRecoveryArtifact::bind_checkpoint_footer(
            body_observed,
            layout.footer_scope,
            layout.footer_range,
            CheckpointFooterIntegrityValidation::Intact(footer_validated),
            counters,
        )
        .into_outcome()
        .unwrap(),
    ]
}

fn bounded_range(bytes: &[u8], range: PhysicalByteRange) -> UntrustedPhysicalArtifact<'_> {
    UntrustedPhysicalArtifact::from_bounded_bytes(
        &bytes[range.offset() as usize..range.end_exclusive() as usize],
    )
}
