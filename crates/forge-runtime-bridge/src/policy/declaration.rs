use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::facade::BridgeRequestKind;
use crate::identity::{BridgeIdentity, BridgeIdentityEvidence, PolicyDeclarationIdentityTag};

use super::{BridgeDiagnosticsTier, BridgeExecutionPolicyClass};

pub type BridgePolicyDeclarationIdentity = BridgeIdentity<PolicyDeclarationIdentityTag>;

impl BridgePolicyDeclarationIdentity {
    pub fn from_bridge_evidence(evidence_identity: &BridgeIdentityEvidence) -> Self {
        Self::admit_bridge_owned(format!(
            "bridge-policy-declaration:external-authority-evidence:{}",
            evidence_identity.as_str()
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePolicyDeclaration {
    declaration_identity: BridgePolicyDeclarationIdentity,
    request_kind: BridgeRequestKind,
    execution_class: BridgeExecutionPolicyClass,
    diagnostics_tier: BridgeDiagnosticsTier,
    require_replay_artifacts: bool,
    request_route_artifacts: bool,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgePolicyDeclaration {
    pub fn new(
        declaration_identity: BridgePolicyDeclarationIdentity,
        request_kind: BridgeRequestKind,
        execution_class: BridgeExecutionPolicyClass,
        diagnostics_tier: BridgeDiagnosticsTier,
        require_replay_artifacts: bool,
        request_route_artifacts: bool,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-policy-declaration|id={}|request-kind:{request_kind:?}|execution:{execution_class:?}|diagnostics:{diagnostics_tier:?}|replay:{}|route-artifacts:{}",
            declaration_identity.as_str(),
            require_replay_artifacts,
            request_route_artifacts,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            declaration_identity,
            request_kind,
            execution_class,
            diagnostics_tier,
            require_replay_artifacts,
            request_route_artifacts,
            canonical_basis,
            digest: Arc::from(format!("bridge-policy-declaration:sha256:{digest:x}")),
        }
    }

    pub fn declaration_identity(&self) -> &BridgePolicyDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn request_kind(&self) -> BridgeRequestKind {
        self.request_kind
    }

    pub fn execution_class(&self) -> BridgeExecutionPolicyClass {
        self.execution_class
    }

    pub fn diagnostics_tier(&self) -> BridgeDiagnosticsTier {
        self.diagnostics_tier
    }

    pub fn require_replay_artifacts(&self) -> bool {
        self.require_replay_artifacts
    }

    pub fn request_route_artifacts(&self) -> bool {
        self.request_route_artifacts
    }

    pub const fn policy_field_count(&self) -> usize {
        4
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
    use super::BridgePolicyDeclaration;
    use crate::facade::BridgeRequestKind;
    use crate::policy::{BridgeDiagnosticsTier, BridgeExecutionPolicyClass};

    #[test]
    fn policy_declaration_is_canonical_for_same_inputs() {
        let left = BridgePolicyDeclaration::new(
            super::BridgePolicyDeclarationIdentity::admit_bridge_owned("policy:request-a"),
            BridgeRequestKind::Authoritative,
            BridgeExecutionPolicyClass::DeterministicCanonical,
            BridgeDiagnosticsTier::Standard,
            true,
            true,
        );
        let right = BridgePolicyDeclaration::new(
            super::BridgePolicyDeclarationIdentity::admit_bridge_owned("policy:request-a"),
            BridgeRequestKind::Authoritative,
            BridgeExecutionPolicyClass::DeterministicCanonical,
            BridgeDiagnosticsTier::Standard,
            true,
            true,
        );

        assert_eq!(left, right);
        assert_eq!(left.digest(), right.digest());
    }
}
