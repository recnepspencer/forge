use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{BridgeIdentity, PolicyProvenanceIdentityTag};

use super::{
    AdmittedBridgePolicyContract, BridgePolicyFieldKind, BridgePolicyResolution,
    BridgePolicySourceClass, LoweredBridgeExecutionPolicy,
};

pub type BridgePolicyProvenanceIdentity = BridgeIdentity<PolicyProvenanceIdentityTag>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BridgePolicyProvenanceEntry {
    field_kind: BridgePolicyFieldKind,
    declared_source: BridgePolicySourceClass,
    operative_source: BridgePolicySourceClass,
    resolution: BridgePolicyResolution,
}

impl BridgePolicyProvenanceEntry {
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
pub struct BridgePolicyProvenanceRecord {
    provenance_identity: BridgePolicyProvenanceIdentity,
    contract_identity: super::contracts::BridgePolicyContractIdentity,
    lowered_policy_identity: super::lowering::LoweredExecutionPolicyIdentity,
    entries: Arc<[BridgePolicyProvenanceEntry]>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgePolicyProvenanceRecord {
    pub fn from_contract_and_lowered(
        contract: &AdmittedBridgePolicyContract,
        lowered: &LoweredBridgeExecutionPolicy,
    ) -> Self {
        let entries = Arc::<[BridgePolicyProvenanceEntry]>::from(
            contract
                .resolution_entries()
                .iter()
                .map(|entry| {
                    BridgePolicyProvenanceEntry::new(
                        entry.field_kind(),
                        entry.declared_source(),
                        entry.operative_source(),
                        entry.resolution(),
                    )
                })
                .collect::<Vec<_>>(),
        );
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
            "bridge-policy-provenance|contract={}|lowered={}|entry-count:{}|entries:{}",
            contract.contract_identity().as_str(),
            lowered.policy_identity().as_str(),
            entries.len(),
            entry_basis,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            provenance_identity: BridgePolicyProvenanceIdentity::new(format!(
                "bridge-policy-provenance:sha256:{digest:x}"
            )),
            contract_identity: contract.contract_identity().clone(),
            lowered_policy_identity: lowered.policy_identity().clone(),
            entries,
            canonical_basis,
            digest: Arc::from(format!("bridge-policy-provenance:sha256:{digest:x}")),
        }
    }

    pub fn provenance_identity(&self) -> &BridgePolicyProvenanceIdentity {
        &self.provenance_identity
    }

    pub fn contract_identity(&self) -> &super::contracts::BridgePolicyContractIdentity {
        &self.contract_identity
    }

    pub fn lowered_policy_identity(&self) -> &super::lowering::LoweredExecutionPolicyIdentity {
        &self.lowered_policy_identity
    }

    pub fn entries(&self) -> &[BridgePolicyProvenanceEntry] {
        &self.entries
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
