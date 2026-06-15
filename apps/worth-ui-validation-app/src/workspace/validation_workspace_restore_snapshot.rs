use super::ValidationWorkspaceState;

#[derive(Clone, Debug, PartialEq)]
pub struct ValidationWorkspaceRestoreSnapshot {
    state: ValidationWorkspaceState,
}

impl ValidationWorkspaceRestoreSnapshot {
    pub(crate) fn capture(state: &ValidationWorkspaceState) -> Self {
        Self {
            state: state.clone(),
        }
    }

    pub(crate) fn into_state(self) -> ValidationWorkspaceState {
        self.state
    }
}
