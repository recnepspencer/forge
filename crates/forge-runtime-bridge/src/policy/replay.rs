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
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-policy-replay-bundle|contract={}|lowered={}|provenance={}",
            contract.digest(),
            lowered.digest(),
            provenance.digest()
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            contract_digest: Arc::from(contract.digest().to_owned()),
            lowered_policy_digest: Arc::from(lowered.digest().to_owned()),
            provenance_digest: Arc::from(provenance.digest().to_owned()),
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
