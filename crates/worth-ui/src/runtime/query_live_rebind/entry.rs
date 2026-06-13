use crate::runtime::query_binding::WorthUiQueryBindingIdentity;
use crate::runtime::query_live_rebind::{
    WorthUiQueryBindingDriftDenial, WorthUiQueryBindingPreservation, WorthUiQueryBindingRebind,
    WorthUiQueryBindingRetirement,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiQueryLiveRebindOutcome {
    Preserve(WorthUiQueryBindingPreservation),
    Rebind(WorthUiQueryBindingRebind),
    Retire(WorthUiQueryBindingRetirement),
    Deny(WorthUiQueryBindingDriftDenial),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryLiveRebindEntry {
    identity: WorthUiQueryBindingIdentity,
    outcome: WorthUiQueryLiveRebindOutcome,
}

impl WorthUiQueryLiveRebindEntry {
    pub(crate) fn new(
        identity: WorthUiQueryBindingIdentity,
        outcome: WorthUiQueryLiveRebindOutcome,
    ) -> Self {
        Self { identity, outcome }
    }

    pub fn identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.identity
    }

    pub fn outcome(&self) -> &WorthUiQueryLiveRebindOutcome {
        &self.outcome
    }
}
