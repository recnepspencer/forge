use super::super::artifacts::{S0ArtifactEnvelopeMetadata, S0NondeterministicMetadata};
use super::super::evidence::S0StableDigest;
use super::test_migration_note_row::TestMigrationNoteRow;
use super::validation::{
    reject_duplicate_rows, require_non_empty, test_migration_notes_digest,
    S0TestMigrationBuildRejection,
};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TestMigrationNotes {
    #[serde(flatten)]
    pub(super) envelope: S0ArtifactEnvelopeMetadata,
    pub(super) rows: Vec<TestMigrationNoteRow>,
}

impl TestMigrationNotes {
    pub fn new(
        source_revision: impl Into<String>,
        roadmap_parent_digest: S0StableDigest,
        generated_by: impl Into<String>,
        nondeterministic_metadata: S0NondeterministicMetadata,
        mut rows: Vec<TestMigrationNoteRow>,
    ) -> Result<Self, S0TestMigrationBuildRejection> {
        let source_revision = require_non_empty(source_revision)?;
        let generated_by = require_non_empty(generated_by)?;
        rows.sort_by(|left, right| left.row_id().cmp(right.row_id()));
        reject_duplicate_rows(&rows)?;
        let deterministic_digest = test_migration_notes_digest(
            &source_revision,
            &roadmap_parent_digest,
            &generated_by,
            &rows,
        )?;
        Ok(Self {
            envelope: S0ArtifactEnvelopeMetadata::new(
                super::super::evidence::S0ArtifactKind::TestMigrationNotes,
                source_revision,
                roadmap_parent_digest,
                generated_by,
                deterministic_digest,
                nondeterministic_metadata,
            ),
            rows,
        })
    }
}
