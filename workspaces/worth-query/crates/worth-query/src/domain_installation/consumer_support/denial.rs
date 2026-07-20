use super::{
    WorthQueryConsumerSupportAdmissionCounters, WorthQueryConsumerSupportDimension,
    WorthQueryConsumerSupportPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerSupportCompatibilityDenial {
    dimension: WorthQueryConsumerSupportDimension,
    runtime_posture: WorthQueryConsumerSupportPosture,
    counters: WorthQueryConsumerSupportAdmissionCounters,
}

impl WorthQueryConsumerSupportCompatibilityDenial {
    pub(crate) fn new(
        dimension: WorthQueryConsumerSupportDimension,
        runtime_posture: WorthQueryConsumerSupportPosture,
        counters: WorthQueryConsumerSupportAdmissionCounters,
    ) -> Self {
        Self {
            dimension,
            runtime_posture,
            counters,
        }
    }

    pub fn dimension(&self) -> WorthQueryConsumerSupportDimension {
        self.dimension
    }

    pub fn runtime_posture(&self) -> WorthQueryConsumerSupportPosture {
        self.runtime_posture
    }

    pub fn counters(&self) -> WorthQueryConsumerSupportAdmissionCounters {
        self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryConsumerProjectionContractDenial {
    StaleInstallationGeneration,
    AlreadyMinted,
    Compatibility(WorthQueryConsumerSupportCompatibilityDenial),
}

impl From<WorthQueryConsumerSupportCompatibilityDenial>
    for WorthQueryConsumerProjectionContractDenial
{
    fn from(denial: WorthQueryConsumerSupportCompatibilityDenial) -> Self {
        Self::Compatibility(denial)
    }
}
