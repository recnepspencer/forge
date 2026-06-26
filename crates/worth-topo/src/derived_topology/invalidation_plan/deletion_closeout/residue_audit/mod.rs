use serde::Serialize;

mod row;

pub use row::DerivedInvalidationResidueAuditRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationResidueAudit {
    rows: Vec<DerivedInvalidationResidueAuditRow>,
    audit_digest: String,
}

impl DerivedInvalidationResidueAudit {
    pub(crate) fn from_rows(rows: Vec<DerivedInvalidationResidueAuditRow>) -> Self {
        let mut parts = vec![
            "worth-topo:derived-invalidation-residue-audit:v1".to_string(),
            format!("rows:{}", rows.len()),
        ];
        parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
        let audit_digest = super::super::catalog::catalog_digest(parts);
        Self { rows, audit_digest }
    }

    pub fn rows(&self) -> &[DerivedInvalidationResidueAuditRow] {
        &self.rows
    }

    pub fn audit_digest(&self) -> &str {
        &self.audit_digest
    }
}
