use std::collections::BTreeSet;

use crate::logic::runtime::RelationalRuntime;
use crate::publication::cdc::data::{
    NormalizedContinuationProof, SubscriberBoundaryAssessment, SubscriberContractDeclaration,
    SubscriberStreamFailure, SubscriberStreamFailureClass, MAX_NORMALIZED_CONTINUATION_BOUNDARIES,
};
use crate::schema::data::{DescriptorSemanticsVersion, SchemaBoundaryFingerprint};

use super::failures::unsupported_continuation_failure;

pub(super) fn compose_normalized_proof(
    runtime: &RelationalRuntime,
    prior_proof: &NormalizedContinuationProof,
    crossed_boundaries: &[SchemaBoundaryFingerprint],
    descriptor_semantics_version: DescriptorSemanticsVersion,
    subscriber_contract: &SubscriberContractDeclaration,
    boundary_assessments: &[SubscriberBoundaryAssessment],
) -> Result<NormalizedContinuationProof, SubscriberStreamFailure> {
    let mut boundary_fingerprints =
        Vec::with_capacity(prior_proof.boundary_fingerprints().len() + crossed_boundaries.len());
    let mut seen = BTreeSet::new();

    for fingerprint in prior_proof
        .boundary_fingerprints()
        .iter()
        .chain(crossed_boundaries.iter())
        .copied()
    {
        if seen.insert(fingerprint) {
            boundary_fingerprints.push(fingerprint);
        }
    }
    runtime
        .performance_access()
        .count_schema_normalized_descriptor_composition(crossed_boundaries.len());

    if boundary_fingerprints.len() > MAX_NORMALIZED_CONTINUATION_BOUNDARIES {
        return Err(unsupported_continuation_failure(
            SubscriberStreamFailureClass::RenegotiationRequired,
            format!(
                "normalized continuation proof exceeded boundary complexity ceiling of {}",
                MAX_NORMALIZED_CONTINUATION_BOUNDARIES
            ),
            subscriber_contract,
            prior_proof,
            crossed_boundaries,
            boundary_assessments,
            descriptor_semantics_version,
        ));
    }

    Ok(NormalizedContinuationProof::new(
        boundary_fingerprints,
        descriptor_semantics_version,
    ))
}

pub(super) fn normalized_boundary_count_at_failure(
    prior_proof: &NormalizedContinuationProof,
    crossed_boundaries: &[SchemaBoundaryFingerprint],
) -> usize {
    let mut seen = BTreeSet::new();
    for fingerprint in prior_proof
        .boundary_fingerprints()
        .iter()
        .chain(crossed_boundaries.iter())
        .copied()
    {
        seen.insert(fingerprint);
    }
    seen.len()
}
