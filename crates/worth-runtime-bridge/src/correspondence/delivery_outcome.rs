use super::BridgeCorrespondenceDenialKind;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CorrespondenceDeliveryCounters {
    pub(crate) source_load_attempts: usize,
    pub(crate) source_envelopes_loaded: usize,
    pub(crate) allocation_registry_lock_attempts: usize,
    pub(crate) signal_capability_admissions: usize,
    pub(crate) failed_deliveries: usize,
    pub(crate) truth_targets_admitted: usize,
    pub(crate) correspondence_lookups: usize,
    pub(crate) signal_seeds_emitted: usize,
    pub(crate) node_fan_out: usize,
    pub(crate) slots_touched: usize,
}

impl CorrespondenceDeliveryCounters {
    pub const fn zero() -> Self {
        Self {
            source_load_attempts: 0,
            source_envelopes_loaded: 0,
            allocation_registry_lock_attempts: 0,
            signal_capability_admissions: 0,
            failed_deliveries: 0,
            truth_targets_admitted: 0,
            correspondence_lookups: 0,
            signal_seeds_emitted: 0,
            node_fan_out: 0,
            slots_touched: 0,
        }
    }

    pub const fn truth_targets_admitted(self) -> usize {
        self.truth_targets_admitted
    }

    pub const fn source_load_attempts(self) -> usize {
        self.source_load_attempts
    }

    pub const fn source_envelopes_loaded(self) -> usize {
        self.source_envelopes_loaded
    }

    pub const fn allocation_registry_lock_attempts(self) -> usize {
        self.allocation_registry_lock_attempts
    }

    pub const fn signal_capability_admissions(self) -> usize {
        self.signal_capability_admissions
    }

    pub const fn failed_deliveries(self) -> usize {
        self.failed_deliveries
    }

    pub const fn correspondence_lookups(self) -> usize {
        self.correspondence_lookups
    }

    pub const fn signal_seeds_emitted(self) -> usize {
        self.signal_seeds_emitted
    }

    pub const fn node_fan_out(self) -> usize {
        self.node_fan_out
    }

    pub const fn slots_touched(self) -> usize {
        self.slots_touched
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BridgeCorrespondenceDeliveryDenial {
    kind: BridgeCorrespondenceDenialKind,
    counters: CorrespondenceDeliveryCounters,
}

impl BridgeCorrespondenceDeliveryDenial {
    pub(crate) const fn new(
        kind: BridgeCorrespondenceDenialKind,
        counters: CorrespondenceDeliveryCounters,
    ) -> Self {
        Self { kind, counters }
    }

    pub const fn kind(self) -> BridgeCorrespondenceDenialKind {
        self.kind
    }

    pub const fn counters(self) -> CorrespondenceDeliveryCounters {
        self.counters
    }
}
