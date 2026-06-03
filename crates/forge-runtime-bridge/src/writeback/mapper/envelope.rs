use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::BridgeWritebackMapperEnvelopeIdentity;
use crate::writeback::{
    AdmittedBridgeWritebackContract, BridgeWritebackEffectClass, BridgeWritebackEffectIntent,
    BridgeWritebackFamilyKind, BridgeWritebackNativeCausalityInputs, BridgeWritebackStrategyClass,
    BridgeWritebackStrategyDescriptorBasis,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackMapperEnvelope {
    envelope_identity: BridgeWritebackMapperEnvelopeIdentity,
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

impl BridgeWritebackMapperEnvelope {
    pub(crate) fn new(
        contract: &AdmittedBridgeWritebackContract,
        causality: &BridgeWritebackNativeCausalityInputs,
        effect_intent: BridgeWritebackEffectIntent,
    ) -> Self {
        let family_basis = contract
            .validated_declaration()
            .family_basis()
            .expect("admitted writeback contract must preserve family basis");
        let strategy_basis = contract
            .validated_declaration()
            .strategy_basis()
            .expect("admitted writeback contract must preserve strategy basis");
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-mapper-envelope|contract={}|family:{:?}|family-basis={}|effect-class:{:?}|strategy-class:{:?}|strategy-basis={}|strategy={}|causality={}|effect-intent={}|effect-intent-basis={}",
            contract.digest(),
            family_basis.family_kind(),
            family_basis.digest(),
            effect_intent.effect_class(),
            strategy_basis.strategy_class(),
            strategy_basis.digest(),
            strategy_basis.strategy_descriptor_basis().digest(),
            causality.digest(),
            effect_intent.digest(),
            effect_intent.patch_canonical_basis(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            envelope_identity: BridgeWritebackMapperEnvelopeIdentity::new(format!(
                "bridge-writeback-mapper-envelope:sha256:{digest:x}"
            )),
            contract_digest: Arc::from(contract.digest().to_owned()),
            family_kind: family_basis.family_kind(),
            effect_class: effect_intent.effect_class(),
            strategy_class: strategy_basis.strategy_class(),
            strategy_descriptor_basis: strategy_basis.strategy_descriptor_basis().clone(),
            causality_digest: Arc::from(causality.digest().to_owned()),
            effect_intent,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-writeback-mapper-envelope:sha256:{digest:x}"
            )),
        }
    }

    pub fn envelope_identity(&self) -> &BridgeWritebackMapperEnvelopeIdentity {
        &self.envelope_identity
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_digest.as_ref()
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
