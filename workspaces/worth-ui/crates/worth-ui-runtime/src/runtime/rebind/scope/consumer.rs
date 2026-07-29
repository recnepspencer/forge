use crate::declaration::UiAspectName;
use crate::graph::{UiGraphFactConsumerIdentity, UiGraphFactConsumerKey};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAffectedConsumer {
    key: UiGraphFactConsumerKey,
    predecessor: Option<UiGraphFactConsumerIdentity>,
    candidate: Option<UiGraphFactConsumerIdentity>,
    affected_aspects: Box<[UiAspectName]>,
}

impl UiAffectedConsumer {
    pub(crate) fn new(
        key: UiGraphFactConsumerKey,
        predecessor: Option<UiGraphFactConsumerIdentity>,
        candidate: Option<UiGraphFactConsumerIdentity>,
        affected_aspects: Box<[UiAspectName]>,
    ) -> Self {
        Self {
            key,
            predecessor,
            candidate,
            affected_aspects,
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

    pub fn affected_aspects(&self) -> &[UiAspectName] {
        &self.affected_aspects
    }
}
