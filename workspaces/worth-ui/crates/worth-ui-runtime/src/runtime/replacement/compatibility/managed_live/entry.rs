use crate::runtime::replacement::compatibility::managed_live::{
    WorthUiQueryBindingDriftDenial, WorthUiQueryBindingPreservation, WorthUiQueryBindingRebind,
    WorthUiQueryBindingRetirement,
};
use crate::runtime::replacement::query_binding::WorthUiQueryBindingIdentity;

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
