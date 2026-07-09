use std::sync::Arc;

use crate::facade::BridgeRequestKind;

use super::{
    BridgeDiagnosticsTier, BridgeExecutionPolicyClass, BridgePolicyDeclaration,
    BridgePolicyFieldKind, BridgePolicyRejection, BridgePolicyRejectionKind,
    BridgePolicyRejectionStage, BridgePolicySourceClass,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedBridgePolicyDeclaration {
    declaration: BridgePolicyDeclaration,
    canonical_basis: Arc<str>,
}

impl ValidatedBridgePolicyDeclaration {
    pub fn new(declaration: BridgePolicyDeclaration) -> Result<Self, BridgePolicyRejection> {
        reject_self_conflicting_execution_bundle(&declaration)?;
        reject_self_conflicting_replay_bundle(&declaration)?;

        let canonical_basis = Arc::<str>::from(format!(
            "validated-bridge-policy-declaration|declaration={}|request-kind:{:?}|policy={}",
            declaration.declaration_identity().as_str(),
            declaration.request_kind(),
            declaration.digest(),
        ));
        Ok(Self {
            declaration,
            canonical_basis,
        })
    }

    pub fn declaration(&self) -> &BridgePolicyDeclaration {
        &self.declaration
    }

    pub fn request_kind(&self) -> BridgeRequestKind {
        self.declaration.request_kind()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }
}

fn reject_self_conflicting_execution_bundle(
    declaration: &BridgePolicyDeclaration,
) -> Result<(), BridgePolicyRejection> {
    if declaration.request_kind() == BridgeRequestKind::Authoritative
        && declaration.execution_class() == BridgeExecutionPolicyClass::Optimized
    {
        return Err(BridgePolicyRejection::new(
            declaration,
            BridgePolicyRejectionKind::UnsupportedExecutionMode,
            BridgePolicyRejectionStage::Validation,
            BridgePolicyFieldKind::ExecutionMode,
            BridgePolicySourceClass::RequestDeclared,
            BridgePolicySourceClass::SpeculationLifecycleAdmitted,
            "authoritative requests currently admit deterministic canonical execution only",
        ));
    }

    Ok(())
}

fn reject_self_conflicting_replay_bundle(
    declaration: &BridgePolicyDeclaration,
) -> Result<(), BridgePolicyRejection> {
    if declaration.require_replay_artifacts() && !declaration.request_route_artifacts() {
        return Err(BridgePolicyRejection::new(
            declaration,
            BridgePolicyRejectionKind::ReplayPolicyConflict,
            BridgePolicyRejectionStage::Validation,
            BridgePolicyFieldKind::ReplayArtifacts,
            BridgePolicySourceClass::RequestDeclared,
            BridgePolicySourceClass::RequestDeclared,
            "replay artifacts require route artifacts so canonical route records exist for reconstruction",
        ));
    }

    if declaration.require_replay_artifacts()
        && declaration.diagnostics_tier() == BridgeDiagnosticsTier::Minimal
    {
        return Err(BridgePolicyRejection::new(
            declaration,
            BridgePolicyRejectionKind::DiagnosticsPolicyConflict,
            BridgePolicyRejectionStage::Validation,
            BridgePolicyFieldKind::DiagnosticsTier,
            BridgePolicySourceClass::RequestDeclared,
            BridgePolicySourceClass::RequestDeclared,
            "replay-capable policy bundles require at least standard diagnostics richness",
        ));
    }

    Ok(())
}
