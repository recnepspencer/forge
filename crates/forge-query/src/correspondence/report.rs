use super::contracts::CorrespondenceComplexityContract;
use super::cost::CorrespondenceCostPosture;
use super::counters::CorrespondenceCounterSnapshot;
#[cfg(test)]
use super::outcome::CorrespondenceOutcome;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrespondenceVocabularyReport {
    outcome_family_name: &'static str,
    cost_posture: CorrespondenceCostPosture,
    complexity_contract: CorrespondenceComplexityContract,
    counters: CorrespondenceCounterSnapshot,
}

impl CorrespondenceVocabularyReport {
    pub fn outcome_family_name(&self) -> &'static str {
        self.outcome_family_name
    }

    pub fn cost_posture(&self) -> &CorrespondenceCostPosture {
        &self.cost_posture
    }

    pub fn complexity_contract(&self) -> &CorrespondenceComplexityContract {
        &self.complexity_contract
    }

    pub fn counters(&self) -> &CorrespondenceCounterSnapshot {
        &self.counters
    }

    #[cfg(test)]
    pub(crate) fn from_outcome(
        outcome: &CorrespondenceOutcome,
        cost_posture: CorrespondenceCostPosture,
        complexity_contract: CorrespondenceComplexityContract,
        counters: CorrespondenceCounterSnapshot,
    ) -> Self {
        Self {
            outcome_family_name: outcome.family_name(),
            cost_posture,
            complexity_contract,
            counters,
        }
    }
}
