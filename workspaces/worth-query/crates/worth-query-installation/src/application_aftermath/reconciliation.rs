//! Installed reconciliation meaning retained for public inspection.

use crate::package::WorthQueryPortableInstalledReconciliationProcedureRecord;

/// Exact installed procedure selected for external-owner reconciliation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryInstalledReconciliationProcedure {
    procedure_slot: String,
}

impl WorthQueryInstalledReconciliationProcedure {
    pub(crate) fn from_portable(
        portable: &WorthQueryPortableInstalledReconciliationProcedureRecord,
    ) -> Self {
        Self {
            procedure_slot: portable.procedure_slot().to_owned(),
        }
    }

    pub fn procedure_slot(&self) -> &str {
        &self.procedure_slot
    }
}
