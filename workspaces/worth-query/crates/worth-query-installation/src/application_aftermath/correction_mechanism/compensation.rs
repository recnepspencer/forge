//! Installed compensation mechanism.

use worth_query_declaration::facade::application_aftermath::DeclaredCompensation;

use super::super::postcondition::InstalledAftermathPostcondition;

/// Installed compensation correction mechanism.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstalledCompensation {
    compensating_operation_slot: String,
    postcondition: InstalledAftermathPostcondition,
}

impl InstalledCompensation {
    pub(crate) fn from_declared(declared: &DeclaredCompensation) -> Self {
        Self {
            compensating_operation_slot: declared.compensating_operation_slot().to_owned(),
            postcondition: InstalledAftermathPostcondition::from_declared(declared.postcondition()),
        }
    }

    pub fn compensating_operation_slot(&self) -> &str {
        &self.compensating_operation_slot
    }

    pub const fn postcondition(&self) -> &InstalledAftermathPostcondition {
        &self.postcondition
    }
}
