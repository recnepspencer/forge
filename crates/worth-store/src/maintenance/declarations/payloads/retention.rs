use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetentionMaintenanceDeclaration {
    batch_label: String,
    closure_commit_count: u64,
    declaration_count: u64,
}

impl RetentionMaintenanceDeclaration {
    pub(crate) fn new(
        batch_label: impl Into<String>,
        closure_commit_count: u64,
        declaration_count: u64,
    ) -> Self {
        Self {
            batch_label: batch_label.into(),
            closure_commit_count,
            declaration_count,
        }
    }

    pub fn batch_label(&self) -> &str {
        &self.batch_label
    }

    pub fn closure_commit_count(&self) -> u64 {
        self.closure_commit_count
    }

    pub fn declaration_count(&self) -> u64 {
        self.declaration_count
    }
}
