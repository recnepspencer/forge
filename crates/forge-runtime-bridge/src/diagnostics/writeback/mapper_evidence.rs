use crate::writeback::{
    BridgeMappedWritebackFamilyInput, BridgeWritebackMapperEnvelope, BridgeWritebackMapperRecord,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackMapperExplanation {
    record: BridgeWritebackMapperRecord,
}

impl BridgeWritebackMapperExplanation {
    pub fn from_record(record: &BridgeWritebackMapperRecord) -> Self {
        Self {
            record: record.clone(),
        }
    }

    pub fn record(&self) -> &BridgeWritebackMapperRecord {
        &self.record
    }

    pub fn record_identity(&self) -> &str {
        self.record.record_identity().as_str()
    }

    pub fn envelope_digest(&self) -> &str {
        self.record.mapper_envelope_digest()
    }

    pub fn mapped_input_digest(&self) -> &str {
        self.record.mapped_input_digest()
    }

    pub fn witness_digest(&self) -> &str {
        self.record.witness_digest()
    }

    pub fn candidate_digest(&self) -> &str {
        self.record.candidate_digest()
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.record.family_kind()
    }

    pub fn effect_class(&self) -> crate::writeback::BridgeWritebackEffectClass {
        self.record.effect_class()
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.record.strategy_class()
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.record.strategy_descriptor_digest()
    }

    pub fn causality_digest(&self) -> &str {
        self.record.causality_digest()
    }

    pub fn effect_intent_digest(&self) -> &str {
        self.record.effect_intent_digest()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackMapperEnvelopeExplanation {
    envelope: BridgeWritebackMapperEnvelope,
}

impl BridgeWritebackMapperEnvelopeExplanation {
    pub fn from_envelope(envelope: &BridgeWritebackMapperEnvelope) -> Self {
        Self {
            envelope: envelope.clone(),
        }
    }

    pub fn envelope(&self) -> &BridgeWritebackMapperEnvelope {
        &self.envelope
    }

    pub fn envelope_identity(&self) -> &str {
        self.envelope.envelope_identity().as_str()
    }

    pub fn contract_digest(&self) -> &str {
        self.envelope.contract_digest()
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.envelope.family_kind()
    }

    pub fn effect_class(&self) -> crate::writeback::BridgeWritebackEffectClass {
        self.envelope.effect_class()
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.envelope.strategy_class()
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.envelope.strategy_descriptor_digest()
    }

    pub fn causality_digest(&self) -> &str {
        self.envelope.causality_digest()
    }

    pub fn effect_intent_digest(&self) -> &str {
        self.envelope.effect_intent_digest()
    }

    pub fn envelope_digest(&self) -> &str {
        self.envelope.digest()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMappedWritebackFamilyInputExplanation {
    mapped_input: BridgeMappedWritebackFamilyInput,
}

impl BridgeMappedWritebackFamilyInputExplanation {
    pub fn from_mapped_input(mapped_input: &BridgeMappedWritebackFamilyInput) -> Self {
        Self {
            mapped_input: mapped_input.clone(),
        }
    }

    pub fn mapped_input(&self) -> &BridgeMappedWritebackFamilyInput {
        &self.mapped_input
    }

    pub fn mapped_input_identity(&self) -> &str {
        self.mapped_input.mapped_input_identity().as_str()
    }

    pub fn mapper_envelope_digest(&self) -> &str {
        self.mapped_input.mapper_envelope_digest()
    }

    pub fn contract_digest(&self) -> &str {
        self.mapped_input.contract_digest()
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.mapped_input.family_kind()
    }

    pub fn effect_class(&self) -> crate::writeback::BridgeWritebackEffectClass {
        self.mapped_input.effect_class()
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.mapped_input.strategy_class()
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.mapped_input.strategy_descriptor_digest()
    }

    pub fn causality_digest(&self) -> &str {
        self.mapped_input.causality_digest()
    }

    pub fn effect_intent_digest(&self) -> &str {
        self.mapped_input.effect_intent_digest()
    }

    pub fn mapped_input_digest(&self) -> &str {
        self.mapped_input.digest()
    }
}
