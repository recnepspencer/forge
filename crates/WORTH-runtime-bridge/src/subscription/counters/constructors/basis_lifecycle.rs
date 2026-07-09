use super::super::BridgeSubscriptionCounters;

impl BridgeSubscriptionCounters {
    pub fn from_basis_binding() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0)
    }

    pub fn from_basis_kind_divergence_rejection() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0)
    }

    pub fn from_basis_resolution_rejection() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0)
    }

    pub fn from_signal_strategy_descriptor() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0)
    }

    pub fn from_admitted_subscription() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0)
    }

    pub fn from_lifecycle_record() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0)
    }

    pub fn from_diagnostics_bundle() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
    }

    pub fn from_replay_reconstruction() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0)
    }

    pub fn from_replay_mismatch() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0)
    }
}
