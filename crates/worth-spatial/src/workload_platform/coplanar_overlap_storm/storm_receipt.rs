use super::storm_counters::CoplanarOverlapStormCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoplanarOverlapStormReceipt {
    storm_digest: String,
    workload_identity: String,
    operator_identity: String,
    counters: CoplanarOverlapStormCounters,
}

impl CoplanarOverlapStormReceipt {
    pub(crate) fn new(
        storm_digest: String,
        workload_identity: String,
        operator_identity: String,
        counters: CoplanarOverlapStormCounters,
    ) -> Self {
        Self {
            storm_digest,
            workload_identity,
            operator_identity,
            counters,
        }
    }

    pub fn storm_digest(&self) -> &str {
        &self.storm_digest
    }

    pub fn workload_identity(&self) -> &str {
        &self.workload_identity
    }

    pub fn operator_identity(&self) -> &str {
        &self.operator_identity
    }

    pub fn counters(&self) -> CoplanarOverlapStormCounters {
        self.counters
    }
}
