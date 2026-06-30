#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiActivationReadiness {
    ready_for_execution_plan_input: bool,
}

impl WorthUiActivationReadiness {
    pub(crate) fn ready_for_execution_plan_input() -> Self {
        Self {
            ready_for_execution_plan_input: true,
        }
    }

    pub fn is_ready_for_execution_plan_input(self) -> bool {
        self.ready_for_execution_plan_input
    }
}
