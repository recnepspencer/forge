use super::super::deletion_ledger::WorthGraphReadDeclarationDeletionLedgerRow;
use super::super::stable_identity_digest::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadDeclarationCappedResidueRow {
    source_path: String,
    owner: String,
    blocker: String,
    removal_trigger: String,
    current_count: usize,
    must_not_exceed_count: usize,
    row_digest: String,
}

impl WorthGraphReadDeclarationCappedResidueRow {
    pub(crate) fn from_deletion_row(
        deletion_row: &WorthGraphReadDeclarationDeletionLedgerRow,
        must_not_exceed_count: usize,
    ) -> Option<Self> {
        let blocker = deletion_row.blocker()?.to_string();
        let source_path = deletion_row.source_path().to_string();
        let owner = deletion_row.owner().to_string();
        let removal_trigger = deletion_row.deletion_trigger().to_string();
        let current_count = 1;
        let row_digest = stable_digest(&[
            "worth_graph_read_declaration_capped_residue_row_v1".to_string(),
            format!("source_path:{source_path}"),
            format!("owner:{owner}"),
            format!("blocker:{blocker}"),
            format!("removal_trigger:{removal_trigger}"),
            format!("current_count:{current_count}"),
            format!("must_not_exceed_count:{must_not_exceed_count}"),
        ]);
        Some(Self {
            source_path,
            owner,
            blocker,
            removal_trigger,
            current_count,
            must_not_exceed_count,
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

    pub const fn current_count(&self) -> usize {
        self.current_count
    }

    pub const fn must_not_exceed_count(&self) -> usize {
        self.must_not_exceed_count
    }

    pub const fn is_within_cap(&self) -> bool {
        self.current_count <= self.must_not_exceed_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
