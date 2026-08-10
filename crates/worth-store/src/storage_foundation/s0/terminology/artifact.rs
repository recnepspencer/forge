use super::super::artifacts::{
    S0ArtifactEnvelopeMetadata, S0ArtifactValidationCostSurface, S0_ARTIFACT_SCHEMA_VERSION,
};
use super::super::evidence::{S0ArtifactKind, S0StableDigest};
use super::phrase_finding::TerminologyPhraseFinding;
use super::raw_schema::RawTerminologyRiskReport;
use super::risk_report::TerminologyRiskReport;
use super::validation::TerminologyCleanupRejection;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct S0ValidatedTerminologyRiskReportArtifact {
    report: TerminologyRiskReport,
    validation_cost: S0ArtifactValidationCostSurface,
}

impl S0ValidatedTerminologyRiskReportArtifact {
    pub fn report(&self) -> &TerminologyRiskReport {
        &self.report
    }

    pub fn validation_cost(&self) -> &S0ArtifactValidationCostSurface {
        &self.validation_cost
    }
}

impl TerminologyRiskReport {
    pub fn envelope(&self) -> &S0ArtifactEnvelopeMetadata {
        &self.envelope
    }

    pub fn rows(&self) -> &[TerminologyPhraseFinding] {
        &self.rows
    }

    pub fn scan_digest(&self) -> &S0StableDigest {
        &self.scan_digest
    }

    pub fn to_canonical_json_bytes(&self) -> Result<Vec<u8>, TerminologyCleanupRejection> {
        serde_json::to_vec_pretty(self)
            .map_err(|_| TerminologyCleanupRejection::SerializationFailed)
    }

    pub fn validate_canonical_json_bytes(
        bytes: &[u8],
    ) -> Result<S0ValidatedTerminologyRiskReportArtifact, TerminologyCleanupRejection> {
        let raw = serde_json::from_slice::<RawTerminologyRiskReport>(bytes)
            .map_err(|_| TerminologyCleanupRejection::NonParseable)?;
        if raw.envelope.schema_version != S0_ARTIFACT_SCHEMA_VERSION {
            return Err(TerminologyCleanupRejection::SchemaVersionMismatch);
        }
        if raw.envelope.artifact_kind != S0ArtifactKind::TerminologyRiskReport {
            return Err(TerminologyCleanupRejection::ArtifactKindMismatch);
        }
        let roadmap_parent_digest = S0StableDigest::new(raw.envelope.roadmap_parent_digest)
            .map_err(|_| TerminologyCleanupRejection::InvalidDigest)?;
        let expected_digest = S0StableDigest::new(raw.envelope.deterministic_digest)
            .map_err(|_| TerminologyCleanupRejection::InvalidDigest)?;
        let rows = raw
            .rows
            .into_iter()
            .map(super::raw_schema::RawTerminologyPhraseFinding::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let scan_digest = S0StableDigest::new(raw.scan_digest)
            .map_err(|_| TerminologyCleanupRejection::InvalidDigest)?;
        let report = Self::new(
            raw.envelope.source_revision,
            roadmap_parent_digest,
            raw.envelope.generated_by,
            raw.envelope.nondeterministic_metadata.into_validated()?,
            rows,
            scan_digest,
        )?;
        let row_count = report.rows().len() as u64;
        if report.envelope().deterministic_digest() != &expected_digest {
            return Err(TerminologyCleanupRejection::DeterministicDigestMismatch);
        }
        let canonicalized_row_byte_count = serde_json::to_vec(report.rows())
            .map_err(|_| TerminologyCleanupRejection::SerializationFailed)?
            .len() as u64;
        Ok(S0ValidatedTerminologyRiskReportArtifact {
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
