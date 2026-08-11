//! Process-local durability postures carried by execution outcomes.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryRecoveryDurabilityPosture {
    StoreCapabilityRequired,
}

impl WorthQueryRecoveryDurabilityPosture {
    pub const fn as_decision58_label(self) -> &'static str {
        match self {
            Self::StoreCapabilityRequired => "store-capability-required",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDispatchOutboxDurabilityPosture {
    StoreCapabilityRequired,
}

impl WorthQueryDispatchOutboxDurabilityPosture {
    pub const fn as_decision58_label(self) -> &'static str {
        match self {
            Self::StoreCapabilityRequired => "store-capability-required",
        }
    }
}
