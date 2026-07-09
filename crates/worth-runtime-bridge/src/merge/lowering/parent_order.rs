use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::merge::AdmittedMergeHistoryContract;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMergeParentOrderDigestBasis {
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeMergeParentOrderDigestBasis {
    pub(crate) fn from_contract(contract: &AdmittedMergeHistoryContract) -> Self {
        let proof = contract
            .validated_declaration()
            .declaration()
            .authority_basis()
            .parent_order_proof();
        let canonical_basis = Arc::<str>::from(format!(
            "merge-parent-order-digest-basis|proof={}|parents={}",
            proof.digest(),
            proof
                .parents()
                .iter()
                .map(|parent| parent.as_str())
                .collect::<Vec<_>>()
                .join(","),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            canonical_basis,
            digest: Arc::from(format!("merge-parent-order-digest-basis:sha256:{digest:x}")),
        }
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
