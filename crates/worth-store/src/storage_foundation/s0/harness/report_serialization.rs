use super::super::artifacts::{S0ArtifactValidationCostSurface, S0_ARTIFACT_SCHEMA_VERSION};
use super::super::evidence::{S0ArtifactKind, S0StableDigest};
use super::raw_schema::RawHarnessMaturityReport;
use super::report::HarnessMaturityReport;
use super::validated_artifact::S0ValidatedHarnessMaturityReportArtifact;
use super::validation::S0HarnessMaturityParseRejection;

impl HarnessMaturityReport {
    pub fn to_canonical_json_bytes(&self) -> Result<Vec<u8>, S0HarnessMaturityParseRejection> {
        serde_json::to_vec_pretty(self)
            .map_err(|_| S0HarnessMaturityParseRejection::SerializationFailed)
    }

    pub fn validate_canonical_json_bytes(
        bytes: &[u8],
    ) -> Result<S0ValidatedHarnessMaturityReportArtifact, S0HarnessMaturityParseRejection> {
        validate_serialized_harness_report(bytes)
    }
}

fn validate_serialized_harness_report(
    bytes: &[u8],
) -> Result<S0ValidatedHarnessMaturityReportArtifact, S0HarnessMaturityParseRejection> {
    let raw = serde_json::from_slice::<RawHarnessMaturityReport>(bytes)
        .map_err(|_| S0HarnessMaturityParseRejection::NonParseable)?;
    if raw.envelope.schema_version != S0_ARTIFACT_SCHEMA_VERSION {
        return Err(S0HarnessMaturityParseRejection::SchemaVersionMismatch);
    }
    if raw.envelope.artifact_kind != S0ArtifactKind::HarnessMaturityReport {
        return Err(S0HarnessMaturityParseRejection::ArtifactKindMismatch);
    }
    let roadmap_parent_digest = S0StableDigest::new(raw.envelope.roadmap_parent_digest)
        .map_err(|_| S0HarnessMaturityParseRejection::InvalidDigest)?;
    let expected_digest = S0StableDigest::new(raw.envelope.deterministic_digest)
        .map_err(|_| S0HarnessMaturityParseRejection::InvalidDigest)?;
    let report = HarnessMaturityReport::new(
        raw.envelope.source_revision,
        roadmap_parent_digest,
        raw.envelope.generated_by,
        raw.envelope.nondeterministic_metadata.into_validated()?,
        raw.rows
            .into_iter()
            .map(super::raw_schema::RawHarnessMaturityRow::into_validated)
            .collect::<Result<Vec<_>, _>>()?,
        raw.evidence_bundle_readiness,
    )?;
    let row_count = report.rows().len() as u64;
    if report.envelope().deterministic_digest() != &expected_digest {
        return Err(S0HarnessMaturityParseRejection::DeterministicDigestMismatch);
    }
    let canonicalized_row_byte_count = serde_json::to_vec(report.rows())
        .map_err(|_| S0HarnessMaturityParseRejection::SerializationFailed)?
        .len() as u64;
    Ok(S0ValidatedHarnessMaturityReportArtifact {
        report,
        validation_cost: S0ArtifactValidationCostSurface::new(
            bytes.len() as u64,
            row_count,
            canonicalized_row_byte_count,
            row_count,
        ),
    })
}
