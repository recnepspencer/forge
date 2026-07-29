use crate::declaration::UiAspectName;
use crate::fact_contract::UiConsumedFactContract;

use super::{UiGraphFactConsumerIdentity, UiGraphFactConsumerKey};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphFactIndexEntry {
    consumer_key: UiGraphFactConsumerKey,
    consumer: UiGraphFactConsumerIdentity,
    affected_aspect: Option<UiAspectName>,
    consumed_fact_contract: UiConsumedFactContract,
}

impl UiGraphFactIndexEntry {
    pub(crate) fn new(
        consumer_key: UiGraphFactConsumerKey,
        consumer: UiGraphFactConsumerIdentity,
        affected_aspect: Option<UiAspectName>,
        consumed_fact_contract: UiConsumedFactContract,
    ) -> Self {
        Self {
            consumer_key,
            consumer,
            affected_aspect,
            consumed_fact_contract,
        }
    }

    pub const fn consumer_key(&self) -> &UiGraphFactConsumerKey {
        &self.consumer_key
    }

    pub const fn consumer(&self) -> UiGraphFactConsumerIdentity {
        self.consumer
    }

    pub const fn affected_aspect(&self) -> Option<&UiAspectName> {
        self.affected_aspect.as_ref()
    }

    pub const fn consumed_fact_contract(&self) -> &UiConsumedFactContract {
        &self.consumed_fact_contract
    }
}
