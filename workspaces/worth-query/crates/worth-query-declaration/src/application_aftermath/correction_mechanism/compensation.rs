//! Declared compensation correction mechanism.

use super::super::postcondition::DeclaredAftermathPostcondition;

/// Correction by a forward operation that neutralizes committed effects.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DeclaredCompensation {
    compensating_operation_slot: String,
    postcondition: DeclaredAftermathPostcondition,
}

impl DeclaredCompensation {
    pub fn new(
        compensating_operation_slot: impl Into<String>,
        postcondition: DeclaredAftermathPostcondition,
    ) -> Result<Self, &'static str> {
        let compensating_operation_slot = compensating_operation_slot.into();
        if compensating_operation_slot.trim().is_empty() {
            return Err("empty-compensating-operation");
        }
        match &postcondition {
            DeclaredAftermathPostcondition::ExactPriorTruth => {
                return Err("compensation-rejects-exact-prior-truth");
            }
            DeclaredAftermathPostcondition::InvariantRestored { invariant }
                if invariant.trim().is_empty() =>
            {
                return Err("empty-compensation-postcondition");
            }
            DeclaredAftermathPostcondition::BusinessPostcondition { identity }
                if identity.trim().is_empty() =>
            {
                return Err("empty-compensation-postcondition");
            }
            DeclaredAftermathPostcondition::InvariantRestored { .. }
            | DeclaredAftermathPostcondition::BusinessPostcondition { .. } => {}
        }
        Ok(Self {
            compensating_operation_slot,
            postcondition,
        })
    }

    pub fn compensating_operation_slot(&self) -> &str {
        &self.compensating_operation_slot
    }

    pub const fn postcondition(&self) -> &DeclaredAftermathPostcondition {
        &self.postcondition
    }
}
