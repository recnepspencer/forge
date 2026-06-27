#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicStateTruthPosture {
    UiRuntimeState,
    DerivedFromAuthoritativeRuntimeTruth,
    AuthoritativeQueryTruthForDiagnostics,
    AuthoritativeRelationalTruthForDiagnostics,
    MissingForDiagnostics,
}

impl MosaicStateTruthPosture {
    pub fn ui_runtime_state() -> Self {
        Self::UiRuntimeState
    }

    pub fn derived_from_authoritative_runtime_truth() -> Self {
        Self::DerivedFromAuthoritativeRuntimeTruth
    }

    pub fn authoritative_query_truth_for_diagnostics() -> Self {
        Self::AuthoritativeQueryTruthForDiagnostics
    }

    pub fn authoritative_relational_truth_for_diagnostics() -> Self {
        Self::AuthoritativeRelationalTruthForDiagnostics
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::MissingForDiagnostics
    }

    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::MissingForDiagnostics)
    }

    pub(crate) fn is_authoritative_truth_claim(&self) -> bool {
        matches!(
            self,
            Self::AuthoritativeQueryTruthForDiagnostics
                | Self::AuthoritativeRelationalTruthForDiagnostics
        )
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::UiRuntimeState => "ui_runtime_state",
            Self::DerivedFromAuthoritativeRuntimeTruth => {
                "derived_from_authoritative_runtime_truth"
            }
            Self::AuthoritativeQueryTruthForDiagnostics => "authoritative_query_truth",
            Self::AuthoritativeRelationalTruthForDiagnostics => "authoritative_relational_truth",
            Self::MissingForDiagnostics => "missing",
        }
    }
}
