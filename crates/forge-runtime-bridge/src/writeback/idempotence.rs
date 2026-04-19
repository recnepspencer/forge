use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{BridgeIdentity, WritebackIdempotenceIdentityTag};

use super::{BridgeDerivedWritebackEffect, BridgeWritebackIdempotenceClass};

pub type BridgeWritebackIdempotenceIdentity = BridgeIdentity<WritebackIdempotenceIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackIdempotenceBasis {
    idempotence_identity: BridgeWritebackIdempotenceIdentity,
    effect_digest: Arc<str>,
    causality_digest: Arc<str>,
    lowered_policy_digest: Arc<str>,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    strategy_descriptor_digest: Arc<str>,
    authoritative_state_digest: Arc<str>,
    idempotence_class: BridgeWritebackIdempotenceClass,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackIdempotenceBasis {
    pub fn new(
        idempotence_identity: BridgeWritebackIdempotenceIdentity,
        effect: &BridgeDerivedWritebackEffect,
        lowered_policy_digest: impl Into<Arc<str>>,
        authoritative_state_digest: impl Into<Arc<str>>,
        idempotence_class: BridgeWritebackIdempotenceClass,
    ) -> Self {
        let lowered_policy_digest = lowered_policy_digest.into();
        let authoritative_state_digest = authoritative_state_digest.into();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-idempotence-basis|id={}|effect={}|causality={}|lowered-policy={}|family:{:?}|strategy-class:{:?}|strategy={}|authoritative-state={}|class:{idempotence_class:?}",
            idempotence_identity.as_str(),
            effect.effect_digest(),
            effect.causality_digest(),
            lowered_policy_digest.as_ref(),
            effect.family_kind(),
            effect.strategy_class(),
            effect.strategy_descriptor_digest(),
            authoritative_state_digest.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            idempotence_identity,
            effect_digest: Arc::from(effect.effect_digest().to_owned()),
            causality_digest: Arc::from(effect.causality_digest().to_owned()),
            lowered_policy_digest,
            family_kind: effect.family_kind(),
            strategy_class: effect.strategy_class(),
            strategy_descriptor_digest: Arc::from(effect.strategy_descriptor_digest().to_owned()),
            authoritative_state_digest,
            idempotence_class,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-writeback-idempotence-basis:sha256:{digest:x}"
            )),
        }
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }

    pub fn idempotence_identity(&self) -> &BridgeWritebackIdempotenceIdentity {
        &self.idempotence_identity
    }

    pub fn effect_digest(&self) -> &str {
        self.effect_digest.as_ref()
    }

    pub fn causality_digest(&self) -> &str {
        self.causality_digest.as_ref()
    }

    pub fn lowered_policy_digest(&self) -> &str {
        self.lowered_policy_digest.as_ref()
    }

    pub fn strategy_class(&self) -> crate::writeback::BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.strategy_descriptor_digest.as_ref()
    }

    pub fn authoritative_state_digest(&self) -> &str {
        self.authoritative_state_digest.as_ref()
    }

    pub fn idempotence_class(&self) -> BridgeWritebackIdempotenceClass {
        self.idempotence_class
    }
}
