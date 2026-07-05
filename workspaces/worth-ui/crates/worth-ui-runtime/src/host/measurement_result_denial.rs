use crate::evidence::UiMeasurementEvidenceCategory;
use worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration;
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use super::UiHostMeasurementExecutionDenial;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostMeasurementNormalizationDenial {
    CategoryMismatch {
        observed: UiMeasurementEvidenceCategory,
        normalized: UiMeasurementEvidenceCategory,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostMeasurementInvalidationReason {
    EvidenceGenerationDrift {
        recorded: UiEvidenceAuthorityGeneration,
        current: UiEvidenceAuthorityGeneration,
    },
    CapabilityObservationGenerationDrift {
        recorded: WorthUiHostCapabilityObservationGeneration,
        current: WorthUiHostCapabilityObservationGeneration,
    },
    CapabilityProfileDrift {
        recorded: u64,
        current: u64,
    },
    ViewportAssumptionDrift {
        recorded: u64,
        current: u64,
    },
    DpiAssumptionDrift {
        recorded: u64,
        current: u64,
    },
    FontAssumptionDrift {
        recorded: u64,
        current: u64,
    },
    AdapterProfileDrift {
        recorded: u64,
        current: u64,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiHostMeasurementEvidenceDenial {
    Execution(UiHostMeasurementExecutionDenial),
    Normalization(UiHostMeasurementNormalizationDenial),
    Stale(UiHostMeasurementInvalidationReason),
}
