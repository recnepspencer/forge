use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalSegmentId,
};
use forge_store_physical_isolation::{
    CurrentGenerationPhysicalReference, GenerationCountedPhysicalReference,
};
use forge_store_wal::{
    BlobWalRecordIdentity, BlobWalRecordKind, DurablePublicationDeclaration,
    WalFrameDurablePublicationScope,
};

use crate::lifecycle::generation_registry_test_support::current_authority;
use crate::publication::test_support::publication_inputs_with_bytes_and_chunk_size;
use crate::test_support::{admitted_multichunk_sequence_for_scope, blob_scope};
use crate::{
    BlobChunkSize, BlobChunkingRuleAdmission, BlobResumeSessionAdmitted,
    BlobResumeSessionDeclaration, BlobResumeStoreAuthority, BlobStreamingContentFrontier,
};
use forge_store_security::StoreTenantScope;

pub(crate) fn root_candidate_resume_checkpoint(case: &str) -> crate::BlobResumeCheckpoint {
    let bytes = b"aaaabbbb";
    let chunk_size = 8;
    let (candidate, _reachability, _) =
        publication_inputs_with_bytes_and_chunk_size(case, bytes, chunk_size);
    let sequence = admitted_multichunk_sequence_for_scope(
        blob_scope(case, StoreTenantScope::TenantPhysicalBoundary),
        bytes,
        chunk_size,
    );
    let leaf = sequence.proof_frontier().first_leaf().clone();
    let frontier = BlobStreamingContentFrontier::from_sequence(&sequence);
    let rule =
        BlobChunkingRuleAdmission::fixed_size(BlobChunkSize::from_bytes(chunk_size).unwrap())
            .expect("chunking rule should admit");
    let declaration =
        BlobResumeSessionDeclaration::new(leaf.security_metadata(), rule, bytes.len() as u64)
            .expect("resume declaration should admit");
    let authority = BlobResumeStoreAuthority::from_current_store_authority(current_authority(
        &format!("{case}.resume-authority"),
        "resume",
    ));
    let durable = BlobResumeSessionAdmitted::admit(declaration, authority)
        .start_chunk_append(leaf.ordinal())
        .record_chunk_bytes_durable(
            wal_record(BlobWalRecordKind::ChunkAppend, 2, case),
            bytes.len() as u64,
            physical_reference(case.len() as u16 + 1),
        )
        .expect("resume bytes should be durable");
    let checkpointed = durable
        .admit_chunk_integrity(leaf)
        .expect("resume integrity should admit")
        .checkpoint_frontier(
            frontier,
            wal_record(BlobWalRecordKind::SessionCheckpoint, 3, case),
        )
        .expect("resume frontier should checkpoint");
    checkpointed
        .build_root_candidate(candidate)
        .expect("root candidate should match frontier")
        .export_checkpoint()
}

fn physical_reference(slot: u16) -> CurrentGenerationPhysicalReference {
    let generation = PhysicalGeneration::from_raw(7).expect("generation");
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(
            PhysicalSegmentId::from_raw(1).expect("segment"),
            PhysicalPageId::from_raw(1).expect("page"),
            PhysicalRecordSlot::from_raw(slot).expect("slot"),
        )
        .with_slot_generation(generation);
    GenerationCountedPhysicalReference::from_admitted_reference(
        PhysicalReferenceAuthority::for_canonical_physical_format().admit_page_slot(cell),
    )
    .require_current_generation(generation)
    .expect("current generation reference")
}

fn wal_record(
    kind: BlobWalRecordKind,
    sequence: u64,
    case: &str,
) -> forge_store_wal::BlobWalRecordEnvelope {
    let payload = format!("phase14:{case}:{kind:?}:{sequence}");
    let scope = WalFrameDurablePublicationScope::new(9, 1, sequence, sequence + 1, &payload, 64)
        .expect("wal scope should admit");
    forge_store_wal::BlobWalRecordEnvelope::new(
        BlobWalRecordIdentity::new(sequence, kind).expect("wal identity should admit"),
        DurablePublicationDeclaration::wal_frame(scope),
        payload,
    )
    .expect("wal record should admit")
}
