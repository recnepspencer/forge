use crate::runtime::{WorthUiValidationChangedFacts, WorthUiValidationReloadEvidence};

pub struct WorthUiValidationReloadRuntimeFactLowering;

impl WorthUiValidationReloadRuntimeFactLowering {
    pub fn from_reload_evidence(
        evidence: &WorthUiValidationReloadEvidence,
    ) -> WorthUiValidationChangedFacts {
        WorthUiValidationChangedFacts::from_reload_evidence(evidence)
    }
}
