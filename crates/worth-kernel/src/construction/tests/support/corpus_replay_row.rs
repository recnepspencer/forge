use crate::construction::certification::corpus::lane_execution::{
    PrimitiveConstructionCorpusBranchLocalLane, PrimitiveConstructionCorpusCurrentHeadLane,
    PrimitiveConstructionCorpusReplayLane,
};
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
    current_head_lane: PrimitiveConstructionCorpusCurrentHeadLane,
    branch_local_lane: PrimitiveConstructionCorpusBranchLocalLane,
    replay_lane: PrimitiveConstructionCorpusReplayLane,
}

impl PrimitiveConstructionCorpusReplaySiegeRow {
    pub(crate) fn new(
        scenario_id: String,
        family: PrimitiveConstructionFamily,
        parameter_role: PrimitiveConstructionCorpusParameterRole,
        current_head_lane: PrimitiveConstructionCorpusCurrentHeadLane,
        branch_local_lane: PrimitiveConstructionCorpusBranchLocalLane,
        replay_lane: PrimitiveConstructionCorpusReplayLane,
    ) -> Self {
        Self {
            scenario_id,
            family,
            parameter_role,
            current_head_lane,
            branch_local_lane,
            replay_lane,
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
        self.current_head_lane.runtime_truth()
    }

    pub(crate) fn current_head_lane(&self) -> &PrimitiveConstructionCorpusCurrentHeadLane {
        &self.current_head_lane
    }

    pub(crate) fn branch_local_lane(&self) -> &PrimitiveConstructionCorpusBranchLocalLane {
        &self.branch_local_lane
    }

    pub(crate) fn replay_lane(&self) -> &PrimitiveConstructionCorpusReplayLane {
        &self.replay_lane
    }

    pub fn outcome_digest(&self) -> &str {
        self.current_head_lane.runtime_truth().outcome_digest()
    }
}
