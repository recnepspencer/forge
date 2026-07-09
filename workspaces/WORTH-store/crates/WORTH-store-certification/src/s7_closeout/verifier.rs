use super::{classifier::ClassifiedS7CloseoutRequest, S7CloseoutDenial};

#[derive(Debug)]
pub(crate) struct VerifiedS7CloseoutRequest {
    pub(crate) input: super::S7CloseoutCertificationInput,
}

pub(crate) fn verify_s7_closeout_request(
    request: ClassifiedS7CloseoutRequest,
) -> Result<VerifiedS7CloseoutRequest, S7CloseoutDenial> {
    let materialized = request.input.materialized_evidence();
    let proof = materialized.proof_summary();
    let lower = materialized.executed_sources().lifecycle_evidence();
    if !proof.checked_execution() || !proof.topology().checked_execution() {
        return Err(S7CloseoutDenial::ProofTopologyNotChecked);
    }
    if !lower.root_publication_matches_lifecycle_identity()
        || !lower.export_matches_root_and_lifecycle_identity()
    {
        return Err(S7CloseoutDenial::MissingChunkTreeIdentityBinding);
    }
    if !lower.export_logical_digest_matches_lifecycle()
        || !lower.export_checksum_distinct_from_stored_digest()
    {
        return Err(S7CloseoutDenial::MissingDigestBinding);
    }
    if !lower.reachability_matches_lifecycle_identity()
        || !lower.reachability_stored_digest_matches_lifecycle()
        || lower.reachability_reference_edges() < lower.executed_topology().chunk_count()
    {
        return Err(S7CloseoutDenial::MissingReachabilityBinding);
    }
    if !lower.placement_matches_reachability() {
        return Err(S7CloseoutDenial::MissingPlacementBinding);
    }
    if !lower.placement_matches_lifecycle_scope() {
        return Err(S7CloseoutDenial::MissingSecurityScopeBinding);
    }
    Ok(VerifiedS7CloseoutRequest { input: request.input })
}
