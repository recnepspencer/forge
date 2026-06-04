use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{BridgeMappedWritebackFamilyInputIdentity, BridgeWritebackMapperEnvelope};
use crate::writeback::{
    BridgeWritebackEffectClass, BridgeWritebackEffectIntent, BridgeWritebackFamilyKind,
    BridgeWritebackStrategyClass, BridgeWritebackStrategyDescriptorBasis,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMappedWritebackFamilyInput {
    mapped_input_identity: BridgeMappedWritebackFamilyInputIdentity,
    mapper_envelope_digest: Arc<str>,
    contract_digest: Arc<str>,
    family_kind: BridgeWritebackFamilyKind,
    effect_class: BridgeWritebackEffectClass,
    strategy_class: BridgeWritebackStrategyClass,
    strategy_descriptor_basis: BridgeWritebackStrategyDescriptorBasis,
    causality_digest: Arc<str>,
    effect_intent: BridgeWritebackEffectIntent,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeMappedWritebackFamilyInput {
    pub(crate) fn from_mapper_envelope(envelope: &BridgeWritebackMapperEnvelope) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-mapped-writeback-family-input|mapper-envelope={}|contract={}|family:{:?}|effect-class:{:?}|strategy-class:{:?}|strategy={}|causality={}|effect-intent={}|effect-intent-basis={}",
            envelope.digest(),
            envelope.contract_digest(),
            envelope.family_kind(),
            envelope.effect_class(),
            envelope.strategy_class(),
            envelope.strategy_descriptor_basis().digest(),
            envelope.causality_digest(),
            envelope.effect_intent_digest(),
            envelope.effect_intent().patch_canonical_basis(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            mapped_input_identity: BridgeMappedWritebackFamilyInputIdentity::new(format!(
                "bridge-mapped-writeback-family-input:sha256:{digest:x}"
            )),
            mapper_envelope_digest: Arc::from(envelope.digest().to_owned()),
            contract_digest: Arc::from(envelope.contract_digest().to_owned()),
            family_kind: envelope.family_kind(),
            effect_class: envelope.effect_class(),
            strategy_class: envelope.strategy_class(),
            strategy_descriptor_basis: envelope.strategy_descriptor_basis().clone(),
            causality_digest: Arc::from(envelope.causality_digest().to_owned()),
            effect_intent: envelope.effect_intent().clone(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-mapped-writeback-family-input:sha256:{digest:x}"
            )),
        }
    }

    pub fn mapped_input_identity(&self) -> &BridgeMappedWritebackFamilyInputIdentity {
        &self.mapped_input_identity
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_digest.as_ref()
    }

    pub fn mapper_envelope_digest(&self) -> &str {
        self.mapper_envelope_digest.as_ref()
    }

    pub fn family_kind(&self) -> BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn effect_class(&self) -> BridgeWritebackEffectClass {
        self.effect_class
    }

    pub fn strategy_class(&self) -> BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub fn strategy_descriptor_basis(&self) -> &BridgeWritebackStrategyDescriptorBasis {
        &self.strategy_descriptor_basis
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.strategy_descriptor_basis.digest()
    }

    pub fn causality_digest(&self) -> &str {
        self.causality_digest.as_ref()
    }

    pub fn effect_intent(&self) -> &BridgeWritebackEffectIntent {
        &self.effect_intent
    }

    pub fn effect_intent_digest(&self) -> &str {
        self.effect_intent.digest()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
