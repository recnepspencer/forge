use crate::writeback::{BridgeValidatedWritebackCandidate, BridgeWritebackFamilyAdmissionRecord};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackAdmissionExplanation {
    record: BridgeWritebackFamilyAdmissionRecord,
}

impl BridgeWritebackAdmissionExplanation {
    pub fn from_record(record: &BridgeWritebackFamilyAdmissionRecord) -> Self {
        Self {
            record: record.clone(),
        }
    }

    pub fn record(&self) -> &BridgeWritebackFamilyAdmissionRecord {
        &self.record
    }

    pub fn record_identity(&self) -> &str {
        self.record.record_identity().as_str()
    }

    pub fn contract_digest(&self) -> &str {
        self.record.contract_digest()
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.record.family_kind()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackCandidateExplanation {
    candidate: BridgeValidatedWritebackCandidate,
}

impl BridgeWritebackCandidateExplanation {
    pub fn from_candidate(candidate: &BridgeValidatedWritebackCandidate) -> Self {
        Self {
            candidate: candidate.clone(),
        }
    }

    pub fn candidate(&self) -> &BridgeValidatedWritebackCandidate {
        &self.candidate
    }

    pub fn candidate_digest(&self) -> &str {
        self.candidate.digest()
    }

    pub fn writeback_effect_artifact_digest(&self) -> &str {
        self.candidate.writeback_effect_artifact_digest()
    }

    pub fn effect_intent_digest(&self) -> &str {
        self.candidate.effect_intent_digest()
    }

    pub fn effect_intent_patch_canonical_basis(&self) -> &str {
        self.candidate.effect_intent_patch_canonical_basis()
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.candidate.strategy_class()
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.candidate.family_kind()
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.candidate.strategy_descriptor_digest()
    }

    pub fn retry_disposition(&self) -> crate::writeback::BridgeWritebackRetryDisposition {
        self.candidate.retry_disposition()
    }

    pub fn loop_prevention_digest(&self) -> &str {
        self.candidate.loop_prevention_digest()
    }

    pub fn strategy_coherence_digest(&self) -> &str {
        self.candidate.strategy_coherence_digest()
    }
}
