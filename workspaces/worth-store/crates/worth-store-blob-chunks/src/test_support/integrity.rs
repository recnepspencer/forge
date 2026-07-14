#![cfg_attr(not(test), allow(dead_code, unused_imports))]

use crate::{
    BlobChunkIntegrityProof, BlobChunkSecurityScope, BlobChunkSequenceAdmission, BlobChunkSize,
    BlobChunkingRuleAdmission,
};

use super::identity::blob_scope;
use super::physical::physical_payload_for_bytes;

pub(crate) fn integrity_proof_for_scope(
    scope: BlobChunkSecurityScope,
    bytes: &[u8],
) -> BlobChunkIntegrityProof {
    admitted_sequence_for_scope(scope, bytes)
        .first_chunk()
        .clone()
}

pub(crate) fn admitted_sequence_for_scope(
    scope: BlobChunkSecurityScope,
    bytes: &[u8],
) -> crate::AdmittedBlobChunkSequence {
    let rule = BlobChunkingRuleAdmission::fixed_size(
        BlobChunkSize::from_bytes(bytes.len() as u64).expect("nonempty chunk size"),
    )
    .expect("fixed-size rule should admit");
    BlobChunkSequenceAdmission::start(scope, rule, bytes.len() as u64)
        .expect("sequence should start")
        .push_payload(0, physical_payload_for_bytes(bytes))
        .expect("window should admit into sequence")
        .finish()
        .expect("sequence should finish")
}

pub(crate) fn admitted_multichunk_sequence_for_scope(
    scope: BlobChunkSecurityScope,
    bytes: &[u8],
    chunk_size: u64,
) -> crate::AdmittedBlobChunkSequence {
    let rule = BlobChunkingRuleAdmission::fixed_size(
        BlobChunkSize::from_bytes(chunk_size).expect("nonempty chunk size"),
    )
    .expect("fixed-size rule should admit");
    let mut admission = BlobChunkSequenceAdmission::start(scope, rule, bytes.len() as u64)
        .expect("sequence should start");
    let mut offset = 0_u64;
    for chunk in bytes.chunks(chunk_size as usize) {
        admission = admission
            .push_payload(offset, physical_payload_for_bytes(chunk))
            .expect("window should admit into sequence");
        offset += chunk.len() as u64;
    }
    admission.finish().expect("sequence should finish")
}

pub(crate) fn frontier_for(
    case: &str,
    bytes: &[u8],
    chunk_size: u64,
) -> crate::BlobStreamingContentFrontier {
    use worth_store_security::StoreTenantScope;

    let sequence = admitted_multichunk_sequence_for_scope(
        blob_scope(case, StoreTenantScope::TenantPhysicalBoundary),
        bytes,
        chunk_size,
    );
    crate::BlobStreamingContentFrontier::from_sequence(&sequence)
}
