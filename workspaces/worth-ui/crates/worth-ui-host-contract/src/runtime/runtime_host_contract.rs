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

pub trait WorthUiMeasurementHostAdapter {
    fn observe_measurement(&self, request: &UiMeasurementRequest) -> UiHostObservationValue;
}

/// Native adapter that can be explicitly admitted into one active host
/// session. The report describes mechanics the adapter can actually observe;
/// the session assigns operational identity and freshness generation.
pub trait WorthUiOperationalHostAdapter: WorthUiMeasurementHostAdapter {
    fn operational_host_contract(&self) -> WorthUiHostContract;

    fn operational_capability_report(&self) -> super::WorthUiHostCapabilityReport;
}

/// A configured host is operational by construction. Contract-only markers
/// are not accepted by application preparation.
pub trait WorthUiHostAdapter: WorthUiOperationalHostAdapter {}

impl<Adapter> WorthUiHostAdapter for Adapter where Adapter: WorthUiOperationalHostAdapter + ?Sized {}

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
