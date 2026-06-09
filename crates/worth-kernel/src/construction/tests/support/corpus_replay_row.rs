use crate::construction::request::PrimitiveConstructionFamily;
use crate::construction::tests::support::runtime_truth::PrimitiveConstructionCertificationRuntimeTruth;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PrimitiveConstructionCorpusParameterRole {
    MinimalAdmitted,
    GenericAdmitted,
    StressAdmitted,
    ThresholdAdmitted,
    ThresholdRejected,
    ExplicitExhaustion,
    ExplicitRejected,
}

impl PrimitiveConstructionCorpusParameterRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MinimalAdmitted => "minimal_admitted",
            Self::GenericAdmitted => "generic_admitted",
            Self::StressAdmitted => "stress_admitted",
            Self::ThresholdAdmitted => "threshold_admitted",
            Self::ThresholdRejected => "threshold_rejected",
            Self::ExplicitExhaustion => "explicit_exhaustion",
            Self::ExplicitRejected => "explicit_rejected",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PrimitiveConstructionCorpusReplaySiegeRow {
    scenario_id: String,
    family: PrimitiveConstructionFamily,
    parameter_role: PrimitiveConstructionCorpusParameterRole,
    runtime_truth: PrimitiveConstructionCertificationRuntimeTruth,
}

impl PrimitiveConstructionCorpusReplaySiegeRow {
    pub(crate) fn new(
        scenario_id: String,
        family: PrimitiveConstructionFamily,
        parameter_role: PrimitiveConstructionCorpusParameterRole,
        runtime_truth: PrimitiveConstructionCertificationRuntimeTruth,
    ) -> Self {
        Self {
            scenario_id,
            family,
            parameter_role,
            runtime_truth,
        }
    }

    pub fn scenario_id(&self) -> &str {
        &self.scenario_id
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn parameter_role(&self) -> PrimitiveConstructionCorpusParameterRole {
        self.parameter_role
    }

    pub fn runtime_truth(&self) -> &PrimitiveConstructionCertificationRuntimeTruth {
        &self.runtime_truth
    }

    pub fn outcome_digest(&self) -> &str {
        self.runtime_truth.outcome_digest()
    }
}
