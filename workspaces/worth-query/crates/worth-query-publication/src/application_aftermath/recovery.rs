use super::WorthQueryPublishedAftermathPosture;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPublishedRecoverySupportTruth {
    DegradedRecoveryReport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPublishedRecoveryDurability {
    StoreCapabilityRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPublishedRecoverySupport {
    support_truth: WorthQueryPublishedRecoverySupportTruth,
    posture: WorthQueryPublishedAftermathPosture,
    durability: WorthQueryPublishedRecoveryDurability,
}

impl WorthQueryPublishedRecoverySupport {
    pub(super) const fn new(posture: WorthQueryPublishedAftermathPosture) -> Self {
        Self {
            support_truth: WorthQueryPublishedRecoverySupportTruth::DegradedRecoveryReport,
            posture,
            durability: WorthQueryPublishedRecoveryDurability::StoreCapabilityRequired,
        }
    }

    pub const fn support_truth(&self) -> WorthQueryPublishedRecoverySupportTruth {
        self.support_truth
    }

    pub const fn posture(&self) -> WorthQueryPublishedAftermathPosture {
        self.posture
    }

    pub const fn durability(&self) -> WorthQueryPublishedRecoveryDurability {
        self.durability
    }
}
