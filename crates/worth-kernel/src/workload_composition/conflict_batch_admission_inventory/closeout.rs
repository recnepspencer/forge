use std::collections::BTreeSet;
use std::path::PathBuf;

use super::catalog::{
    current_conflict_batch_admission_rows, required_conflict_batch_admission_surfaces,
};
use super::counters::ConflictBatchAdmissionInventoryCounters;
use super::cut_line::ConflictBatchAdmissionCutLine;
use super::discovery::{
    ConflictBatchAdmissionDiscoveredSurface, ConflictBatchAdmissionDiscoveryReport,
    ConflictBatchAdmissionReconciliation,
};
use super::error::ConflictBatchAdmissionInventoryError;
use super::row::{ConflictBatchAdmissionInventoryRow, ConflictBatchAdmissionSurfaceIdentity};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictBatchAdmissionInventory {
    rows: Vec<ConflictBatchAdmissionInventoryRow>,
    counters: ConflictBatchAdmissionInventoryCounters,
    cut_line: ConflictBatchAdmissionCutLine,
    scanned_file_count: usize,
    discovered_surface_count: usize,
    unclassified_surface_count: usize,
}

pub fn current_conflict_batch_admission_inventory(
) -> Result<ConflictBatchAdmissionInventory, ConflictBatchAdmissionInventoryError> {
    ConflictBatchAdmissionInventory::from_rows_for_validation(
        current_conflict_batch_admission_rows()?,
    )?
    .with_default_workspace_closeout()
}

impl ConflictBatchAdmissionInventory {
    pub fn rows(&self) -> &[ConflictBatchAdmissionInventoryRow] {
        &self.rows
    }

    pub const fn counters(&self) -> &ConflictBatchAdmissionInventoryCounters {
        &self.counters
    }

    pub const fn cut_line(&self) -> &ConflictBatchAdmissionCutLine {
        &self.cut_line
    }

    pub const fn unclassified_count(&self) -> usize {
        self.unclassified_surface_count
    }

    pub const fn keep_disposition_count(&self) -> usize {
        0
    }

    pub const fn scanned_file_count(&self) -> usize {
        self.scanned_file_count
    }

    pub const fn discovered_surface_count(&self) -> usize {
        self.discovered_surface_count
    }

    pub fn row_for_surface(
        &self,
        surface: ConflictBatchAdmissionSurfaceIdentity,
    ) -> Option<&ConflictBatchAdmissionInventoryRow> {
        self.rows
            .iter()
            .find(|row| row.surface_identity() == surface)
    }

    pub(crate) fn from_rows_for_validation(
        rows: Vec<ConflictBatchAdmissionInventoryRow>,
    ) -> Result<Self, ConflictBatchAdmissionInventoryError> {
        validate_rows(&rows)?;
        let counters = ConflictBatchAdmissionInventoryCounters::from_rows(&rows);
        let cut_line = ConflictBatchAdmissionCutLine::from_counts(
            counters,
            required_conflict_batch_admission_surfaces().len(),
            0,
            0,
            0,
            0,
        );
        Ok(Self {
            rows,
            counters,
            cut_line,
            scanned_file_count: 0,
            discovered_surface_count: 0,
            unclassified_surface_count: 0,
        })
    }

    fn with_default_workspace_closeout(
        mut self,
    ) -> Result<Self, ConflictBatchAdmissionInventoryError> {
        let roots = default_workspace_scan_roots();
        let discovery = ConflictBatchAdmissionDiscoveryReport::scan_roots(&roots)?;
        let reconciliation =
            ConflictBatchAdmissionReconciliation::from_inventory_and_discovery(&self, &discovery)?;
        self.scanned_file_count = discovery.scanned_file_count();
        self.discovered_surface_count = discovery.discovered_surfaces().len();
        self.unclassified_surface_count = reconciliation.unclassified_surfaces().len();
        self.cut_line = ConflictBatchAdmissionCutLine::from_counts(
            self.counters,
            required_conflict_batch_admission_surfaces().len(),
            0,
            0,
            self.discovered_surface_count,
            self.unclassified_surface_count,
        );
        Ok(self)
    }

    pub(crate) fn contains_discovered_surface(
        &self,
        discovered: &ConflictBatchAdmissionDiscoveredSurface,
    ) -> bool {
        self.rows.iter().any(|row| {
            discovered.path_matches(row.source_path())
                && discovered.surface_matches(row.surface_name())
        })
    }
}

fn default_workspace_scan_roots() -> [PathBuf; 3] {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("worth-kernel should live under workspace/crates/worth-kernel")
        .to_path_buf();
    [
        workspace_root.join("crates/worth-kernel/src/workload_composition"),
        workspace_root.join("crates/worth-topo/src"),
        workspace_root.join("crates/worth-spatial/src"),
    ]
}

fn validate_rows(
    rows: &[ConflictBatchAdmissionInventoryRow],
) -> Result<(), ConflictBatchAdmissionInventoryError> {
    let mut surfaces = BTreeSet::new();
    for row in rows {
        if !surfaces.insert(row.surface_identity()) {
            return Err(ConflictBatchAdmissionInventoryError::DuplicateSurface(
                row.surface_identity(),
            ));
        }
    }

    for required in required_conflict_batch_admission_surfaces() {
        if !surfaces.contains(required) {
            return Err(ConflictBatchAdmissionInventoryError::MissingRequiredSurface(*required));
        }
    }
    Ok(())
}
