use super::contracts::HistoricalPathComplexityContract;
use super::cost::HistoricalPathCostPosture;
use super::counters::HistoricalCounterSnapshot;
#[cfg(test)]
use super::request::HistoricalEvaluationRequest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalPathVocabularyReport {
    requested_path_class_name: &'static str,
    cost_posture: HistoricalPathCostPosture,
    complexity_contract: HistoricalPathComplexityContract,
    counters: HistoricalCounterSnapshot,
}

impl HistoricalPathVocabularyReport {
    pub fn requested_path_class_name(&self) -> &'static str {
        self.requested_path_class_name
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

    #[cfg(test)]
    pub(crate) fn from_request(
        request: &HistoricalEvaluationRequest,
        complexity_contract: HistoricalPathComplexityContract,
        counters: HistoricalCounterSnapshot,
    ) -> Self {
        Self {
            requested_path_class_name: request.requested_path_class().as_str(),
            cost_posture: request.cost_posture().clone(),
            complexity_contract,
            counters,
        }
    }
}
