use super::{classifier::ClassifiedBlobCloseoutRequest, BlobCloseoutDenial};

#[derive(Debug)]
pub(crate) struct VerifiedBlobCloseoutRequest {
    pub(crate) input: super::BlobCloseoutCertificationInput,
}

pub(crate) fn verify_blob_closeout_request(
    request: ClassifiedBlobCloseoutRequest,
) -> Result<VerifiedBlobCloseoutRequest, BlobCloseoutDenial> {
    let materialized = request.input.materialized_evidence();
    let proof = materialized.proof_summary();
    let lower = materialized.executed_sources().lifecycle_evidence();
    if !proof.checked_execution() || !proof.topology().checked_execution() {
        return Err(BlobCloseoutDenial::ProofTopologyNotChecked);
    }
    if !lower.root_publication_matches_lifecycle_identity()
        || !lower.export_matches_root_and_lifecycle_identity()
    {
        return Err(BlobCloseoutDenial::MissingChunkTreeIdentityBinding);
    }
    if !lower.export_logical_digest_matches_lifecycle()
        || !lower.export_checksum_distinct_from_stored_digest()
    {
        return Err(BlobCloseoutDenial::MissingDigestBinding);
    }
    if !lower.reachability_matches_lifecycle_identity()
        || !lower.reachability_stored_digest_matches_lifecycle()
        || lower.reachability_reference_edges() < lower.executed_topology().chunk_count()
    {
        return Err(BlobCloseoutDenial::MissingReachabilityBinding);
    }
    if !lower.placement_matches_reachability() {
        return Err(BlobCloseoutDenial::MissingPlacementBinding);
    }
    if !lower.placement_matches_lifecycle_scope() {
        return Err(BlobCloseoutDenial::MissingSecurityScopeBinding);
    }
    Ok(VerifiedBlobCloseoutRequest {
        input: request.input,
    })
}
