use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{BridgeIdentity, LoweredExecutionPolicyIdentityTag};

use super::{AdmittedBridgePolicyContract, BridgeDiagnosticsTier, BridgeExecutionPolicyClass};

pub type LoweredExecutionPolicyIdentity = BridgeIdentity<LoweredExecutionPolicyIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRoutePlanningPolicy {
    lowered_policy_identity: LoweredExecutionPolicyIdentity,
    execution_class: BridgeExecutionPolicyClass,
    diagnostics_tier: BridgeDiagnosticsTier,
    route_artifacts: bool,
    replay_artifacts: bool,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredBridgeExecutionPolicy {
    policy_identity: LoweredExecutionPolicyIdentity,
    contract_identity: super::contracts::BridgePolicyContractIdentity,
    execution_class: BridgeExecutionPolicyClass,
    diagnostics_tier: BridgeDiagnosticsTier,
    route_artifacts: bool,
    replay_artifacts: bool,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl LoweredBridgeExecutionPolicy {
    pub fn from_contract(contract: &AdmittedBridgePolicyContract) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "lowered-bridge-execution-policy|contract={}|execution:{:?}|diagnostics:{:?}|route-artifacts:{}|replay-artifacts:{}",
            contract.contract_identity().as_str(),
            contract.resolved_execution_class(),
            contract.resolved_diagnostics_tier(),
            contract.resolved_route_artifacts(),
            contract.resolved_replay_artifacts(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            policy_identity: LoweredExecutionPolicyIdentity::admit_bridge_owned(format!(
                "lowered-bridge-execution-policy:sha256:{digest:x}"
            )),
            contract_identity: contract.contract_identity().clone(),
            execution_class: contract.resolved_execution_class(),
            diagnostics_tier: contract.resolved_diagnostics_tier(),
            route_artifacts: contract.resolved_route_artifacts(),
            replay_artifacts: contract.resolved_replay_artifacts(),
            canonical_basis,
            digest: Arc::from(format!("lowered-bridge-execution-policy:sha256:{digest:x}")),
        }
    }

    pub fn policy_identity(&self) -> &LoweredExecutionPolicyIdentity {
        &self.policy_identity
    }

    pub fn contract_identity(&self) -> &super::contracts::BridgePolicyContractIdentity {
        &self.contract_identity
    }

    pub fn execution_class(&self) -> BridgeExecutionPolicyClass {
        self.execution_class
    }

    pub fn diagnostics_tier(&self) -> BridgeDiagnosticsTier {
        self.diagnostics_tier
    }

    pub fn route_artifacts(&self) -> bool {
        self.route_artifacts
    }

    pub fn replay_artifacts(&self) -> bool {
        self.replay_artifacts
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }

    pub fn route_planning_policy(&self) -> BridgeRoutePlanningPolicy {
        BridgeRoutePlanningPolicy::from_lowered(self)
    }
}

impl BridgeRoutePlanningPolicy {
    pub fn from_lowered(lowered: &LoweredBridgeExecutionPolicy) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-route-planning-policy|lowered={}|execution:{:?}|diagnostics:{:?}|route-artifacts:{}|replay-artifacts:{}",
            lowered.policy_identity().as_str(),
            lowered.execution_class(),
            lowered.diagnostics_tier(),
            lowered.route_artifacts(),
            lowered.replay_artifacts(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            lowered_policy_identity: lowered.policy_identity().clone(),
            execution_class: lowered.execution_class(),
            diagnostics_tier: lowered.diagnostics_tier(),
            route_artifacts: lowered.route_artifacts(),
            replay_artifacts: lowered.replay_artifacts(),
            canonical_basis,
            digest: Arc::from(format!("bridge-route-planning-policy:sha256:{digest:x}")),
        }
    }

    pub fn lowered_policy_identity(&self) -> &LoweredExecutionPolicyIdentity {
        &self.lowered_policy_identity
    }

    pub fn execution_class(&self) -> BridgeExecutionPolicyClass {
        self.execution_class
    }

    pub fn diagnostics_tier(&self) -> BridgeDiagnosticsTier {
        self.diagnostics_tier
    }

    pub fn route_artifacts(&self) -> bool {
        self.route_artifacts
    }

    pub fn replay_artifacts(&self) -> bool {
        self.replay_artifacts
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
