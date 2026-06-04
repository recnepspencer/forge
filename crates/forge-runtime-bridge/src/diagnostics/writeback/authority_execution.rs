use crate::writeback::{BridgeWritebackAuthorityOutcome, BridgeWritebackExecutionRecord};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackOutcomeExplanation {
    outcome: BridgeWritebackAuthorityOutcome,
}

impl BridgeWritebackOutcomeExplanation {
    pub fn from_outcome(outcome: &BridgeWritebackAuthorityOutcome) -> Self {
        Self {
            outcome: outcome.clone(),
        }
    }

    pub fn outcome(&self) -> &BridgeWritebackAuthorityOutcome {
        &self.outcome
    }

    pub fn outcome_digest(&self) -> &str {
        self.outcome.digest()
    }

    pub fn outcome_class(&self) -> crate::writeback::BridgeWritebackOutcomeClass {
        self.outcome.outcome_class()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackExecutionExplanation {
    record: BridgeWritebackExecutionRecord,
}

impl BridgeWritebackExecutionExplanation {
    pub fn from_record(record: &BridgeWritebackExecutionRecord) -> Self {
        Self {
            record: record.clone(),
        }
    }

    pub fn record(&self) -> &BridgeWritebackExecutionRecord {
        &self.record
    }

    pub fn record_identity(&self) -> &str {
        self.record.record_identity().as_str()
    }

    pub fn failure_class(&self) -> Option<crate::writeback::BridgeWritebackFailureClass> {
        self.record.failure_class()
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.record.family_kind()
    }

    pub fn idempotence_digest(&self) -> &str {
        self.record.idempotence_digest()
    }

    pub fn effect_intent_patch_canonical_basis(&self) -> &str {
        self.record.effect_intent_patch_canonical_basis()
    }

    pub fn loop_prevention_digest(&self) -> &str {
        self.record.loop_prevention_digest()
    }

    pub fn strategy_coherence_digest(&self) -> &str {
        self.record.strategy_coherence_digest()
    }

    pub fn mapper_record_digest(&self) -> Option<&str> {
        self.record.mapper_record_digest()
    }

    pub fn counter_digest(&self) -> &str {
        self.record.counters().digest()
    }
}
