use worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration;
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiQueryWorldCompatibilityFailure {
    InstalledAuthorityMismatch,
    SnapshotBasisMismatch,
    QueryAuthorityUnavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiMeasurementGenerationCompatibility {
    Compatible,
    StaleQueryFactReceipt {
        expected: UiEvidenceAuthorityGeneration,
        observed: UiEvidenceAuthorityGeneration,
    },
    StaleHostEvidence {
        expected: UiEvidenceAuthorityGeneration,
        observed: UiEvidenceAuthorityGeneration,
    },
    StaleHostCapability {
        expected: WorthUiHostCapabilityObservationGeneration,
        observed: WorthUiHostCapabilityObservationGeneration,
    },
    IncompatibleWorld {
        reason: UiQueryWorldCompatibilityFailure,
    },
    IncompatibleHostProfile {
        expected_profile_digest: u64,
        observed_profile_digest: u64,
    },
}

impl UiMeasurementGenerationCompatibility {
    pub const fn is_compatible(&self) -> bool {
        matches!(self, Self::Compatible)
    }
}
