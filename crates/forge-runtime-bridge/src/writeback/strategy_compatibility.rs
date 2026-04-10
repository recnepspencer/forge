use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{BridgeIdentity, WritebackStrategyCompatibilityIdentityTag};

use super::{
    AdmittedBridgeWritebackContract, BridgeDerivedWritebackEffect, BridgeWritebackIdempotenceBasis,
};

pub type BridgeWritebackStrategyCompatibilityIdentity =
    BridgeIdentity<WritebackStrategyCompatibilityIdentityTag>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeWritebackStrategyCompatibilityDisposition {
    Compatible,
    FamilyKindMismatch,
    StrategyClassMismatch,
    StrategyDescriptorMismatch,
    EffectClassMismatch,
    IdempotenceClassMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackStrategyCompatibilityReport {
    compatibility_identity: BridgeWritebackStrategyCompatibilityIdentity,
    contract_digest: Arc<str>,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    strategy_basis_digest: Arc<str>,
    effect_digest: Arc<str>,
    idempotence_digest: Arc<str>,
    disposition: BridgeWritebackStrategyCompatibilityDisposition,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackStrategyCompatibilityReport {
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
            BridgeWritebackStrategyCompatibilityDisposition::FamilyKindMismatch
        } else if strategy_basis.strategy_class() != effect.strategy_class()
            || strategy_basis.strategy_class() != idempotence.strategy_class()
        {
            BridgeWritebackStrategyCompatibilityDisposition::StrategyClassMismatch
        } else if strategy_basis.strategy_descriptor_digest()
            != effect.strategy_descriptor_digest()
            || strategy_basis.strategy_descriptor_digest() != idempotence.strategy_descriptor_digest()
        {
            BridgeWritebackStrategyCompatibilityDisposition::StrategyDescriptorMismatch
        } else if declaration.effect_class() != effect.effect_class() {
            BridgeWritebackStrategyCompatibilityDisposition::EffectClassMismatch
        } else if declaration.idempotence_class() != idempotence.idempotence_class() {
            BridgeWritebackStrategyCompatibilityDisposition::IdempotenceClassMismatch
        } else {
            BridgeWritebackStrategyCompatibilityDisposition::Compatible
        };
        let contract_digest = Arc::<str>::from(contract.digest().to_owned());
        let strategy_basis_digest = Arc::<str>::from(strategy_basis.digest().to_owned());
        let effect_digest = Arc::<str>::from(effect.digest().to_owned());
        let idempotence_digest = Arc::<str>::from(idempotence.digest().to_owned());
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-strategy-compatibility|contract={}|family:{:?}|strategy-basis={}|effect={}|idempotence={}|strategy-class:{:?}|effect-strategy-class:{:?}|idempotence-strategy-class:{:?}|contract-effect:{:?}|effect-effect:{:?}|contract-idempotence:{:?}|idempotence-class:{:?}|disposition:{disposition:?}",
            contract_digest.as_ref(),
            strategy_basis.family_kind(),
            strategy_basis_digest.as_ref(),
            effect_digest.as_ref(),
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
            compatibility_identity: BridgeWritebackStrategyCompatibilityIdentity::new(format!(
                "bridge-writeback-strategy-compatibility:sha256:{digest:x}"
            )),
            contract_digest,
            family_kind: strategy_basis.family_kind(),
            strategy_basis_digest,
            effect_digest,
            idempotence_digest,
            disposition,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-writeback-strategy-compatibility:sha256:{digest:x}"
            )),
        }
    }

    pub fn disposition(&self) -> BridgeWritebackStrategyCompatibilityDisposition {
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

    pub fn effect_digest(&self) -> &str {
        self.effect_digest.as_ref()
    }

    pub fn idempotence_digest(&self) -> &str {
        self.idempotence_digest.as_ref()
    }
}
