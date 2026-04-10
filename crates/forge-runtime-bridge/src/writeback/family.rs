use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::error::{BridgeWritebackError, BridgeWritebackErrorKind};
use crate::identity::{BridgeIdentity, WritebackFamilyIdentityTag};

use super::{
    BridgeWritebackDeclaration, BridgeWritebackEffectClass, BridgeWritebackFamilyKind,
    BridgeWritebackIdempotenceClass, BridgeWritebackStrategyClass,
};

pub type BridgeWritebackFamilyIdentity = BridgeIdentity<WritebackFamilyIdentityTag>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BridgeWritebackFamilyRegistryEntry {
    family_kind: BridgeWritebackFamilyKind,
    effect_class: BridgeWritebackEffectClass,
    strategy_class: BridgeWritebackStrategyClass,
    idempotence_class: BridgeWritebackIdempotenceClass,
}

impl BridgeWritebackFamilyRegistryEntry {
    #[cfg(test)]
    pub const fn family_kind(&self) -> BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub const fn effect_class(&self) -> BridgeWritebackEffectClass {
        self.effect_class
    }

    pub const fn strategy_class(&self) -> BridgeWritebackStrategyClass {
        self.strategy_class
    }

    pub const fn idempotence_class(&self) -> BridgeWritebackIdempotenceClass {
        self.idempotence_class
    }
}

const fn registry_entry(family_kind: BridgeWritebackFamilyKind) -> BridgeWritebackFamilyRegistryEntry {
    match family_kind {
        BridgeWritebackFamilyKind::ProjectedStateDiff => BridgeWritebackFamilyRegistryEntry {
            family_kind: BridgeWritebackFamilyKind::ProjectedStateDiff,
            effect_class: BridgeWritebackEffectClass::ProjectedStateDiff,
            strategy_class: BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
            idempotence_class: BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
        },
        BridgeWritebackFamilyKind::AspectReconciliation => BridgeWritebackFamilyRegistryEntry {
            family_kind: BridgeWritebackFamilyKind::AspectReconciliation,
            effect_class: BridgeWritebackEffectClass::AspectReconciliation,
            strategy_class: BridgeWritebackStrategyClass::AspectReconciliationCommit,
            idempotence_class: BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
        },
    }
}

pub(crate) fn admitted_family_registry_entry(
    family_kind: BridgeWritebackFamilyKind,
) -> BridgeWritebackFamilyRegistryEntry {
    registry_entry(family_kind)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeWritebackFamilyBasis {
    family_identity: BridgeWritebackFamilyIdentity,
    declaration_digest: Arc<str>,
    family_kind: BridgeWritebackFamilyKind,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeWritebackFamilyBasis {
    pub(crate) fn from_declaration(
        declaration: &BridgeWritebackDeclaration,
    ) -> Result<Self, BridgeWritebackError> {
        let family_kind = declaration.family_kind().ok_or_else(|| {
            BridgeWritebackError::new(
                BridgeWritebackErrorKind::FamilyBindingMismatch,
                format!(
                    "Writeback-capable declaration `{}` must bind an explicit writeback family.",
                    declaration.declaration_identity().as_str()
                ),
            )
        })?;
        let registry_entry = admitted_family_registry_entry(family_kind);

        if declaration.effect_class() != registry_entry.effect_class() {
            return Err(BridgeWritebackError::new(
                BridgeWritebackErrorKind::FamilyBindingMismatch,
                format!(
                    "Writeback declaration `{}` binds family `{:?}` but effect class `{:?}` instead of `{:?}`.",
                    declaration.declaration_identity().as_str(),
                    family_kind,
                    declaration.effect_class(),
                    registry_entry.effect_class(),
                ),
            ));
        }

        if declaration.strategy_class() != Some(registry_entry.strategy_class()) {
            return Err(BridgeWritebackError::new(
                BridgeWritebackErrorKind::FamilyBindingMismatch,
                format!(
                    "Writeback declaration `{}` binds family `{:?}` but strategy class `{:?}` instead of `{:?}`.",
                    declaration.declaration_identity().as_str(),
                    family_kind,
                    declaration.strategy_class(),
                    registry_entry.strategy_class(),
                ),
            ));
        }

        if declaration.idempotence_class() != registry_entry.idempotence_class() {
            return Err(BridgeWritebackError::new(
                BridgeWritebackErrorKind::FamilyBindingMismatch,
                format!(
                    "Writeback declaration `{}` binds family `{:?}` but idempotence class `{:?}` instead of `{:?}`.",
                    declaration.declaration_identity().as_str(),
                    family_kind,
                    declaration.idempotence_class(),
                    registry_entry.idempotence_class(),
                ),
            ));
        }

        let declaration_digest = Arc::<str>::from(declaration.digest().to_owned());
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-writeback-family-basis|declaration={}|family:{family_kind:?}|effect:{:?}|strategy:{:?}|idempotence:{:?}",
            declaration_digest.as_ref(),
            registry_entry.effect_class(),
            registry_entry.strategy_class(),
            registry_entry.idempotence_class(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Ok(Self {
            family_identity: BridgeWritebackFamilyIdentity::new(format!(
                "bridge-writeback-family:sha256:{digest:x}"
            )),
            declaration_digest,
            family_kind,
            canonical_basis,
            digest: Arc::from(format!("bridge-writeback-family-basis:sha256:{digest:x}")),
        })
    }

    pub fn family_identity(&self) -> &BridgeWritebackFamilyIdentity {
        &self.family_identity
    }

    pub fn declaration_digest(&self) -> &str {
        self.declaration_digest.as_ref()
    }

    pub fn family_kind(&self) -> BridgeWritebackFamilyKind {
        self.family_kind
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::{admitted_family_registry_entry, BridgeWritebackFamilyKind};

    #[test]
    fn writeback_family_registry_remains_closed_world_for_phase_1() {
        let first = admitted_family_registry_entry(BridgeWritebackFamilyKind::ProjectedStateDiff);
        let second =
            admitted_family_registry_entry(BridgeWritebackFamilyKind::AspectReconciliation);

        assert_eq!(first.family_kind(), BridgeWritebackFamilyKind::ProjectedStateDiff);
        assert_eq!(
            second.family_kind(),
            BridgeWritebackFamilyKind::AspectReconciliation
        );
    }
}
