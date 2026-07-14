use super::super::BlobReachabilityStagingIdentity;
use crate::BlobChunkSecurityMetadataWitness;

pub(crate) fn publication_payload_frame_digest(
    staging_identity: &BlobReachabilityStagingIdentity,
) -> String {
    format!(
        "blob-publication:v1:object={}:generation={}:root={}:logical={}:security={}:counter={}",
        staging_identity.object_id().digest().as_str(),
        staging_identity.generation().sequence(),
        staging_identity.chunk_tree_root().digest().as_str(),
        staging_identity.logical_content_digest().digest().as_str(),
        security_receipt_basis(staging_identity.security_metadata()),
        staging_identity.counter_receipt_identity().as_str()
    )
}

fn security_receipt_basis(metadata: BlobChunkSecurityMetadataWitness) -> String {
    let receipt_id = metadata.receipt().receipt_id();
    format!(
        "admission={}:scope={}:progression={}",
        receipt_id.admission_sequence(),
        receipt_id.security_scope_fingerprint(),
        receipt_id.proof_progression_fingerprint()
    )
}
