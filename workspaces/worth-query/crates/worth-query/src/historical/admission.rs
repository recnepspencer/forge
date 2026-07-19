use super::contracts::{
    HistoricalPathComplexityContract, HistoricalPathReuseDescriptor,
    HistoricalReconstructionBudget, HistoricalReplaySpanBudget,
};
use super::cost::HistoricalPathCostPosture;
use super::counters::HistoricalCounterSnapshot;
use super::path_classes::{
    AdmittedHistoricalPathClass, HistoricalPathCompatibilityOutcome, RequestedHistoricalPathClass,
    ResolvedHistoricalPathClass,
};
use super::request::HistoricalPathRequested;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalEvaluationAdmission {
    requested_path: HistoricalPathRequested,
    admitted_path: HistoricalPathAdmitted,
    compatibility_outcome: HistoricalPathCompatibilityOutcome,
    cost_posture: HistoricalPathCostPosture,
    replay_budget: HistoricalReplaySpanBudget,
    reconstruction_budget: HistoricalReconstructionBudget,
    reuse_descriptor: HistoricalPathReuseDescriptor,
    complexity_contract: HistoricalPathComplexityContract,
    counters: HistoricalCounterSnapshot,
}

impl HistoricalEvaluationAdmission {
    pub fn requested_path(&self) -> &HistoricalPathRequested {
        &self.requested_path
    }

    pub fn admitted_path(&self) -> &HistoricalPathAdmitted {
        &self.admitted_path
    }

    pub fn compatibility_outcome(&self) -> &HistoricalPathCompatibilityOutcome {
        &self.compatibility_outcome
    }

    pub fn cost_posture(&self) -> &HistoricalPathCostPosture {
        &self.cost_posture
    }

    pub fn replay_budget(&self) -> &HistoricalReplaySpanBudget {
        &self.replay_budget
    }

    pub fn reconstruction_budget(&self) -> &HistoricalReconstructionBudget {
        &self.reconstruction_budget
    }

    pub fn reuse_descriptor(&self) -> &HistoricalPathReuseDescriptor {
        &self.reuse_descriptor
    }

    pub fn complexity_contract(&self) -> &HistoricalPathComplexityContract {
        &self.complexity_contract
    }

    pub fn counters(&self) -> &HistoricalCounterSnapshot {
        &self.counters
    }

    pub(crate) fn admitted(
        requested_path: HistoricalPathRequested,
        admitted_path: HistoricalPathAdmitted,
        cost_posture: HistoricalPathCostPosture,
        replay_budget: HistoricalReplaySpanBudget,
        reconstruction_budget: HistoricalReconstructionBudget,
        reuse_descriptor: HistoricalPathReuseDescriptor,
        complexity_contract: HistoricalPathComplexityContract,
        counters: HistoricalCounterSnapshot,
    ) -> Self {
        Self {
            requested_path,
            admitted_path,
            compatibility_outcome: HistoricalPathCompatibilityOutcome::Admitted,
            cost_posture,
            replay_budget,
            reconstruction_budget,
            reuse_descriptor,
            complexity_contract,
            counters,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalPathAdmitted {
    requested_path_class: RequestedHistoricalPathClass,
    admitted_path_class: AdmittedHistoricalPathClass,
}

impl HistoricalPathAdmitted {
    pub fn requested_path_class(&self) -> &RequestedHistoricalPathClass {
        &self.requested_path_class
    }

    pub fn admitted_path_class(&self) -> &AdmittedHistoricalPathClass {
        &self.admitted_path_class
    }

    pub(crate) fn new(
        requested_path_class: RequestedHistoricalPathClass,
        admitted_path_class: AdmittedHistoricalPathClass,
    ) -> Self {
        Self {
            requested_path_class,
            admitted_path_class,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalPathSubstitutionDenied {
    requested_path_class: RequestedHistoricalPathClass,
    attempted_resolved_path_class: ResolvedHistoricalPathClass,
    reason: &'static str,
}

impl HistoricalPathSubstitutionDenied {
    pub fn requested_path_class(&self) -> &RequestedHistoricalPathClass {
        &self.requested_path_class
    }

    pub fn attempted_resolved_path_class(&self) -> &ResolvedHistoricalPathClass {
        &self.attempted_resolved_path_class
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}
