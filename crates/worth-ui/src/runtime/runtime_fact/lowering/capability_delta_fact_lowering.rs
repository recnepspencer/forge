use crate::runtime::{WorthUiCapabilityChangedFacts, WorthUiCapabilityReloadEvidence};

pub struct WorthUiCapabilityDeltaRuntimeFactLowering;

impl WorthUiCapabilityDeltaRuntimeFactLowering {
    pub fn from_reload_evidence(
        evidence: &WorthUiCapabilityReloadEvidence,
    ) -> WorthUiCapabilityChangedFacts {
        WorthUiCapabilityChangedFacts::from_reload_evidence(evidence)
    }
}
