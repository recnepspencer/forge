use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{BridgeIdentity, WritebackStrategyCoherenceIdentityTag};

use super::{
    AdmittedBridgeWritebackContract, BridgeDerivedWritebackEffect, BridgeWritebackIdempotenceBasis,
};

pub type BridgeWritebackStrategyCoherenceIdentity =
    BridgeIdentity<WritebackStrategyCoherenceIdentityTag>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeWritebackStrategyCoherenceDisposition {
    Coherent,
    FamilyKindMismatch,
    StrategyClassMismatch,
    StrategyDescriptorMismatch,
    EffectClassMismatch,
    IdempotenceClassMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackStrategyCoherenceReport {
    coherence_identity: BridgeWritebackStrategyCoherenceIdentity,
    contract_digest: Arc<str>,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    strategy_basis_digest: Arc<str>,
    writeback_effect_artifact_digest: Arc<str>,
    effect_intent_digest: Arc<str>,
    effect_intent_patch_canonical_basis: Arc<str>,
    idempotence_digest: Arc<str>,
    disposition: BridgeWritebackStrategyCoherenceDisposition,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackStrategyCoherenceReport {
    pub fn classify(
        contract: &AdmittedBridgeWritebackContract,
        effect: &BridgeDerivedWritebackEffect,
        idempotence: &BridgeWritebackIdempotenceBasis,
    ) -> Self {
        let declaration = contract.validated_declaration().declaration();
        let strategy_basis = contract
            .validated_declaration()
            .strategy_basis()
            .expect("admitted writeback contract must preserve strategy basis");
        let disposition = if strategy_basis.family_kind() != effect.family_kind()
            || strategy_basis.family_kind() != idempotence.family_kind()
        {
            BridgeWritebackStrategyCoherenceDisposition::FamilyKindMismatch
        } else if strategy_basis.strategy_class() != effect.strategy_class()
            || strategy_basis.strategy_class() != idempotence.strategy_class()
        {
            BridgeWritebackStrategyCoherenceDisposition::StrategyClassMismatch
        } else if strategy_basis.strategy_descriptor_basis() != effect.strategy_descriptor_basis()
            || strategy_basis.strategy_descriptor_basis() != idempotence.strategy_descriptor_basis()
        {
            BridgeWritebackStrategyCoherenceDisposition::StrategyDescriptorMismatch
        } else if declaration.effect_class() != effect.effect_class() {
            BridgeWritebackStrategyCoherenceDisposition::EffectClassMismatch
        } else if declaration.idempotence_class() != idempotence.idempotence_class() {
            BridgeWritebackStrategyCoherenceDisposition::IdempotenceClassMismatch
        } else {
            BridgeWritebackStrategyCoherenceDisposition::Coherent
        };
        let contract_digest = Arc::<str>::from(contract.digest().to_owned());
        let strategy_basis_digest = Arc::<str>::from(strategy_basis.digest().to_owned());
        let writeback_effect_artifact_digest = Arc::<str>::from(effect.digest().to_owned());
        let effect_intent_digest = Arc::<str>::from(effect.effect_intent_digest().to_owned());
        let effect_intent_patch_canonical_basis =
            Arc::<str>::from(effect.effect_intent().patch_canonical_basis().to_owned());
        let idempotence_digest = Arc::<str>::from(idempotence.digest().to_owned());
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-strategy-coherence|contract={}|family:{:?}|strategy-basis={}|writeback-effect-artifact={}|effect-intent={}|effect-intent-basis={}|idempotence={}|strategy-class:{:?}|effect-strategy-class:{:?}|idempotence-strategy-class:{:?}|contract-effect:{:?}|effect-effect:{:?}|contract-idempotence:{:?}|idempotence-class:{:?}|disposition:{disposition:?}",
            contract_digest.as_ref(),
            strategy_basis.family_kind(),
            strategy_basis_digest.as_ref(),
            writeback_effect_artifact_digest.as_ref(),
            effect_intent_digest.as_ref(),
            effect_intent_patch_canonical_basis.as_ref(),
            idempotence_digest.as_ref(),
            strategy_basis.strategy_class(),
            effect.strategy_class(),
            idempotence.strategy_class(),
            declaration.effect_class(),
            effect.effect_class(),
            declaration.idempotence_class(),
            idempotence.idempotence_class(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            coherence_identity: BridgeWritebackStrategyCoherenceIdentity::new(format!(
                "bridge-writeback-strategy-coherence:sha256:{digest:x}"
            )),
            contract_digest,
            family_kind: strategy_basis.family_kind(),
            strategy_basis_digest,
            writeback_effect_artifact_digest,
            effect_intent_digest,
            effect_intent_patch_canonical_basis,
            idempotence_digest,
            disposition,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-writeback-strategy-coherence:sha256:{digest:x}"
            )),
        }
    }

    pub fn disposition(&self) -> BridgeWritebackStrategyCoherenceDisposition {
        self.disposition
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_digest.as_ref()
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn strategy_basis_digest(&self) -> &str {
        self.strategy_basis_digest.as_ref()
    }

    pub fn writeback_effect_artifact_digest(&self) -> &str {
        self.writeback_effect_artifact_digest.as_ref()
    }

    pub fn effect_intent_digest(&self) -> &str {
        self.effect_intent_digest.as_ref()
    }

    pub fn effect_intent_patch_canonical_basis(&self) -> &str {
        self.effect_intent_patch_canonical_basis.as_ref()
    }

    pub fn idempotence_digest(&self) -> &str {
        self.idempotence_digest.as_ref()
    }
}
