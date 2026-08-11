use std::collections::BTreeMap;

use crate::data::handle::NodeId;
use crate::data::temporal::{ClockTick, ReadyTemporalWake, RuntimeClockBasis, TemporalWakeOwner};

#[derive(Debug, Clone, Default)]
pub(crate) struct TemporalLoweringContext {
    runtime_clock_basis: Option<RuntimeClockBasis>,
    ready_wakes_by_owner: BTreeMap<TemporalWakeOwner, ReadyTemporalWake>,
}

impl TemporalLoweringContext {
    pub(crate) fn graph_only() -> Self {
        Self {
            runtime_clock_basis: None,
            ready_wakes_by_owner: BTreeMap::new(),
        }
    }

    pub(crate) fn runtime_clock_basis(runtime_clock_basis: RuntimeClockBasis) -> Self {
        Self {
            runtime_clock_basis: Some(runtime_clock_basis),
            ready_wakes_by_owner: BTreeMap::new(),
        }
    }

    pub(crate) fn with_ready_wake(mut self, wake: ReadyTemporalWake) -> Self {
        self.ready_wakes_by_owner.insert(wake.owner(), wake);
        self
    }

    pub(super) fn runtime_tick_for(
        &self,
        domain: crate::data::temporal::ClockDomain,
    ) -> Option<ClockTick> {
        self.runtime_clock_basis
            .filter(|basis| basis.domain() == domain)
            .map(RuntimeClockBasis::current_tick)
    }

    pub(super) fn current_runtime_tick(&self) -> Option<ClockTick> {
        self.runtime_clock_basis
            .map(RuntimeClockBasis::current_tick)
    }

    pub(super) fn ready_wake_for_node(&self, node: NodeId) -> Option<&ReadyTemporalWake> {
        self.ready_wakes_by_owner
            .get(&TemporalWakeOwner::Node(node))
    }
}
