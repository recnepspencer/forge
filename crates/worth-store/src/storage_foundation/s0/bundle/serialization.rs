use super::super::artifacts::{
    S0ArtifactEnvelopeMetadata, S0ArtifactValidationCostSurface, S0_ARTIFACT_SCHEMA_VERSION,
};
use super::super::evidence::{S0ArtifactKind, S0RequiredArtifactSet};
use super::aggregate::S0EvidenceBundle;
use super::digests::{failure_digest, stable_digest, S0EvidenceBundleDigestBasis};
use super::raw_schema::{parse_bundle, RawS0EvidenceBundleParts};
use super::validated_artifact::S0ValidatedEvidenceBundleArtifact;
use super::validation::{artifact_spec, S0EvidenceBundleParseRejection};

impl S0EvidenceBundle {
    pub fn to_canonical_json_bytes(&self) -> Result<Vec<u8>, S0EvidenceBundleParseRejection> {
        serde_json::to_vec_pretty(self)
            .map_err(|_| S0EvidenceBundleParseRejection::SerializationFailed)
    }

    pub fn validate_canonical_json_bytes(
        bytes: &[u8],
    ) -> Result<S0ValidatedEvidenceBundleArtifact, S0EvidenceBundleParseRejection> {
        let raw = parse_bundle(bytes)?;
        super::raw_schema::ensure_supported_schema_and_kind(&raw)?;
        let parts = raw.into_validated_parts()?;
        validate_failure_digest(&parts)?;
        validate_provenance(&parts)?;
        validate_artifact_validation(&parts)?;
        let expected_digest = parts.expected_digest.clone();
        let row_count = parts.row_count;
        let recomputed_digest = recompute_bundle_digest(&parts)?;
        let bundle = rebuild_bundle(parts, recomputed_digest);
        let canonicalized_row_byte_count = canonicalized_row_byte_count(&bundle)?;
        reject_digest_mismatch(&bundle, &expected_digest)?;
        Ok(S0ValidatedEvidenceBundleArtifact {
            bundle,
            validation_cost: S0ArtifactValidationCostSurface::new(
                bytes.len() as u64,
                row_count,
                canonicalized_row_byte_count,
                row_count,
            ),
        })
    }
}

fn validate_failure_digest(
    parts: &RawS0EvidenceBundleParts,
) -> Result<(), S0EvidenceBundleParseRejection> {
    let expected_failure_digest = failure_digest(&parts.certification_rows)
        .map_err(|_| S0EvidenceBundleParseRejection::InvalidDigest)?;
    if expected_failure_digest != parts.failure_digest {
        return Err(S0EvidenceBundleParseRejection::FailureDigestMismatch);
    }
    Ok(())
}

fn validate_provenance(
    parts: &RawS0EvidenceBundleParts,
) -> Result<(), S0EvidenceBundleParseRejection> {
    if parts.evidence_provenance.source_revision != parts.source_revision
        || parts.evidence_provenance.roadmap_parent_digest != parts.roadmap_parent_digest
    {
        return Err(S0EvidenceBundleParseRejection::ProvenanceMismatch);
    }
    Ok(())
}

fn validate_artifact_validation(
    parts: &RawS0EvidenceBundleParts,
) -> Result<(), S0EvidenceBundleParseRejection> {
    let expected_artifact_validation = S0RequiredArtifactSet::canonical()
        .validate_present_artifacts(
            parts
                .evidence_provenance
                .artifact_digests()
                .iter()
                .cloned()
                .chain(std::iter::once(artifact_spec(
                    S0ArtifactKind::S0EvidenceBundle,
                    parts.expected_digest.clone(),
                ))),
        );
    if expected_artifact_validation != parts.artifact_validation {
        return Err(S0EvidenceBundleParseRejection::ArtifactValidationMismatch);
    }
    Ok(())
}

fn recompute_bundle_digest(
    parts: &RawS0EvidenceBundleParts,
) -> Result<super::super::evidence::S0StableDigest, S0EvidenceBundleParseRejection> {
    stable_digest(&S0EvidenceBundleDigestBasis {
        schema_version: S0_ARTIFACT_SCHEMA_VERSION,
        artifact_kind: S0ArtifactKind::S0EvidenceBundle,
        source_revision: &parts.source_revision,
        roadmap_parent_digest: &parts.roadmap_parent_digest,
        generated_by: &parts.generated_by,
        certification_rows: &parts.certification_rows,
        artifact_validation: &parts.artifact_validation,
        evidence_provenance: &parts.evidence_provenance,
        staleness_report: &parts.staleness_report,
        regeneration_requirement: &parts.regeneration_requirement,
        accepted_handoff_digest: &parts.accepted_handoff_digest,
        release_claim_report_digest: &parts.release_claim_report_digest,
        complexity_contract_summary_digest: &parts.complexity_contract_summary_digest,
        roadmap_gate_readiness: &parts.roadmap_gate_readiness,
        counter_snapshot: &parts.counter_snapshot,
        failure_digest: &parts.failure_digest,
    })
    .map_err(|_| S0EvidenceBundleParseRejection::InvalidDigest)
}

fn rebuild_bundle(
    parts: RawS0EvidenceBundleParts,
    recomputed_digest: super::super::evidence::S0StableDigest,
) -> S0EvidenceBundle {
    S0EvidenceBundle {
        envelope: S0ArtifactEnvelopeMetadata::new(
            S0ArtifactKind::S0EvidenceBundle,
            parts.source_revision,
            parts.roadmap_parent_digest,
            parts.generated_by,
            recomputed_digest,
            parts.nondeterministic_metadata,
        ),
        certification_rows: parts.certification_rows,
        artifact_validation: parts.artifact_validation,
        evidence_provenance: parts.evidence_provenance,
        staleness_report: parts.staleness_report,
        regeneration_requirement: parts.regeneration_requirement,
        accepted_handoff_digest: parts.accepted_handoff_digest,
        release_claim_report_digest: parts.release_claim_report_digest,
        complexity_contract_summary_digest: parts.complexity_contract_summary_digest,
        roadmap_gate_readiness: parts.roadmap_gate_readiness,
        counter_snapshot: parts.counter_snapshot,
        failure_digest: parts.failure_digest,
    }
}

fn canonicalized_row_byte_count(
    bundle: &S0EvidenceBundle,
) -> Result<u64, S0EvidenceBundleParseRejection> {
    Ok(serde_json::to_vec(bundle.certification_rows())
        .map_err(|_| S0EvidenceBundleParseRejection::SerializationFailed)?
        .len() as u64)
}

fn reject_digest_mismatch(
    bundle: &S0EvidenceBundle,
    expected_digest: &super::super::evidence::S0StableDigest,
) -> Result<(), S0EvidenceBundleParseRejection> {
    if bundle.envelope().deterministic_digest() != expected_digest {
        return Err(S0EvidenceBundleParseRejection::DeterministicDigestMismatch);
    }
    Ok(())
}
