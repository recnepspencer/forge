use serde::Serialize;

mod row;

pub use row::{DerivedInvalidationDeletionDisposition, DerivedInvalidationDeletionRow};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationDeletionLedger {
    rows: Vec<DerivedInvalidationDeletionRow>,
    ledger_digest: String,
}

impl DerivedInvalidationDeletionLedger {
    pub(crate) fn from_rows(rows: Vec<DerivedInvalidationDeletionRow>) -> Self {
        let mut parts = vec![
            "worth-topo:derived-invalidation-deletion-ledger:v1".to_string(),
            format!("rows:{}", rows.len()),
        ];
        parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
        let ledger_digest = super::super::catalog::catalog_digest(parts);
        Self {
            rows,
            ledger_digest,
        }
    }

    pub fn rows(&self) -> &[DerivedInvalidationDeletionRow] {
        &self.rows
    }

    pub fn ledger_digest(&self) -> &str {
        &self.ledger_digest
    }
}
