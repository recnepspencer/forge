use super::{BridgeAsyncCompletionClass, BridgeAsyncCompletionState};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BridgeAsyncEffectsIndeterminateCompletion {
    state: BridgeAsyncCompletionState,
}

impl BridgeAsyncEffectsIndeterminateCompletion {
    pub(crate) const fn from_owner_observation() -> Self {
        Self {
            state: BridgeAsyncCompletionState::Admitted(
                BridgeAsyncCompletionClass::EffectsIndeterminate,
            ),
        }
    }

    pub const fn state(self) -> BridgeAsyncCompletionState {
        self.state
    }
}
