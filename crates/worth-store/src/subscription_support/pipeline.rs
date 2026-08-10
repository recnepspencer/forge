mod action_consequence;
mod base;
mod compatibility;
mod counters;
mod maintenance;
mod operational_verdict_translation;
mod portability;
mod program_path;
mod retention;

use super::{SubscriptionSupportCatalog, SubscriptionSupportCounterSnapshot};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportPublicationPipeline {
    catalog: SubscriptionSupportCatalog,
    counters: SubscriptionSupportCounterSnapshot,
}

impl Default for SubscriptionSupportPublicationPipeline {
    fn default() -> Self {
        Self::first_ship()
    }
}

impl SubscriptionSupportPublicationPipeline {
    pub fn first_ship() -> Self {
        Self {
            catalog: SubscriptionSupportCatalog::first_ship(),
            counters: SubscriptionSupportCounterSnapshot::default(),
        }
    }
}
