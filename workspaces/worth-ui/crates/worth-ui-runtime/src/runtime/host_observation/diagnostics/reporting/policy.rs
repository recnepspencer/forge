use crate::runtime::host_observation::diagnostics::WorthUiDiagnosticRichnessTier;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiDiagnosticRichnessPolicy {
    tier: WorthUiDiagnosticRichnessTier,
}

/// Compatibility launch policy retained from the runtime authority boundary.
pub type WorthUiRuntimeDiagnosticPolicy = WorthUiDiagnosticRichnessPolicy;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiSupportReportPolicy {
    requested_tier: WorthUiDiagnosticRichnessTier,
}

impl WorthUiDiagnosticRichnessPolicy {
    pub fn off() -> Self {
        Self {
            tier: WorthUiDiagnosticRichnessTier::Off,
        }
    }

    pub fn minimal() -> Self {
        Self {
            tier: WorthUiDiagnosticRichnessTier::Minimal,
        }
    }

    pub fn standard() -> Self {
        Self {
            tier: WorthUiDiagnosticRichnessTier::Standard,
        }
    }

    pub fn full() -> Self {
        Self {
            tier: WorthUiDiagnosticRichnessTier::Full,
        }
    }

    pub fn support() -> Self {
        Self {
            tier: WorthUiDiagnosticRichnessTier::Support,
        }
    }

    pub fn rich() -> Self {
        Self::full()
    }

    pub fn tier(self) -> WorthUiDiagnosticRichnessTier {
        self.tier
    }
}

impl WorthUiSupportReportPolicy {
    pub fn from_diagnostic_policy(policy: WorthUiDiagnosticRichnessPolicy) -> Self {
        Self {
            requested_tier: policy.tier(),
        }
    }

    pub fn may_materialize_support_sections(self) -> bool {
        self.requested_tier.emits_support_sections()
    }
}

impl Default for WorthUiDiagnosticRichnessPolicy {
    fn default() -> Self {
        Self::minimal()
    }
}
