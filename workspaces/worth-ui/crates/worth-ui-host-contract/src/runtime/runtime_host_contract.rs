use super::{UiHostObservationValue, UiMeasurementRequest};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiHostKind {
    Headless,
    Egui,
    CapabilityProbeInconclusive,
    DiagnosticsOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiHostContract {
    kind: WorthUiHostKind,
}

pub trait WorthUiHostAdapter {
    fn host_contract(self) -> WorthUiHostContract;
}

pub trait WorthUiMeasurementHostAdapter {
    fn observe_measurement(&self, request: &UiMeasurementRequest) -> UiHostObservationValue;
}

impl WorthUiHostContract {
    pub fn headless() -> Self {
        Self {
            kind: WorthUiHostKind::Headless,
        }
    }

    pub fn egui() -> Self {
        Self {
            kind: WorthUiHostKind::Egui,
        }
    }

    pub fn capability_probe_inconclusive() -> Self {
        Self {
            kind: WorthUiHostKind::CapabilityProbeInconclusive,
        }
    }

    pub fn diagnostics_only() -> Self {
        Self {
            kind: WorthUiHostKind::DiagnosticsOnly,
        }
    }

    pub fn new(kind: WorthUiHostKind) -> Self {
        Self { kind }
    }

    pub fn kind(&self) -> WorthUiHostKind {
        self.kind
    }
}

impl WorthUiHostAdapter for WorthUiHostContract {
    fn host_contract(self) -> WorthUiHostContract {
        self
    }
}
