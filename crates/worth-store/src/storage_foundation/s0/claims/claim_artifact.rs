use super::super::artifacts::{
    S0ArtifactEnvelopeMetadata, S0ArtifactValidationCostSurface, S0_ARTIFACT_SCHEMA_VERSION,
};
use super::super::evidence::{S0ArtifactKind, S0StableDigest};
use super::claim_raw_schema::RawSemanticPhysicalClaimReport;
use super::claim_report::SemanticPhysicalClaimReport;
use super::claim_validation::S0ClaimReportParseRejection;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct S0ValidatedSemanticPhysicalClaimReportArtifact {
    report: SemanticPhysicalClaimReport,
    validation_cost: S0ArtifactValidationCostSurface,
}

impl S0ValidatedSemanticPhysicalClaimReportArtifact {
    pub fn report(&self) -> &SemanticPhysicalClaimReport {
        &self.report
    }

    pub fn validation_cost(&self) -> &S0ArtifactValidationCostSurface {
        &self.validation_cost
    }
}

impl SemanticPhysicalClaimReport {
    pub fn envelope(&self) -> &S0ArtifactEnvelopeMetadata {
        &self.envelope
    }

    pub fn rows(&self) -> &[super::claim_report_row::SemanticPhysicalClaimReportRow] {
        &self.rows
    }

    pub fn to_canonical_json_bytes(&self) -> Result<Vec<u8>, S0ClaimReportParseRejection> {
        serde_json::to_vec_pretty(self)
            .map_err(|_| S0ClaimReportParseRejection::SerializationFailed)
    }

    pub fn validate_canonical_json_bytes(
        bytes: &[u8],
    ) -> Result<S0ValidatedSemanticPhysicalClaimReportArtifact, S0ClaimReportParseRejection> {
        let raw = serde_json::from_slice::<RawSemanticPhysicalClaimReport>(bytes)
            .map_err(|_| S0ClaimReportParseRejection::NonParseable)?;
        if raw.envelope.schema_version != S0_ARTIFACT_SCHEMA_VERSION {
            return Err(S0ClaimReportParseRejection::SchemaVersionMismatch);
        }
        if raw.envelope.artifact_kind != S0ArtifactKind::SemanticPhysicalClaimReport {
            return Err(S0ClaimReportParseRejection::ArtifactKindMismatch);
        }
        let roadmap_parent_digest = S0StableDigest::new(raw.envelope.roadmap_parent_digest)
            .map_err(|_| S0ClaimReportParseRejection::InvalidDigest)?;
        let expected_digest = S0StableDigest::new(raw.envelope.deterministic_digest)
            .map_err(|_| S0ClaimReportParseRejection::InvalidDigest)?;
        let rows = raw
            .rows
            .into_iter()
            .map(super::claim_raw_schema::RawSemanticPhysicalClaimReportRow::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let report = Self::new(
            raw.envelope.source_revision,
            roadmap_parent_digest,
            raw.envelope.generated_by,
            raw.envelope.nondeterministic_metadata.into_validated()?,
            rows,
        )?;
        let row_count = report.rows().len() as u64;
        if report.envelope().deterministic_digest() != &expected_digest {
            return Err(S0ClaimReportParseRejection::DeterministicDigestMismatch);
        }
        let canonicalized_row_byte_count = serde_json::to_vec(report.rows())
            .map_err(|_| S0ClaimReportParseRejection::SerializationFailed)?
            .len() as u64;
        Ok(S0ValidatedSemanticPhysicalClaimReportArtifact {
            report,
            validation_cost: S0ArtifactValidationCostSurface::new(
                bytes.len() as u64,
                row_count,
                canonicalized_row_byte_count,
                row_count,
            ),
        })
    }
}
