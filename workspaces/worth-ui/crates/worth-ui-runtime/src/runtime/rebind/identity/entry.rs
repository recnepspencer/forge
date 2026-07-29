use crate::graph::{UiGraphFactConsumerIdentity, UiGraphFactConsumerKey};

use super::UiIdentityLifecycleDecision;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiIdentityLifecycleEntry {
    key: UiGraphFactConsumerKey,
    predecessor: Option<UiGraphFactConsumerIdentity>,
    candidate: Option<UiGraphFactConsumerIdentity>,
    decision: UiIdentityLifecycleDecision,
}

impl UiIdentityLifecycleEntry {
    pub(crate) const fn new(
        key: UiGraphFactConsumerKey,
        predecessor: Option<UiGraphFactConsumerIdentity>,
        candidate: Option<UiGraphFactConsumerIdentity>,
        decision: UiIdentityLifecycleDecision,
    ) -> Self {
        Self {
            key,
            predecessor,
            candidate,
            decision,
        }
    }

    pub const fn key(&self) -> &UiGraphFactConsumerKey {
        &self.key
    }

    pub const fn predecessor(&self) -> Option<UiGraphFactConsumerIdentity> {
        self.predecessor
    }

    pub const fn candidate(&self) -> Option<UiGraphFactConsumerIdentity> {
        self.candidate
    }

    pub const fn decision(&self) -> UiIdentityLifecycleDecision {
        self.decision
    }
}
