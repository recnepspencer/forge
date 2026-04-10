use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{
    BridgeIdentity, WritebackCausalityIdentityTag, WritebackEffectIdentityTag,
};

use super::{BridgeMappedWritebackFamilyInput, BridgeWritebackEffectClass};

pub type BridgeWritebackCausalityIdentity = BridgeIdentity<WritebackCausalityIdentityTag>;
pub type BridgeWritebackEffectIdentity = BridgeIdentity<WritebackEffectIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackCausalityBasis {
    causality_identity: BridgeWritebackCausalityIdentity,
    truth_trigger_digest: Arc<str>,
    route_digest: Arc<str>,
    evaluation_surface_digest: Arc<str>,
    truth_view_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackCausalityBasis {
    pub fn new(
        causality_identity: BridgeWritebackCausalityIdentity,
        truth_trigger_digest: impl Into<Arc<str>>,
        route_digest: impl Into<Arc<str>>,
        evaluation_surface_digest: impl Into<Arc<str>>,
        truth_view_digest: impl Into<Arc<str>>,
    ) -> Self {
        let truth_trigger_digest = truth_trigger_digest.into();
        let route_digest = route_digest.into();
        let evaluation_surface_digest = evaluation_surface_digest.into();
        let truth_view_digest = truth_view_digest.into();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-causality|id={}|truth-trigger={}|route={}|evaluation={}|truth-view={}",
            causality_identity.as_str(),
            truth_trigger_digest.as_ref(),
            route_digest.as_ref(),
            evaluation_surface_digest.as_ref(),
            truth_view_digest.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            causality_identity,
            truth_trigger_digest,
            route_digest,
            evaluation_surface_digest,
            truth_view_digest,
            canonical_basis,
            digest: Arc::from(format!("bridge-writeback-causality:sha256:{digest:x}")),
        }
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }

    pub fn truth_trigger_digest(&self) -> &str {
        self.truth_trigger_digest.as_ref()
    }

    pub fn route_digest(&self) -> &str {
        self.route_digest.as_ref()
    }

    pub fn evaluation_surface_digest(&self) -> &str {
        self.evaluation_surface_digest.as_ref()
    }

    pub fn truth_view_digest(&self) -> &str {
        self.truth_view_digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackFeedbackProvenance {
    contract_digest: Arc<str>,
    derived_effect_digest: Arc<str>,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    effect_class: BridgeWritebackEffectClass,
    effect_digest: Arc<str>,
    causality_digest: Arc<str>,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    strategy_descriptor_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackFeedbackProvenance {
    pub fn new(effect: &BridgeDerivedWritebackEffect) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-feedback-provenance|family:{:?}|effect:{:?}|effect-digest={}|causality={}|strategy-class:{:?}|strategy={}",
            effect.family_kind(),
            effect.effect_class(),
            effect.effect_digest(),
            effect.causality_digest(),
            effect.strategy_class(),
            effect.strategy_descriptor_digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            contract_digest: Arc::from(effect.contract_digest().to_owned()),
            derived_effect_digest: Arc::from(effect.digest().to_owned()),
            family_kind: effect.family_kind(),
            effect_class: effect.effect_class(),
            effect_digest: Arc::from(effect.effect_digest().to_owned()),
            causality_digest: Arc::from(effect.causality_digest().to_owned()),
            strategy_class: effect.strategy_class(),
            strategy_descriptor_digest: Arc::from(effect.strategy_descriptor_digest().to_owned()),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-writeback-feedback-provenance:sha256:{digest:x}"
            )),
        }
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_digest.as_ref()
    }

    pub fn derived_effect_digest(&self) -> &str {
        self.derived_effect_digest.as_ref()
    }

    pub fn effect_class(&self) -> BridgeWritebackEffectClass {
        self.effect_class
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn effect_digest(&self) -> &str {
        self.effect_digest.as_ref()
    }

    pub fn causality_digest(&self) -> &str {
        self.causality_digest.as_ref()
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.strategy_descriptor_digest.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeDerivedWritebackEffect {
    effect_identity: BridgeWritebackEffectIdentity,
    mapper_envelope_digest: Arc<str>,
    mapped_input_digest: Arc<str>,
    contract_digest: Arc<str>,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    effect_class: BridgeWritebackEffectClass,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    strategy_descriptor_digest: Arc<str>,
    causality_digest: Arc<str>,
    effect_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeDerivedWritebackEffect {
    pub fn new(
        effect_identity: BridgeWritebackEffectIdentity,
        mapped_input: &BridgeMappedWritebackFamilyInput,
    ) -> Self {
        let contract_digest = Arc::<str>::from(mapped_input.contract_digest().to_owned());
        let strategy_descriptor_digest =
            Arc::<str>::from(mapped_input.strategy_descriptor_digest().to_owned());
        let causality_digest = Arc::<str>::from(mapped_input.causality_digest().to_owned());
        let effect_digest = Arc::<str>::from(mapped_input.domain_payload_digest().to_owned());
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-derived-writeback-effect|id={}|mapped-input={}|contract={}|family:{:?}|effect:{:?}|strategy-class:{:?}|strategy={}|causality={}|effect-digest={}|domain-evidence={}",
            effect_identity.as_str(),
            mapped_input.digest(),
            contract_digest.as_ref(),
            mapped_input.family_kind(),
            mapped_input.effect_class(),
            mapped_input.strategy_class(),
            strategy_descriptor_digest.as_ref(),
            causality_digest.as_ref(),
            effect_digest.as_ref(),
            mapped_input.domain_evidence_digest(),
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
            strategy_descriptor_digest,
            causality_digest,
            effect_digest,
            canonical_basis,
            digest: Arc::from(format!("bridge-derived-writeback-effect:sha256:{digest:x}")),
        }
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.strategy_descriptor_digest.as_ref()
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

    pub fn effect_digest(&self) -> &str {
        self.effect_digest.as_ref()
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
