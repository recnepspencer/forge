use std::sync::{Arc, Weak};

use super::{BridgeInstalledConditionalLowering, BridgeOwnedSignalRuntime};

/// Weak observation of the concrete Bridge and Signal resources owned by one
/// conditional runtime at the instant the probe is acquired.
pub struct BridgeConditionalRuntimeLifecycleProbe {
    signal_graph: Option<worth_signal::facade::SignalGraphLifecycleProbe>,
    providers: Vec<Weak<BridgeInstalledConditionalLowering>>,
    managed_clocks: Vec<Weak<()>>,
}

impl BridgeConditionalRuntimeLifecycleProbe {
    pub fn live_signal_graph_count(&self) -> usize {
        self.signal_graph
            .as_ref()
            .is_some_and(worth_signal::facade::SignalGraphLifecycleProbe::is_live)
            .into()
    }

    pub fn live_provider_count(&self) -> usize {
        self.providers
            .iter()
            .filter(|provider| provider.strong_count() != 0)
            .count()
    }

    pub fn live_managed_clock_count(&self) -> usize {
        self.managed_clocks
            .iter()
            .filter(|clock| clock.strong_count() != 0)
            .count()
    }
}

impl BridgeOwnedSignalRuntime {
    pub fn conditional_lifecycle_probe(&self) -> BridgeConditionalRuntimeLifecycleProbe {
        BridgeConditionalRuntimeLifecycleProbe {
            signal_graph: (!self.conditional_lowerings.is_empty()
                || !self.managed_clock_lanes.is_empty())
            .then(|| self.signal_runtime.graph().lifecycle_probe()),
            providers: self
                .conditional_lowerings
                .values()
                .map(Arc::downgrade)
                .collect(),
            managed_clocks: self
                .managed_clock_lanes
                .values()
                .map(|lane| Arc::downgrade(&lane.lifecycle_token))
                .collect(),
        }
    }
}
