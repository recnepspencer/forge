use std::path::Path;

use crate::graph_read_access_inventory::WorthGraphReadDeletionLedgerItem;

use super::super::stable_identity_digest::stable_digest;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadDeclarationDeletionStatus {
    Deleted,
    CappedResidue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadDeclarationDeletionLedgerRow {
    source_path: String,
    owner: String,
    current_caller: String,
    deletion_trigger: String,
    blocker: Option<String>,
    status: WorthGraphReadDeclarationDeletionStatus,
    row_digest: String,
}

impl WorthGraphReadDeclarationDeletionLedgerRow {
    pub(crate) fn from_deletion_item(
        item: &WorthGraphReadDeletionLedgerItem,
        workspace_root: &Path,
    ) -> Self {
        let identity = item.inventory_row_identity();
        let source_path = identity.source_path().to_string();
        let status = if workspace_root.join(&source_path).exists() {
            WorthGraphReadDeclarationDeletionStatus::CappedResidue
        } else {
            WorthGraphReadDeclarationDeletionStatus::Deleted
        };
        let owner = format!("{:?}", identity.owner());
        let current_caller = identity.current_caller().to_string();
        let deletion_trigger = item.deletion_trigger().to_string();
        let blocker = item.blocker().map(str::to_string);
        let row_digest = stable_digest(&[
            "worth_graph_read_declaration_deletion_ledger_row_v1".to_string(),
            format!("source_path:{source_path}"),
            format!("owner:{owner}"),
            format!("current_caller:{current_caller}"),
            format!("deletion_trigger:{deletion_trigger}"),
            format!("blocker:{}", blocker.as_deref().unwrap_or("none")),
            format!("status:{}", status.digest_part()),
        ]);
        Self {
            source_path,
            owner,
            current_caller,
            deletion_trigger,
            blocker,
            status,
            row_digest,
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn current_caller(&self) -> &str {
        &self.current_caller
    }

    pub fn deletion_trigger(&self) -> &str {
        &self.deletion_trigger
    }

    pub fn blocker(&self) -> Option<&str> {
        self.blocker.as_deref()
    }

    pub const fn status(&self) -> WorthGraphReadDeclarationDeletionStatus {
        self.status
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

impl WorthGraphReadDeclarationDeletionStatus {
    pub const fn digest_part(self) -> &'static str {
        match self {
            Self::Deleted => "deleted",
            Self::CappedResidue => "capped_residue",
        }
    }
}
