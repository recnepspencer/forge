use std::path::PathBuf;

use super::super::inventory_error::{
    WorthGraphReadAccessInventoryError, WorthGraphReadAccessInventoryErrorKind,
};
use super::super::row::{WorthGraphReadAccessClassification, WorthGraphReadAccessInventoryRow};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadDeletedSourceReport {
    deleted_source_paths: Vec<String>,
    existing_deleted_source_paths: Vec<String>,
}

impl WorthGraphReadDeletedSourceReport {
    pub(super) fn from_rows(
        rows: &[WorthGraphReadAccessInventoryRow],
    ) -> Result<Self, WorthGraphReadAccessInventoryError> {
        let deleted_source_paths = rows
            .iter()
            .filter(|row| {
                row.classification() == WorthGraphReadAccessClassification::DeletionTarget
            })
            .map(|row| row.source_path().to_string())
            .collect::<Vec<_>>();
        let existing_deleted_source_paths = deleted_source_paths
            .iter()
            .filter(|path| workspace_root().join(path).exists())
            .cloned()
            .collect::<Vec<_>>();

        if !existing_deleted_source_paths.is_empty() {
            return Err(WorthGraphReadAccessInventoryError::new(
                WorthGraphReadAccessInventoryErrorKind::DeletedGraphReadSourceStillExists,
            ));
        }

        Ok(Self {
            deleted_source_paths,
            existing_deleted_source_paths,
        })
    }

    pub fn deleted_source_paths(&self) -> &[String] {
        &self.deleted_source_paths
    }

    pub const fn deleted_source_count(&self) -> usize {
        self.deleted_source_paths.len()
    }

    pub fn existing_deleted_source_paths(&self) -> &[String] {
        &self.existing_deleted_source_paths
    }

    pub const fn existing_deleted_source_count(&self) -> usize {
        self.existing_deleted_source_paths.len()
    }
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}
