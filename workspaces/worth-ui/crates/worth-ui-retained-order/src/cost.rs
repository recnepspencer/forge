use std::cell::Cell;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiRetainedOrderCost {
    identity_lookups: u64,
    node_touches: u64,
    rotations: u64,
    live_entries: u64,
    allocated_slots: u64,
    high_water_entries: u64,
}

impl UiRetainedOrderCost {
    pub(crate) fn observed(
        identity_lookups: u64,
        node_touches: u64,
        rotations: u64,
        live_entries: usize,
        allocated_slots: usize,
        high_water_entries: usize,
    ) -> Self {
        Self {
            identity_lookups,
            node_touches,
            rotations,
            live_entries: exact(live_entries),
            allocated_slots: exact(allocated_slots),
            high_water_entries: exact(high_water_entries),
        }
    }

    pub const fn identity_lookups(self) -> u64 {
        self.identity_lookups
    }

    pub const fn node_touches(self) -> u64 {
        self.node_touches
    }

    pub const fn rotations(self) -> u64 {
        self.rotations
    }

    pub const fn live_entries(self) -> u64 {
        self.live_entries
    }

    pub const fn allocated_slots(self) -> u64 {
        self.allocated_slots
    }

    pub const fn high_water_entries(self) -> u64 {
        self.high_water_entries
    }
}

fn exact(value: usize) -> u64 {
    u64::try_from(value).expect("retained-order capacity fits its u64 evidence counter")
}

#[derive(Clone, Copy, Default)]
struct OperationCounters {
    identity_lookups: u64,
    node_touches: u64,
    rotations: u64,
}

pub(crate) struct CostTracker {
    counters: Cell<OperationCounters>,
}

impl CostTracker {
    pub(crate) fn new() -> Self {
        Self {
            counters: Cell::new(OperationCounters::default()),
        }
    }

    pub(crate) fn identity_lookup(&self) {
        self.update(|counters| &mut counters.identity_lookups, "identity lookup");
    }

    pub(crate) fn node_touch(&self) {
        self.update(|counters| &mut counters.node_touches, "node touch");
    }

    pub(crate) fn rotation(&self) {
        self.update(|counters| &mut counters.rotations, "rotation");
    }

    pub(crate) fn take(
        &self,
        live_entries: usize,
        allocated_slots: usize,
        high_water_entries: usize,
    ) -> UiRetainedOrderCost {
        let counters = self.counters.replace(OperationCounters::default());
        UiRetainedOrderCost::observed(
            counters.identity_lookups,
            counters.node_touches,
            counters.rotations,
            live_entries,
            allocated_slots,
            high_water_entries,
        )
    }

    fn update(&self, field: impl FnOnce(&mut OperationCounters) -> &mut u64, name: &str) {
        let mut counters = self.counters.get();
        let value = field(&mut counters);
        *value = value
            .checked_add(1)
            .unwrap_or_else(|| panic!("retained-order {name} counter overflowed"));
        self.counters.set(counters);
    }
}
