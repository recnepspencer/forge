use std::sync::Arc;

use sha2::{Digest, Sha256};
use worth_foundational::facade::{
    AspectContract, AspectContractRevision, AspectIdentity, AspectKey, ScalarAspectType,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotReadContract {
    aspect_contract: AspectContract,
    canonical_basis: Arc<str>,
}

impl SnapshotReadContract {
    pub fn new(aspect_contract: AspectContract) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "snapshot-read-contract|key={}|identity={}|revision={}|shape={:?}",
            aspect_contract.key().as_str(),
            aspect_contract.identity().0,
            aspect_contract.revision().0,
            aspect_contract.shape(),
        ));

        Self {
            aspect_contract,
            canonical_basis,
        }
    }

    pub fn scalar(aspect_key: AspectKey, scalar_type: ScalarAspectType) -> Self {
        let identity = derived_snapshot_contract_identity(&aspect_key, scalar_type);
        Self::new(AspectContract::scalar(
            aspect_key,
            identity,
            AspectContractRevision(1),
            scalar_type,
        ))
    }

    pub fn aspect_contract(&self) -> &AspectContract {
        &self.aspect_contract
    }

    pub fn aspect_key(&self) -> &AspectKey {
        self.aspect_contract.key()
    }

    pub(crate) fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }
}

fn derived_snapshot_contract_identity(
    aspect_key: &AspectKey,
    scalar_type: ScalarAspectType,
) -> AspectIdentity {
    let digest = Sha256::digest(format!("{}:{scalar_type:?}", aspect_key.as_str()).as_bytes());
    let mut identity_seed = [0_u8; 8];
    identity_seed.copy_from_slice(&digest[..8]);
    AspectIdentity(u64::from_be_bytes(identity_seed))
}
