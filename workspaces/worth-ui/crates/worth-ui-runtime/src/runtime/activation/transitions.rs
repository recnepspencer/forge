use crate::runtime::replacement::WorthUiReplacementLoweringReady;

/// Activation staging entry proof minted only by the replacement lowering orchestrator.
#[derive(Debug)]
pub struct WorthUiActivationLaneInput(pub(crate) WorthUiReplacementLoweringReady);

impl WorthUiActivationLaneInput {
    pub(crate) fn from_lowering(lowering: WorthUiReplacementLoweringReady) -> Self {
        Self(lowering)
    }

    pub fn lowering(&self) -> &WorthUiReplacementLoweringReady {
        &self.0
    }
}