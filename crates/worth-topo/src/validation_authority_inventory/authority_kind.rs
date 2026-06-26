#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorthValidationAuthorityKind {
    WholeViewValidatorEntry,
    DerivedRuleRegistryEntry,
    RuntimeInvariantRegistrationPack,
    CertificationExpectationArray,
    OperatorCloseoutValidationProof,
    CertificationComparisonReport,
}

impl WorthValidationAuthorityKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WholeViewValidatorEntry => "whole-view-validator-entry",
            Self::DerivedRuleRegistryEntry => "derived-rule-registry-entry",
            Self::RuntimeInvariantRegistrationPack => "runtime-invariant-registration-pack",
            Self::CertificationExpectationArray => "certification-expectation-array",
            Self::OperatorCloseoutValidationProof => "operator-closeout-validation-proof",
            Self::CertificationComparisonReport => "certification-comparison-report",
        }
    }
}
