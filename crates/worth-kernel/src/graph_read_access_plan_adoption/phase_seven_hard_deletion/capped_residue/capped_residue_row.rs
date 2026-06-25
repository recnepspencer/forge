use super::super::deletion_proof::{
    WorthGraphReadAccessHardDeletionProofRow, WorthGraphReadAccessHardDeletionStatus,
};
use super::super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessHardDeletionCappedResidueRow {
    source_path: String,
    owner: String,
    blocker: String,
    removal_trigger: String,
    observed_residue_count: usize,
    allowed_residue_count: usize,
    row_digest: String,
}

impl WorthGraphReadAccessHardDeletionCappedResidueRow {
    pub(crate) fn from_deletion_row(
        row: &WorthGraphReadAccessHardDeletionProofRow,
        allowed_residue_count: usize,
    ) -> Option<Self> {
        if row.status() != WorthGraphReadAccessHardDeletionStatus::CappedResidue {
            return None;
        }
        let blocker = row.blocker()?.to_string();
        let observed_residue_count = 1;
        let row_digest = stable_digest(&[
            "worth_graph_read_access_hard_deletion_capped_residue_row_v1".to_string(),
            format!("source_path:{}", row.source_path()),
            format!("owner:{}", row.owner()),
            format!("blocker:{blocker}"),
            format!("removal_trigger:{}", row.removal_trigger()),
            format!("observed:{observed_residue_count}"),
            format!("allowed:{allowed_residue_count}"),
        ]);
        Some(Self {
            source_path: row.source_path().to_string(),
            owner: row.owner().to_string(),
            blocker,
            removal_trigger: row.removal_trigger().to_string(),
            observed_residue_count,
            allowed_residue_count,
            row_digest,
        })
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }

    pub const fn observed_residue_count(&self) -> usize {
        self.observed_residue_count
    }

    pub const fn allowed_residue_count(&self) -> usize {
        self.allowed_residue_count
    }

    pub const fn is_within_cap(&self) -> bool {
        self.observed_residue_count <= self.allowed_residue_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
