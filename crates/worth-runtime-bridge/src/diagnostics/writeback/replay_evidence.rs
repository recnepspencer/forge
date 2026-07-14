use crate::writeback::{BridgeWritebackReplayBundle, BridgeWritebackReplayRecord};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackReplayExplanation {
    bundle: BridgeWritebackReplayBundle,
}

impl BridgeWritebackReplayExplanation {
    pub fn from_bundle(bundle: &BridgeWritebackReplayBundle) -> Self {
        Self {
            bundle: bundle.clone(),
        }
    }

    pub fn replay_bundle(&self) -> &BridgeWritebackReplayBundle {
        &self.bundle
    }

    pub fn replay_bundle_digest(&self) -> &str {
        self.bundle.digest()
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.bundle.strategy_class()
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.bundle.family_kind()
    }

    pub fn semantic_digest(&self) -> &str {
        self.bundle.semantic_digest()
    }

    pub fn causality_digest(&self) -> &str {
        self.bundle.causality_digest()
    }

    pub fn effect_intent_digest(&self) -> &str {
        self.bundle.effect_intent_digest()
    }

    pub fn effect_intent_patch_canonical_basis(&self) -> &str {
        self.bundle.effect_intent_patch_canonical_basis()
    }

    pub fn retry_disposition(&self) -> crate::writeback::BridgeWritebackRetryDisposition {
        self.bundle.retry_disposition()
    }

    pub fn outcome_class(&self) -> crate::writeback::BridgeWritebackOutcomeClass {
        self.bundle.outcome_class()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackReplayRecordExplanation {
    record: BridgeWritebackReplayRecord,
}

impl BridgeWritebackReplayRecordExplanation {
    pub fn from_record(record: &BridgeWritebackReplayRecord) -> Self {
        Self {
            record: record.clone(),
        }
    }

    pub fn replay_record(&self) -> &BridgeWritebackReplayRecord {
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

    pub fn expected_causality_digest(&self) -> &str {
        self.record.expected_causality_digest()
    }

    pub fn replayed_causality_digest(&self) -> &str {
        self.record.replayed_causality_digest()
    }

    pub fn expected_effect_intent_digest(&self) -> &str {
        self.record.expected_effect_intent_digest()
    }

    pub fn replayed_effect_intent_digest(&self) -> &str {
        self.record.replayed_effect_intent_digest()
    }

    pub fn expected_effect_intent_patch_canonical_basis(&self) -> &str {
        self.record.expected_effect_intent_patch_canonical_basis()
    }

    pub fn replayed_effect_intent_patch_canonical_basis(&self) -> &str {
        self.record.replayed_effect_intent_patch_canonical_basis()
    }
}
