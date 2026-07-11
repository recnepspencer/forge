use super::{S8LayoutProductionOperation, S8LayoutStateMachine};

/// Owner-local case identity sealed into every production transition fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct S8OwnerOutcomeCase {
    machine: S8LayoutStateMachine,
    production_operation: S8LayoutProductionOperation,
    name: &'static str,
}

impl S8OwnerOutcomeCase {
    pub(super) const fn new(
        machine: S8LayoutStateMachine,
        production_operation: S8LayoutProductionOperation,
        name: &'static str,
    ) -> Self {
        Self {
            machine,
            production_operation,
            name,
        }
    }

    pub const fn machine(self) -> S8LayoutStateMachine {
        self.machine
    }
    pub const fn production_operation(self) -> S8LayoutProductionOperation {
        self.production_operation
    }
    pub const fn name(self) -> &'static str {
        self.name
    }
}
