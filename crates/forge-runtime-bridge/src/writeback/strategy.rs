use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{BridgeIdentity, WritebackStrategyIdentityTag};

use super::BridgeWritebackDeclaration;

pub type BridgeWritebackStrategyIdentity = BridgeIdentity<WritebackStrategyIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackStrategyBasis {
    strategy_identity: BridgeWritebackStrategyIdentity,
    declaration_digest: Arc<str>,
    family_digest: Arc<str>,
    family_kind: crate::writeback::BridgeWritebackFamilyKind,
    strategy_class: crate::writeback::BridgeWritebackStrategyClass,
    strategy_descriptor_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackStrategyBasis {
    pub fn from_declaration(declaration: &BridgeWritebackDeclaration) -> Self {
        let family_basis = crate::writeback::BridgeWritebackFamilyBasis::from_declaration(declaration)
            .expect("writeback strategy basis requires explicit admitted family");
        let strategy_class = declaration
            .strategy_class()
            .expect("writeback strategy basis requires explicit strategy class");
        let strategy_identity = BridgeWritebackStrategyIdentity::new(format!(
            "bridge-writeback-strategy:{}",
            declaration.declaration_identity().as_str()
        ));
        let declaration_digest = Arc::<str>::from(declaration.digest().to_owned());
        let family_digest = Arc::<str>::from(family_basis.digest().to_owned());
        let strategy_descriptor_digest =
            Arc::<str>::from(declaration.strategy_descriptor_digest().to_owned());
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-strategy-basis|id={}|declaration={}|family={}|family-kind:{:?}|request-mode:{:?}|effect:{:?}|strategy-class:{strategy_class:?}|strategy={}",
            strategy_identity.as_str(),
            declaration_digest.as_ref(),
            family_digest.as_ref(),
            family_basis.family_kind(),
            declaration.request_mode(),
            declaration.effect_class(),
            strategy_descriptor_digest.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            strategy_identity,
            declaration_digest,
            family_digest,
            family_kind: family_basis.family_kind(),
            strategy_class,
            strategy_descriptor_digest,
            canonical_basis,
            digest: Arc::from(format!("bridge-writeback-strategy-basis:sha256:{digest:x}")),
        }
    }

    pub fn strategy_identity(&self) -> &BridgeWritebackStrategyIdentity {
        &self.strategy_identity
    }

    pub fn declaration_digest(&self) -> &str {
        self.declaration_digest.as_ref()
    }

    pub fn family_digest(&self) -> &str {
        self.family_digest.as_ref()
    }

    pub fn family_kind(&self) -> crate::writeback::BridgeWritebackFamilyKind {
        self.family_kind
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
