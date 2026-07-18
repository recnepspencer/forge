use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum ControlledDefectKind {
    LostUiDenial,
    InvertedScenarioAssertion,
    BroadSupportDependency,
    HiddenNestedCargo,
    OmittedCiPartition,
    SameProcessCrashSubstitute,
    StalePreflightEvidence,
    FeatureLeakage,
}

impl ControlledDefectKind {
    pub const ALL: [Self; 8] = [
        Self::LostUiDenial,
        Self::InvertedScenarioAssertion,
        Self::BroadSupportDependency,
        Self::HiddenNestedCargo,
        Self::OmittedCiPartition,
        Self::SameProcessCrashSubstitute,
        Self::StalePreflightEvidence,
        Self::FeatureLeakage,
    ];

    pub const fn expected_product(self) -> &'static str {
        match self {
            Self::LostUiDenial => "store-ui",
            Self::InvertedScenarioAssertion => "store-ci:recovery",
            Self::BroadSupportDependency => "store-owner",
            Self::HiddenNestedCargo => "store-ci:test-control",
            Self::OmittedCiPartition => "store-ci",
            Self::SameProcessCrashSubstitute => "store-ci:physical-isolation",
            Self::StalePreflightEvidence => "structural-preflight",
            Self::FeatureLeakage => "store-ci:feature-compatibility",
        }
    }

    pub const fn expected_predicate(self) -> &'static str {
        match self {
            Self::LostUiDenial => "checked-semantic-denial",
            Self::InvertedScenarioAssertion => "scenario-assertion-verdict",
            Self::BroadSupportDependency => "owner-build-closure",
            Self::HiddenNestedCargo => "declared-process-topology",
            Self::OmittedCiPartition => "required-ci-lane-completeness",
            Self::SameProcessCrashSubstitute => "fresh-process-probe-identity",
            Self::StalePreflightEvidence => "preflight-input-freshness",
            Self::FeatureLeakage => "production-feature-authority",
        }
    }

    pub const fn expected_failure_code(self) -> &'static str {
        match self {
            Self::LostUiDenial => "semantic-denial-missing",
            Self::InvertedScenarioAssertion => "named-assertion-failed",
            Self::BroadSupportDependency => "high-radius-owner-dependency",
            Self::HiddenNestedCargo => "nested-cargo-authority-missing",
            Self::OmittedCiPartition => "required-lane-missing",
            Self::SameProcessCrashSubstitute => "fresh-process-evidence-missing",
            Self::StalePreflightEvidence => "preflight-input-changed",
            Self::FeatureLeakage => "test-authority-feature-on-production-edge",
        }
    }
}
