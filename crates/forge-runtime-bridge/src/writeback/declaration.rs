use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::facade::BridgeRequestKind;
use crate::identity::{BridgeIdentity, WritebackDeclarationIdentityTag};

use super::{
    BridgeWritebackEffectClass, BridgeWritebackFamilyKind, BridgeWritebackIdempotenceClass,
    BridgeWritebackRequestMode, BridgeWritebackStrategyClass,
};

pub type BridgeWritebackDeclarationIdentity = BridgeIdentity<WritebackDeclarationIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackDeclaration {
    declaration_identity: BridgeWritebackDeclarationIdentity,
    request_kind: BridgeRequestKind,
    request_mode: BridgeWritebackRequestMode,
    family_kind: Option<BridgeWritebackFamilyKind>,
    effect_class: BridgeWritebackEffectClass,
    strategy_class: Option<BridgeWritebackStrategyClass>,
    strategy_descriptor_digest: Arc<str>,
    idempotence_class: BridgeWritebackIdempotenceClass,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackDeclaration {
    pub(crate) fn new(
        declaration_identity: BridgeWritebackDeclarationIdentity,
        request_kind: BridgeRequestKind,
        request_mode: BridgeWritebackRequestMode,
        family_kind: Option<BridgeWritebackFamilyKind>,
        effect_class: BridgeWritebackEffectClass,
        strategy_class: Option<BridgeWritebackStrategyClass>,
        strategy_descriptor_digest: impl Into<Arc<str>>,
        idempotence_class: BridgeWritebackIdempotenceClass,
    ) -> Self {
        let strategy_descriptor_digest = strategy_descriptor_digest.into();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-declaration|id={}|request-kind:{request_kind:?}|request-mode:{request_mode:?}|family:{}|effect:{effect_class:?}|strategy-class:{}|strategy={}|idempotence:{idempotence_class:?}",
            declaration_identity.as_str(),
            family_kind
                .map(|kind| format!("{kind:?}"))
                .unwrap_or_else(|| "none".to_string()),
            strategy_class
                .map(|class| format!("{class:?}"))
                .unwrap_or_else(|| "none".to_string()),
            strategy_descriptor_digest.as_ref(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            declaration_identity,
            request_kind,
            request_mode,
            family_kind,
            effect_class,
            strategy_class,
            strategy_descriptor_digest,
            idempotence_class,
            canonical_basis,
            digest: Arc::from(format!("bridge-writeback-declaration:sha256:{digest:x}")),
        }
    }

    pub fn read_only(
        declaration_identity: BridgeWritebackDeclarationIdentity,
        request_kind: BridgeRequestKind,
        effect_class: BridgeWritebackEffectClass,
        idempotence_class: BridgeWritebackIdempotenceClass,
    ) -> Self {
        Self::new(
            declaration_identity,
            request_kind,
            BridgeWritebackRequestMode::ReadOnly,
            None,
            effect_class,
            None,
            "",
            idempotence_class,
        )
    }

    pub fn writeback_capable(
        declaration_identity: BridgeWritebackDeclarationIdentity,
        request_kind: BridgeRequestKind,
        family_kind: BridgeWritebackFamilyKind,
        effect_class: BridgeWritebackEffectClass,
        strategy_class: BridgeWritebackStrategyClass,
        strategy_descriptor_digest: impl Into<Arc<str>>,
        idempotence_class: BridgeWritebackIdempotenceClass,
    ) -> Self {
        Self::new(
            declaration_identity,
            request_kind,
            BridgeWritebackRequestMode::WritebackCapable,
            Some(family_kind),
            effect_class,
            Some(strategy_class),
            strategy_descriptor_digest,
            idempotence_class,
        )
    }

    pub fn declaration_identity(&self) -> &BridgeWritebackDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn request_kind(&self) -> BridgeRequestKind {
        self.request_kind
    }

    pub fn request_mode(&self) -> BridgeWritebackRequestMode {
        self.request_mode
    }

    pub fn effect_class(&self) -> BridgeWritebackEffectClass {
        self.effect_class
    }

    pub fn family_kind(&self) -> Option<BridgeWritebackFamilyKind> {
        self.family_kind
    }

    pub fn strategy_class(&self) -> Option<BridgeWritebackStrategyClass> {
        self.strategy_class
    }

    pub fn strategy_descriptor_digest(&self) -> &str {
        self.strategy_descriptor_digest.as_ref()
    }

    pub fn idempotence_class(&self) -> BridgeWritebackIdempotenceClass {
        self.idempotence_class
    }

    pub const fn declaration_field_count(&self) -> usize {
        7
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
