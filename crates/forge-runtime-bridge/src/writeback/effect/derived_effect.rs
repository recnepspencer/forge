use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{BridgeIdentity, WritebackEffectIdentityTag};
use crate::writeback::{
    BridgeMappedWritebackFamilyInput, BridgeWritebackEffectClass, BridgeWritebackEffectIntent,
    BridgeWritebackStrategyDescriptorBasis,
};

pub type BridgeWritebackEffectIdentity = BridgeIdentity<WritebackEffectIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeDerivedWritebackEffect {
    effect_identity: BridgeWritebackEffectIdentity,
    mapper_envelope_digest: Arc<str>,
    mapped_input_digest: Arc<str>,
    contract_digest: Arc<str>,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    effect_class: BridgeWritebackEffectClass,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    strategy_descriptor_basis: BridgeWritebackStrategyDescriptorBasis,
    causality_digest: Arc<str>,
    effect_intent: BridgeWritebackEffectIntent,
    effect_intent_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeDerivedWritebackEffect {
    pub fn new(
        effect_identity: BridgeWritebackEffectIdentity,
        mapped_input: &BridgeMappedWritebackFamilyInput,
    ) -> Self {
        let contract_digest = Arc::<str>::from(mapped_input.contract_digest().to_owned());
        let strategy_descriptor_basis = mapped_input.strategy_descriptor_basis().clone();
        let causality_digest = Arc::<str>::from(mapped_input.causality_digest().to_owned());
        let effect_intent = mapped_input.effect_intent().clone();
        let effect_intent_digest = Arc::<str>::from(effect_intent.digest().to_owned());
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-derived-writeback-effect|id={}|mapped-input={}|contract={}|family:{:?}|effect:{:?}|strategy-class:{:?}|strategy={}|causality={}|effect-intent={}|effect-intent-basis={}",
            effect_identity.as_str(),
            mapped_input.digest(),
            contract_digest.as_ref(),
            mapped_input.family_kind(),
            mapped_input.effect_class(),
            mapped_input.strategy_class(),
            strategy_descriptor_basis.digest(),
            causality_digest.as_ref(),
            effect_intent_digest.as_ref(),
            effect_intent.patch_canonical_basis(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            effect_identity,
            mapper_envelope_digest: Arc::from(mapped_input.mapper_envelope_digest().to_owned()),
            mapped_input_digest: Arc::from(mapped_input.digest().to_owned()),
            contract_digest,
            family_kind: mapped_input.family_kind(),
            effect_class: mapped_input.effect_class(),
            strategy_class: mapped_input.strategy_class(),
            strategy_descriptor_basis,
            causality_digest,
            effect_intent,
            effect_intent_digest,
            canonical_basis,
            digest: Arc::from(format!("bridge-derived-writeback-effect:sha256:{digest:x}")),
        }
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.strategy_descriptor_basis.digest()
    }

    pub fn strategy_descriptor_basis(&self) -> &BridgeWritebackStrategyDescriptorBasis {
        &self.strategy_descriptor_basis
    }

    pub fn effect_identity(&self) -> &BridgeWritebackEffectIdentity {
        &self.effect_identity
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_digest.as_ref()
    }

    pub fn mapper_envelope_digest(&self) -> &str {
        self.mapper_envelope_digest.as_ref()
    }

    pub fn mapped_input_digest(&self) -> &str {
        self.mapped_input_digest.as_ref()
    }

    pub fn effect_class(&self) -> BridgeWritebackEffectClass {
        self.effect_class
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub fn effect_intent_digest(&self) -> &str {
        self.effect_intent_digest.as_ref()
    }

    pub fn effect_intent(&self) -> &BridgeWritebackEffectIntent {
        &self.effect_intent
    }

    pub fn causality_digest(&self) -> &str {
        self.causality_digest.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
