use crate::obligations::selection::UiSelectedObligation;

use super::dispatch_execution::UiObligationDispatchExecution;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiObligationDispatchEntry {
    selected: UiSelectedObligation,
    execution: UiObligationDispatchExecution,
}

impl UiObligationDispatchEntry {
    pub(crate) fn new(
        selected: UiSelectedObligation,
        execution: UiObligationDispatchExecution,
    ) -> Self {
        Self {
            selected,
            execution,
        }
    }

    pub fn selected(&self) -> &UiSelectedObligation {
        &self.selected
    }

    pub(crate) fn execution(&self) -> UiObligationDispatchExecution {
        self.execution
    }
}
