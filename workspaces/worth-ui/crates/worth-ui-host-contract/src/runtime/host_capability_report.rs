use super::{
    WorthUiHostCapability, WorthUiHostCapabilityPosture, WorthUiHostContract, WorthUiHostKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHostCapabilityReport {
    posture: WorthUiHostCapabilityPosture,
    observed_capabilities: Box<[WorthUiHostCapability]>,
}

impl WorthUiHostCapabilityReport {
    pub fn from_contract(contract: WorthUiHostContract) -> Self {
        match contract.kind() {
            WorthUiHostKind::Headless => Self::missing(Vec::new()),
            WorthUiHostKind::Egui => Self::available(vec![
                WorthUiHostCapability::Accessibility,
                WorthUiHostCapability::FontMetrics,
                WorthUiHostCapability::Ime,
                WorthUiHostCapability::TextInput,
                WorthUiHostCapability::VisualCapture,
            ]),
            WorthUiHostKind::CapabilityProbeInconclusive => {
                Self::ambiguous(vec![WorthUiHostCapability::TextInput])
            }
            WorthUiHostKind::DiagnosticsOnly => {
                Self::diagnostic_only(vec![WorthUiHostCapability::TextInput])
            }
        }
    }

    pub fn available(capabilities: Vec<WorthUiHostCapability>) -> Self {
        Self::new(WorthUiHostCapabilityPosture::Available, capabilities)
    }

    pub fn missing(capabilities: Vec<WorthUiHostCapability>) -> Self {
        Self::new(WorthUiHostCapabilityPosture::Missing, capabilities)
    }

    pub fn ambiguous(capabilities: Vec<WorthUiHostCapability>) -> Self {
        Self::new(WorthUiHostCapabilityPosture::Ambiguous, capabilities)
    }

    pub fn diagnostic_only(capabilities: Vec<WorthUiHostCapability>) -> Self {
        Self::new(WorthUiHostCapabilityPosture::DiagnosticOnly, capabilities)
    }

    fn new(
        posture: WorthUiHostCapabilityPosture,
        mut observed_capabilities: Vec<WorthUiHostCapability>,
    ) -> Self {
        observed_capabilities.sort_by_key(|capability| capability.as_str());
        observed_capabilities.dedup();

        Self {
            posture,
            observed_capabilities: observed_capabilities.into_boxed_slice(),
        }
    }

    pub fn posture(&self) -> WorthUiHostCapabilityPosture {
        self.posture
    }

    pub fn observed_capabilities(&self) -> &[WorthUiHostCapability] {
        &self.observed_capabilities
    }

    pub fn supports(&self, capability: WorthUiHostCapability) -> bool {
        self.observed_capabilities.contains(&capability)
    }
}
