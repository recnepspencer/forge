use super::super::artifacts::{
    S0ArtifactEnvelopeMetadata, S0ArtifactValidationCostSurface, S0_ARTIFACT_SCHEMA_VERSION,
};
use super::super::evidence::{S0ArtifactKind, S0StableDigest};
use super::migration_notes::TestMigrationNotes;
use super::raw_schema::RawTestMigrationNotes;
use super::test_migration_note_row::TestMigrationNoteRow;
use super::validation::S0TestMigrationParseRejection;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct S0ValidatedTestMigrationNotesArtifact {
    report: TestMigrationNotes,
    validation_cost: S0ArtifactValidationCostSurface,
}

impl S0ValidatedTestMigrationNotesArtifact {
    pub fn report(&self) -> &TestMigrationNotes {
        &self.report
    }

    pub fn validation_cost(&self) -> &S0ArtifactValidationCostSurface {
        &self.validation_cost
    }
}

impl TestMigrationNotes {
    pub fn envelope(&self) -> &S0ArtifactEnvelopeMetadata {
        &self.envelope
    }

    pub fn rows(&self) -> &[TestMigrationNoteRow] {
        &self.rows
    }

    pub fn to_canonical_json_bytes(&self) -> Result<Vec<u8>, S0TestMigrationParseRejection> {
        serde_json::to_vec_pretty(self)
            .map_err(|_| S0TestMigrationParseRejection::SerializationFailed)
    }

    pub fn validate_canonical_json_bytes(
        bytes: &[u8],
    ) -> Result<S0ValidatedTestMigrationNotesArtifact, S0TestMigrationParseRejection> {
        let raw = serde_json::from_slice::<RawTestMigrationNotes>(bytes)
            .map_err(|_| S0TestMigrationParseRejection::NonParseable)?;
        if raw.envelope.schema_version != S0_ARTIFACT_SCHEMA_VERSION {
            return Err(S0TestMigrationParseRejection::SchemaVersionMismatch);
        }
        if raw.envelope.artifact_kind != S0ArtifactKind::TestMigrationNotes {
            return Err(S0TestMigrationParseRejection::ArtifactKindMismatch);
        }
        let roadmap_parent_digest = S0StableDigest::new(raw.envelope.roadmap_parent_digest)
            .map_err(|_| S0TestMigrationParseRejection::InvalidDigest)?;
        let expected_digest = S0StableDigest::new(raw.envelope.deterministic_digest)
            .map_err(|_| S0TestMigrationParseRejection::InvalidDigest)?;
        let rows = raw
            .rows
            .into_iter()
            .map(super::raw_schema::RawTestMigrationNoteRow::into_validated)
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
            return Err(S0TestMigrationParseRejection::DeterministicDigestMismatch);
        }
        let canonicalized_row_byte_count = serde_json::to_vec(report.rows())
            .map_err(|_| S0TestMigrationParseRejection::SerializationFailed)?
            .len() as u64;
        Ok(S0ValidatedTestMigrationNotesArtifact {
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
