use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{
    BridgeIdentity, MergeAuthorityBasisIdentityTag, MergeParentOrderIdentityTag,
};
use crate::input::envelope::TruthCommitIdentity;

use super::taxonomy::BridgeMergeAuthorityBasisKind;

pub type BridgeMergeAuthorityBasisIdentity = BridgeIdentity<MergeAuthorityBasisIdentityTag>;
pub type BridgeMergeParentOrderProofIdentity = BridgeIdentity<MergeParentOrderIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMergeParentOrderProof {
    proof_identity: BridgeMergeParentOrderProofIdentity,
    parents: Arc<[TruthCommitIdentity]>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeMergeParentOrderProof {
    pub fn new(parents: Vec<TruthCommitIdentity>) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "merge-parent-order-proof|parents={}",
            parents
                .iter()
                .map(TruthCommitIdentity::as_str)
                .collect::<Vec<_>>()
                .join(","),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            proof_identity: BridgeMergeParentOrderProofIdentity::admit_bridge_owned(format!(
                "merge-parent-order-proof:sha256:{digest:x}"
            )),
            parents: Arc::from(parents),
            canonical_basis,
            digest: Arc::from(format!("merge-parent-order-proof:sha256:{digest:x}")),
        }
    }

    pub fn proof_identity(&self) -> &BridgeMergeParentOrderProofIdentity {
        &self.proof_identity
    }

    pub fn parents(&self) -> &[TruthCommitIdentity] {
        &self.parents
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMergeAuthorityBasis {
    basis_identity: BridgeMergeAuthorityBasisIdentity,
    basis_kind: BridgeMergeAuthorityBasisKind,
    artifact_identity: Arc<str>,
    ontology_version: Arc<str>,
    schema_policy_descriptor_version: Arc<str>,
    parent_order_proof: BridgeMergeParentOrderProof,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeMergeAuthorityBasis {
    pub fn new(
        basis_kind: BridgeMergeAuthorityBasisKind,
        artifact_identity: impl Into<Arc<str>>,
        ontology_version: impl Into<Arc<str>>,
        schema_policy_descriptor_version: impl Into<Arc<str>>,
        parent_order_proof: BridgeMergeParentOrderProof,
    ) -> Self {
        let artifact_identity = artifact_identity.into();
        let ontology_version = ontology_version.into();
        let schema_policy_descriptor_version = schema_policy_descriptor_version.into();
        let canonical_basis = Arc::<str>::from(format!(
            "merge-authority-basis|kind:{basis_kind:?}|artifact={}|ontology-version={}|policy-version={}|parent-order={}",
            artifact_identity.as_ref(),
            ontology_version.as_ref(),
            schema_policy_descriptor_version.as_ref(),
            parent_order_proof.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());

        Self {
            basis_identity: BridgeMergeAuthorityBasisIdentity::admit_bridge_owned(format!(
                "merge-authority-basis:sha256:{digest:x}"
            )),
            basis_kind,
            artifact_identity,
            ontology_version,
            schema_policy_descriptor_version,
            parent_order_proof,
            canonical_basis,
            digest: Arc::from(format!("merge-authority-basis:sha256:{digest:x}")),
        }
    }

    pub fn basis_identity(&self) -> &BridgeMergeAuthorityBasisIdentity {
        &self.basis_identity
    }

    pub fn basis_kind(&self) -> BridgeMergeAuthorityBasisKind {
        self.basis_kind
    }

    pub fn artifact_identity(&self) -> &str {
        self.artifact_identity.as_ref()
    }

    pub fn ontology_version(&self) -> &str {
        self.ontology_version.as_ref()
    }

    pub fn schema_policy_descriptor_version(&self) -> &str {
        self.schema_policy_descriptor_version.as_ref()
    }

    pub fn parent_order_proof(&self) -> &BridgeMergeParentOrderProof {
        &self.parent_order_proof
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
