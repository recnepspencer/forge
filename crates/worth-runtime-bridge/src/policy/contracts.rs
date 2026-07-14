use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{BridgeIdentity, PolicyContractIdentityTag};

use super::{
    BridgeDiagnosticsTier, BridgeExecutionPolicyClass, BridgePolicyFieldKind,
    BridgePolicyResolution, BridgePolicySourceClass, ValidatedBridgePolicyDeclaration,
};

pub type BridgePolicyContractIdentity = BridgeIdentity<PolicyContractIdentityTag>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BridgePolicyAuthorityInputs {
    baseline_diagnostics_tier: BridgeDiagnosticsTier,
    replay_artifacts_permitted: bool,
    route_artifacts_permitted: bool,
}

impl BridgePolicyAuthorityInputs {
    pub fn new(
        baseline_diagnostics_tier: BridgeDiagnosticsTier,
        replay_artifacts_permitted: bool,
        route_artifacts_permitted: bool,
    ) -> Self {
        Self {
            baseline_diagnostics_tier,
            replay_artifacts_permitted,
            route_artifacts_permitted,
        }
    }

    pub fn baseline_diagnostics_tier(&self) -> BridgeDiagnosticsTier {
        self.baseline_diagnostics_tier
    }

    pub fn replay_artifacts_permitted(&self) -> bool {
        self.replay_artifacts_permitted
    }

    pub fn route_artifacts_permitted(&self) -> bool {
        self.route_artifacts_permitted
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BridgePolicyResolutionEntry {
    field_kind: BridgePolicyFieldKind,
    declared_source: BridgePolicySourceClass,
    operative_source: BridgePolicySourceClass,
    resolution: BridgePolicyResolution,
}

impl BridgePolicyResolutionEntry {
    pub fn new(
        field_kind: BridgePolicyFieldKind,
        declared_source: BridgePolicySourceClass,
        operative_source: BridgePolicySourceClass,
        resolution: BridgePolicyResolution,
    ) -> Self {
        Self {
            field_kind,
            declared_source,
            operative_source,
            resolution,
        }
    }

    pub fn field_kind(&self) -> BridgePolicyFieldKind {
        self.field_kind
    }

    pub fn declared_source(&self) -> BridgePolicySourceClass {
        self.declared_source
    }

    pub fn operative_source(&self) -> BridgePolicySourceClass {
        self.operative_source
    }

    pub fn resolution(&self) -> BridgePolicyResolution {
        self.resolution
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedBridgePolicyContract {
    contract_identity: BridgePolicyContractIdentity,
    validated_declaration: ValidatedBridgePolicyDeclaration,
    authority_inputs: BridgePolicyAuthorityInputs,
    resolved_execution_class: BridgeExecutionPolicyClass,
    resolved_diagnostics_tier: BridgeDiagnosticsTier,
    resolved_route_artifacts: bool,
    resolved_replay_artifacts: bool,
    resolution_entries: Arc<[BridgePolicyResolutionEntry]>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

pub struct AdmittedBridgePolicyContractParts {
    pub validated_declaration: ValidatedBridgePolicyDeclaration,
    pub authority_inputs: BridgePolicyAuthorityInputs,
    pub resolved_execution_class: BridgeExecutionPolicyClass,
    pub resolved_diagnostics_tier: BridgeDiagnosticsTier,
    pub resolved_route_artifacts: bool,
    pub resolved_replay_artifacts: bool,
    pub resolution_entries: Vec<BridgePolicyResolutionEntry>,
}

impl AdmittedBridgePolicyContract {
    pub fn new(parts: AdmittedBridgePolicyContractParts) -> Self {
        let entries = Arc::<[BridgePolicyResolutionEntry]>::from(parts.resolution_entries);
        let entry_basis = entries
            .iter()
            .map(|entry| {
                format!(
                    "{:?}|{:?}|{:?}|{:?}",
                    entry.field_kind(),
                    entry.declared_source(),
                    entry.operative_source(),
                    entry.resolution(),
                )
            })
            .collect::<Vec<_>>()
            .join("|");
        let canonical_basis = Arc::<str>::from(format!(
            "admitted-bridge-policy-contract|declaration={}|validated={}|baseline-diagnostics:{:?}|replay-permitted:{}|route-permitted:{}|execution:{:?}|diagnostics:{:?}|route-artifacts:{}|replay-artifacts:{}|entry-count:{}|entries:{}",
            parts
                .validated_declaration
                .declaration()
                .declaration_identity()
                .as_str(),
            parts.validated_declaration.canonical_basis(),
            parts.authority_inputs.baseline_diagnostics_tier(),
            parts.authority_inputs.replay_artifacts_permitted(),
            parts.authority_inputs.route_artifacts_permitted(),
            parts.resolved_execution_class,
            parts.resolved_diagnostics_tier,
            parts.resolved_route_artifacts,
            parts.resolved_replay_artifacts,
            entries.len(),
            entry_basis,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            contract_identity: BridgePolicyContractIdentity::admit_bridge_owned(format!(
                "bridge-policy-contract:sha256:{digest:x}"
            )),
            validated_declaration: parts.validated_declaration,
            authority_inputs: parts.authority_inputs,
            resolved_execution_class: parts.resolved_execution_class,
            resolved_diagnostics_tier: parts.resolved_diagnostics_tier,
            resolved_route_artifacts: parts.resolved_route_artifacts,
            resolved_replay_artifacts: parts.resolved_replay_artifacts,
            resolution_entries: entries,
            canonical_basis,
            digest: Arc::from(format!("admitted-bridge-policy-contract:sha256:{digest:x}")),
        }
    }

    pub fn contract_identity(&self) -> &BridgePolicyContractIdentity {
        &self.contract_identity
    }

    pub fn validated_declaration(&self) -> &ValidatedBridgePolicyDeclaration {
        &self.validated_declaration
    }

    pub fn authority_inputs(&self) -> BridgePolicyAuthorityInputs {
        self.authority_inputs
    }

    pub fn resolved_execution_class(&self) -> BridgeExecutionPolicyClass {
        self.resolved_execution_class
    }

    pub fn resolved_diagnostics_tier(&self) -> BridgeDiagnosticsTier {
        self.resolved_diagnostics_tier
    }

    pub fn resolved_route_artifacts(&self) -> bool {
        self.resolved_route_artifacts
    }

    pub fn resolved_replay_artifacts(&self) -> bool {
        self.resolved_replay_artifacts
    }

    pub fn resolution_entries(&self) -> &[BridgePolicyResolutionEntry] {
        &self.resolution_entries
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
