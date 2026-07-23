use super::WorthQueryProjectionPromotionCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryLiveProjectionReceipt {
    operational_identity: String,
    resource_name: String,
    settled_identity: String,
    conditional_attempt: u64,
    read_context_identity: String,
    counters: WorthQueryProjectionPromotionCounters,
}

impl WorthQueryLiveProjectionReceipt {
    pub(super) fn new(
        operational_identity: String,
        resource_name: String,
        settled_identity: String,
        conditional_attempt: u64,
        read_context_identity: String,
        counters: WorthQueryProjectionPromotionCounters,
    ) -> Self {
        Self {
            operational_identity,
            resource_name,
            settled_identity,
            conditional_attempt,
            read_context_identity,
            counters,
        }
    }

    pub fn operational_identity(&self) -> &str {
        &self.operational_identity
    }

    pub fn resource_name(&self) -> &str {
        &self.resource_name
    }

    pub fn settled_identity(&self) -> &str {
        &self.settled_identity
    }

    pub fn conditional_attempt(&self) -> u64 {
        self.conditional_attempt
    }

    pub fn read_context_identity(&self) -> &str {
        &self.read_context_identity
    }

    pub fn counters(&self) -> WorthQueryProjectionPromotionCounters {
        self.counters
    }
}
