use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::BridgeDerivedWritebackEffect;
use crate::writeback::{BridgeWritebackEffectClass, BridgeWritebackStrategyDescriptorBasis};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackFeedbackProvenance {
    contract_digest: Arc<str>,
    writeback_effect_artifact_digest: Arc<str>,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    effect_class: BridgeWritebackEffectClass,
    effect_intent_digest: Arc<str>,
    effect_intent_patch_canonical_basis: Arc<str>,
    causality_digest: Arc<str>,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    strategy_descriptor_basis: BridgeWritebackStrategyDescriptorBasis,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackFeedbackProvenance {
    pub fn new(effect: &BridgeDerivedWritebackEffect) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-feedback-provenance|family:{:?}|effect:{:?}|effect-intent={}|effect-intent-basis={}|causality={}|strategy-class:{:?}|strategy={}",
            effect.family_kind(),
            effect.effect_class(),
            effect.effect_intent_digest(),
            effect.effect_intent().patch_canonical_basis(),
            effect.causality_digest(),
            effect.strategy_class(),
            effect.strategy_descriptor_basis().digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            contract_digest: Arc::from(effect.contract_digest().to_owned()),
            writeback_effect_artifact_digest: Arc::from(effect.digest().to_owned()),
            family_kind: effect.family_kind(),
            effect_class: effect.effect_class(),
            effect_intent_digest: Arc::from(effect.effect_intent_digest().to_owned()),
            effect_intent_patch_canonical_basis: Arc::from(
                effect.effect_intent().patch_canonical_basis().to_owned(),
            ),
            causality_digest: Arc::from(effect.causality_digest().to_owned()),
            strategy_class: effect.strategy_class(),
            strategy_descriptor_basis: effect.strategy_descriptor_basis().clone(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-writeback-feedback-provenance:sha256:{digest:x}"
            )),
        }
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_digest.as_ref()
    }

    pub fn writeback_effect_artifact_digest(&self) -> &str {
        self.writeback_effect_artifact_digest.as_ref()
    }

    pub fn effect_class(&self) -> BridgeWritebackEffectClass {
        self.effect_class
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn effect_intent_digest(&self) -> &str {
        self.effect_intent_digest.as_ref()
    }

    pub fn effect_intent_patch_canonical_basis(&self) -> &str {
        self.effect_intent_patch_canonical_basis.as_ref()
    }

    pub fn causality_digest(&self) -> &str {
        self.causality_digest.as_ref()
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub fn strategy_descriptor_basis(&self) -> &BridgeWritebackStrategyDescriptorBasis {
        &self.strategy_descriptor_basis
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.strategy_descriptor_basis.digest()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackFeedbackContext {
    provenance_digest: Arc<str>,
    causality_digest: Arc<str>,
    effect_intent_digest: Arc<str>,
    effect_intent_patch_canonical_basis: Arc<str>,
    strategy_descriptor_basis: BridgeWritebackStrategyDescriptorBasis,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackFeedbackContext {
    pub fn from_provenance(provenance: &BridgeWritebackFeedbackProvenance) -> Self {
        let provenance_digest = Arc::<str>::from(provenance.digest().to_owned());
        let causality_digest = Arc::<str>::from(provenance.causality_digest().to_owned());
        let effect_intent_digest = Arc::<str>::from(provenance.effect_intent_digest().to_owned());
        let effect_intent_patch_canonical_basis =
            Arc::<str>::from(provenance.effect_intent_patch_canonical_basis().to_owned());
        let strategy_descriptor_basis = provenance.strategy_descriptor_basis().clone();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-feedback-context|provenance={}|causality={}|effect-intent={}|effect-intent-basis={}|strategy={}",
            provenance_digest.as_ref(),
            causality_digest.as_ref(),
            effect_intent_digest.as_ref(),
            effect_intent_patch_canonical_basis.as_ref(),
            strategy_descriptor_basis.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            provenance_digest,
            causality_digest,
            effect_intent_digest,
            effect_intent_patch_canonical_basis,
            strategy_descriptor_basis,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-writeback-feedback-context:sha256:{digest:x}"
            )),
        }
    }

    pub fn provenance_digest(&self) -> &str {
        self.provenance_digest.as_ref()
    }

    pub fn causality_digest(&self) -> &str {
        self.causality_digest.as_ref()
    }

    pub fn effect_intent_digest(&self) -> &str {
        self.effect_intent_digest.as_ref()
    }

    pub fn effect_intent_patch_canonical_basis(&self) -> &str {
        self.effect_intent_patch_canonical_basis.as_ref()
    }

    pub fn strategy_descriptor_basis(&self) -> &BridgeWritebackStrategyDescriptorBasis {
        &self.strategy_descriptor_basis
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.strategy_descriptor_basis.digest()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
