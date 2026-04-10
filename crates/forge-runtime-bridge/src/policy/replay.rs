use std::sync::Arc;

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePolicyReplayBundle {
    contract_digest: Arc<str>,
    lowered_policy_digest: Arc<str>,
    provenance_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgePolicyReplayBundle {
    pub fn from_canonical_records(
        contract: &super::AdmittedBridgePolicyContract,
        lowered: &super::LoweredBridgeExecutionPolicy,
        provenance: &super::BridgePolicyProvenanceRecord,
    ) -> Self {
        Self::from_digests(contract.digest(), lowered.digest(), provenance.digest())
    }

    pub fn from_digests(
        contract_digest: impl Into<Arc<str>>,
        lowered_policy_digest: impl Into<Arc<str>>,
        provenance_digest: impl Into<Arc<str>>,
    ) -> Self {
        let contract_digest = contract_digest.into();
        let lowered_policy_digest = lowered_policy_digest.into();
        let provenance_digest = provenance_digest.into();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-policy-replay-bundle|contract={}|lowered={}|provenance={}",
            contract_digest, lowered_policy_digest, provenance_digest
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            contract_digest,
            lowered_policy_digest,
            provenance_digest,
            canonical_basis,
            digest: Arc::from(format!("bridge-policy-replay-bundle:sha256:{digest:x}")),
        }
    }

    pub fn contract_digest(&self) -> &str {
        self.contract_digest.as_ref()
    }

    pub fn lowered_policy_digest(&self) -> &str {
        self.lowered_policy_digest.as_ref()
    }

    pub fn provenance_digest(&self) -> &str {
        self.provenance_digest.as_ref()
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
