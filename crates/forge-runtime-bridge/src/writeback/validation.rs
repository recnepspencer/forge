use std::sync::Arc;

use crate::facade::BridgeRequestKind;

use super::{
    BridgeWritebackDeclaration, BridgeWritebackFamilyBasis, BridgeWritebackRequestMode,
    BridgeWritebackStrategyBasis,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedBridgeWritebackDeclaration {
    declaration: BridgeWritebackDeclaration,
    family_basis: Option<BridgeWritebackFamilyBasis>,
    strategy_basis: Option<BridgeWritebackStrategyBasis>,
    canonical_basis: Arc<str>,
}

impl ValidatedBridgeWritebackDeclaration {
    pub(crate) fn new(
        declaration: BridgeWritebackDeclaration,
    ) -> Result<Self, crate::error::BridgeWritebackError> {
        reject_preview_writeback(&declaration)?;
        reject_read_only_family_binding(&declaration)?;
        reject_read_only_strategy_binding(&declaration)?;
        reject_read_only_strategy_class_binding(&declaration)?;
        reject_writeback_capable_missing_family_kind(&declaration)?;
        reject_writeback_capable_missing_strategy_descriptor(&declaration)?;
        reject_writeback_capable_missing_strategy_class(&declaration)?;
        let family_basis =
            (declaration.request_mode() == BridgeWritebackRequestMode::WritebackCapable)
                .then(|| BridgeWritebackFamilyBasis::from_declaration(&declaration))
                .transpose()?;
        let strategy_basis =
            (declaration.request_mode() == BridgeWritebackRequestMode::WritebackCapable)
                .then(|| BridgeWritebackStrategyBasis::from_declaration(&declaration));

        let canonical_basis = Arc::<str>::from(format!(
            "validated-bridge-writeback-declaration|declaration={}|request-kind:{:?}|request-mode:{:?}|family={}|strategy={}|writeback={}",
            declaration.declaration_identity().as_str(),
            declaration.request_kind(),
            declaration.request_mode(),
            family_basis
                .as_ref()
                .map(|basis| basis.digest())
                .unwrap_or("none"),
            strategy_basis
                .as_ref()
                .map(|basis| basis.digest())
                .unwrap_or("none"),
            declaration.digest(),
        ));

        Ok(Self {
            declaration,
            family_basis,
            strategy_basis,
            canonical_basis,
        })
    }

    pub fn declaration(&self) -> &BridgeWritebackDeclaration {
        &self.declaration
    }

    pub fn strategy_basis(&self) -> Option<&BridgeWritebackStrategyBasis> {
        self.strategy_basis.as_ref()
    }

    pub fn family_basis(&self) -> Option<&BridgeWritebackFamilyBasis> {
        self.family_basis.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }
}

fn reject_preview_writeback(
    declaration: &BridgeWritebackDeclaration,
) -> Result<(), crate::error::BridgeWritebackError> {
    if declaration.request_kind() == BridgeRequestKind::Preview
        && declaration.request_mode() == BridgeWritebackRequestMode::WritebackCapable
    {
        return Err(crate::error::BridgeWritebackError::new(
            crate::error::BridgeWritebackErrorKind::PreviewWritebackRejected,
            format!(
                "Writeback declaration `{}` cannot request truth mutation from preview request kind.",
                declaration.declaration_identity().as_str()
            ),
        ));
    }

    Ok(())
}

fn reject_read_only_family_binding(
    declaration: &BridgeWritebackDeclaration,
) -> Result<(), crate::error::BridgeWritebackError> {
    if declaration.request_mode() == BridgeWritebackRequestMode::ReadOnly
        && declaration.family_kind().is_some()
    {
        return Err(crate::error::BridgeWritebackError::new(
            crate::error::BridgeWritebackErrorKind::WritebackNotRequested,
            format!(
                "Read-only writeback declaration `{}` cannot bind writeback family `{:?}`.",
                declaration.declaration_identity().as_str(),
                declaration.family_kind(),
            ),
        ));
    }

    Ok(())
}

fn reject_read_only_strategy_binding(
    declaration: &BridgeWritebackDeclaration,
) -> Result<(), crate::error::BridgeWritebackError> {
    if declaration.request_mode() == BridgeWritebackRequestMode::ReadOnly
        && !declaration.strategy_descriptor_digest().is_empty()
    {
        return Err(crate::error::BridgeWritebackError::new(
            crate::error::BridgeWritebackErrorKind::WritebackNotRequested,
            format!(
                "Read-only writeback declaration `{}` cannot bind strategy descriptor `{}`.",
                declaration.declaration_identity().as_str(),
                declaration.strategy_descriptor_digest(),
            ),
        ));
    }

    Ok(())
}

fn reject_read_only_strategy_class_binding(
    declaration: &BridgeWritebackDeclaration,
) -> Result<(), crate::error::BridgeWritebackError> {
    if declaration.request_mode() == BridgeWritebackRequestMode::ReadOnly
        && declaration.strategy_class().is_some()
    {
        return Err(crate::error::BridgeWritebackError::new(
            crate::error::BridgeWritebackErrorKind::WritebackNotRequested,
            format!(
                "Read-only writeback declaration `{}` cannot bind strategy class `{:?}`.",
                declaration.declaration_identity().as_str(),
                declaration
                    .strategy_class()
                    .expect("read-only rejection already proved strategy class exists"),
            ),
        ));
    }

    Ok(())
}

fn reject_writeback_capable_missing_family_kind(
    declaration: &BridgeWritebackDeclaration,
) -> Result<(), crate::error::BridgeWritebackError> {
    if declaration.request_mode() == BridgeWritebackRequestMode::WritebackCapable
        && declaration.family_kind().is_none()
    {
        return Err(crate::error::BridgeWritebackError::new(
            crate::error::BridgeWritebackErrorKind::FamilyBindingMismatch,
            format!(
                "Writeback-capable declaration `{}` must bind an explicit writeback family.",
                declaration.declaration_identity().as_str(),
            ),
        ));
    }

    Ok(())
}

fn reject_writeback_capable_missing_strategy_descriptor(
    declaration: &BridgeWritebackDeclaration,
) -> Result<(), crate::error::BridgeWritebackError> {
    if declaration.request_mode() == BridgeWritebackRequestMode::WritebackCapable
        && declaration.strategy_descriptor_digest().trim().is_empty()
    {
        return Err(crate::error::BridgeWritebackError::new(
            crate::error::BridgeWritebackErrorKind::StrategyDescriptorMismatch,
            format!(
                "Writeback-capable declaration `{}` must bind a non-empty strategy descriptor.",
                declaration.declaration_identity().as_str(),
            ),
        ));
    }

    Ok(())
}

fn reject_writeback_capable_missing_strategy_class(
    declaration: &BridgeWritebackDeclaration,
) -> Result<(), crate::error::BridgeWritebackError> {
    if declaration.request_mode() == BridgeWritebackRequestMode::WritebackCapable
        && declaration.strategy_class().is_none()
    {
        return Err(crate::error::BridgeWritebackError::new(
            crate::error::BridgeWritebackErrorKind::StrategyDescriptorMismatch,
            format!(
                "Writeback-capable declaration `{}` must bind an explicit strategy class.",
                declaration.declaration_identity().as_str(),
            ),
        ));
    }

    Ok(())
}
