//! Declared reconciliation procedure for external-owner correction authority.

/// Named reconciliation procedure required when correction authority is
/// `RuntimeWithExternalOwner`.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DeclaredReconciliationProcedure {
    procedure_slot: String,
}

impl DeclaredReconciliationProcedure {
    pub fn new(procedure_slot: impl Into<String>) -> Result<Self, &'static str> {
        let procedure_slot = procedure_slot.into();
        if procedure_slot.trim().is_empty() {
            return Err("empty-reconciliation-procedure");
        }
        Ok(Self { procedure_slot })
    }

    pub fn procedure_slot(&self) -> &str {
        &self.procedure_slot
    }
}
