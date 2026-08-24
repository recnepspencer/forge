//! Installed reconciliation meaning retained for public inspection.

use worth_query_declaration::facade::application_aftermath::DeclaredReconciliationProcedure;

/// Exact installed procedure selected for external-owner reconciliation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryInstalledReconciliationProcedure {
    procedure_slot: String,
}

impl WorthQueryInstalledReconciliationProcedure {
    pub(crate) fn from_declared(declared: &DeclaredReconciliationProcedure) -> Self {
        Self {
            procedure_slot: declared.procedure_slot().to_owned(),
        }
    }

    pub fn procedure_slot(&self) -> &str {
        &self.procedure_slot
    }
}
