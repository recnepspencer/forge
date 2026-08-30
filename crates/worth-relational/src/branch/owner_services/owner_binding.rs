use std::sync::Weak;

use crate::runtime::{RelationalRuntime, RelationalRuntimeOwnerBinding, RelationalRuntimeState};

/// Weak entry to the exact runtime state and its real operation-admission gate.
#[derive(Debug, Clone)]
pub(super) struct RelationalOwnerServiceBinding {
    state: Weak<RelationalRuntimeState>,
    lifecycle: RelationalRuntimeOwnerBinding,
}

impl RelationalOwnerServiceBinding {
    pub(super) fn new(
        state: Weak<RelationalRuntimeState>,
        lifecycle: RelationalRuntimeOwnerBinding,
    ) -> Self {
        Self { state, lifecycle }
    }

    pub(super) fn state_is_alive(&self) -> bool {
        self.state.strong_count() != 0
    }

    pub(super) fn admitted_runtime(&self) -> Option<RelationalRuntime> {
        let state = self.state.upgrade()?;
        let operation = self.lifecycle.admit()?;
        Some(RelationalRuntime::admitted(state, operation))
    }
}
