use super::contracts::HistoricalPathComplexityContract;
use super::cost::HistoricalPathCostPosture;
use super::counters::HistoricalCounterSnapshot;
use super::path_classes::{
    AdmittedHistoricalPathClass, RequestedHistoricalPathClass, ResolvedHistoricalPathClass,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalPathResolved {
    requested_path_class: RequestedHistoricalPathClass,
    admitted_path_class: AdmittedHistoricalPathClass,
    resolved_path_class: ResolvedHistoricalPathClass,
    cost_posture: HistoricalPathCostPosture,
    complexity_contract: HistoricalPathComplexityContract,
    counters: HistoricalCounterSnapshot,
}

impl HistoricalPathResolved {
    pub fn requested_path_class(&self) -> &RequestedHistoricalPathClass {
        &self.requested_path_class
    }

    pub fn admitted_path_class(&self) -> &AdmittedHistoricalPathClass {
        &self.admitted_path_class
    }

    pub fn resolved_path_class(&self) -> &ResolvedHistoricalPathClass {
        &self.resolved_path_class
    }

    pub fn cost_posture(&self) -> &HistoricalPathCostPosture {
        &self.cost_posture
    }

    pub fn complexity_contract(&self) -> &HistoricalPathComplexityContract {
        &self.complexity_contract
    }

    pub fn counters(&self) -> &HistoricalCounterSnapshot {
        &self.counters
    }

    pub(crate) fn new(
        requested_path_class: RequestedHistoricalPathClass,
        admitted_path_class: AdmittedHistoricalPathClass,
        resolved_path_class: ResolvedHistoricalPathClass,
        cost_posture: HistoricalPathCostPosture,
        complexity_contract: HistoricalPathComplexityContract,
        counters: HistoricalCounterSnapshot,
    ) -> Self {
        Self {
            requested_path_class,
            admitted_path_class,
            resolved_path_class,
            cost_posture,
            complexity_contract,
            counters,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalMaterializationPathMetadata {
    requested_path_class: RequestedHistoricalPathClass,
    admitted_path_class: AdmittedHistoricalPathClass,
    resolved_path_class: ResolvedHistoricalPathClass,
}

impl HistoricalMaterializationPathMetadata {
    pub fn requested_path_class(&self) -> &RequestedHistoricalPathClass {
        &self.requested_path_class
    }

    pub fn admitted_path_class(&self) -> &AdmittedHistoricalPathClass {
        &self.admitted_path_class
    }

    pub fn resolved_path_class(&self) -> &ResolvedHistoricalPathClass {
        &self.resolved_path_class
    }

    pub(crate) fn from_resolved(resolved: HistoricalPathResolved) -> Self {
        Self {
            requested_path_class: resolved.requested_path_class,
            admitted_path_class: resolved.admitted_path_class,
            resolved_path_class: resolved.resolved_path_class,
        }
    }
}
