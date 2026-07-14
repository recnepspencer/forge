use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{BridgeMappedWritebackFamilyInput, BridgeWritebackMapperWitnessIdentity};
use crate::writeback::{
    BridgeDerivedWritebackEffect, BridgeWritebackEffectClass, BridgeWritebackFamilyKind,
    BridgeWritebackStrategyClass, BridgeWritebackStrategyDescriptorBasis,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackMapperWitness {
    witness_identity: BridgeWritebackMapperWitnessIdentity,
    mapper_envelope_digest: Arc<str>,
    mapped_input_digest: Arc<str>,
    family_kind: BridgeWritebackFamilyKind,
    effect_class: BridgeWritebackEffectClass,
    strategy_class: BridgeWritebackStrategyClass,
    strategy_descriptor_basis: BridgeWritebackStrategyDescriptorBasis,
    causality_digest: Arc<str>,
    effect_intent_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackMapperWitness {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn issue(mapped_input: &BridgeMappedWritebackFamilyInput) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-mapper-witness|mapper-envelope={}|mapped-input={}|family:{:?}|effect-class:{:?}|strategy-class:{:?}|strategy={}|causality={}|effect-intent={}",
            mapped_input.mapper_envelope_digest(),
            mapped_input.digest(),
            mapped_input.family_kind(),
            mapped_input.effect_class(),
            mapped_input.strategy_class(),
            mapped_input.strategy_descriptor_basis().digest(),
            mapped_input.causality_digest(),
            mapped_input.effect_intent_digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            witness_identity: BridgeWritebackMapperWitnessIdentity::admit_bridge_owned(format!(
                "bridge-writeback-mapper-witness:sha256:{digest:x}"
            )),
            mapper_envelope_digest: Arc::from(mapped_input.mapper_envelope_digest().to_owned()),
            mapped_input_digest: Arc::from(mapped_input.digest().to_owned()),
            family_kind: mapped_input.family_kind(),
            effect_class: mapped_input.effect_class(),
            strategy_class: mapped_input.strategy_class(),
            strategy_descriptor_basis: mapped_input.strategy_descriptor_basis().clone(),
            causality_digest: Arc::from(mapped_input.causality_digest().to_owned()),
            effect_intent_digest: Arc::from(mapped_input.effect_intent_digest().to_owned()),
            canonical_basis,
            digest: Arc::from(format!("bridge-writeback-mapper-witness:sha256:{digest:x}")),
        }
    }

    pub(crate) fn issue_from_effect(effect: &BridgeDerivedWritebackEffect) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-mapper-witness|mapper-envelope={}|mapped-input={}|family:{:?}|effect-class:{:?}|strategy-class:{:?}|strategy={}|causality={}|effect-intent={}",
            effect.mapper_envelope_digest(),
            effect.mapped_input_digest(),
            effect.family_kind(),
            effect.effect_class(),
            effect.strategy_class(),
            effect.strategy_descriptor_basis().digest(),
            effect.causality_digest(),
            effect.effect_intent_digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            witness_identity: BridgeWritebackMapperWitnessIdentity::admit_bridge_owned(format!(
                "bridge-writeback-mapper-witness:sha256:{digest:x}"
            )),
            mapper_envelope_digest: Arc::from(effect.mapper_envelope_digest().to_owned()),
            mapped_input_digest: Arc::from(effect.mapped_input_digest().to_owned()),
            family_kind: effect.family_kind(),
            effect_class: effect.effect_class(),
            strategy_class: effect.strategy_class(),
            strategy_descriptor_basis: effect.strategy_descriptor_basis().clone(),
            causality_digest: Arc::from(effect.causality_digest().to_owned()),
            effect_intent_digest: Arc::from(effect.effect_intent_digest().to_owned()),
            canonical_basis,
            digest: Arc::from(format!("bridge-writeback-mapper-witness:sha256:{digest:x}")),
        }
    }

    pub fn witness_identity(&self) -> &BridgeWritebackMapperWitnessIdentity {
        &self.witness_identity
    }

    pub fn family_kind(&self) -> BridgeWritebackFamilyKind {
        self.family_kind
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

    pub fn effect_intent_digest(&self) -> &str {
        self.effect_intent_digest.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
