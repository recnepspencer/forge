use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::identity::{
    BridgeIdentity, BridgeIdentityEvidence, PreviewBranchBindingIdentityTag,
    SpeculativeSignalBranchIdentityTag,
};
use crate::input::envelope::TruthBranchIdentity;

pub type BridgeSignalBranchIdentity = BridgeIdentity<SpeculativeSignalBranchIdentityTag>;
pub type BridgeSpeculativeBranchBindingIdentity = BridgeIdentity<PreviewBranchBindingIdentityTag>;

impl BridgeSignalBranchIdentity {
    pub fn from_bridge_evidence(evidence_identity: &BridgeIdentityEvidence) -> Self {
        Self::new(format!(
            "bridge-preview-signal-branch:external-authority-evidence:{}",
            evidence_identity.as_str()
        ))
    }

    pub fn from_stable_name(value: impl Into<Arc<str>>) -> Self {
        Self::new(value)
    }
}

impl BridgeSpeculativeBranchBindingIdentity {
    pub fn from_bridge_evidence(evidence_identity: &BridgeIdentityEvidence) -> Self {
        Self::new(format!(
            "bridge-preview-branch-binding:external-authority-evidence:{}",
            evidence_identity.as_str()
        ))
    }

    pub fn from_stable_name(value: impl Into<Arc<str>>) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSpeculativeBranchBinding {
    binding_identity: BridgeSpeculativeBranchBindingIdentity,
    truth_branch_identity: TruthBranchIdentity,
    signal_branch_identity: BridgeSignalBranchIdentity,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSpeculativeBranchBinding {
    pub fn new(
        binding_identity: BridgeSpeculativeBranchBindingIdentity,
        truth_branch_identity: TruthBranchIdentity,
        signal_branch_identity: BridgeSignalBranchIdentity,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "speculative-branch-binding|id={}|truth-branch={}|signal-branch={}",
            binding_identity.as_str(),
            truth_branch_identity.as_str(),
            signal_branch_identity.as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            binding_identity,
            truth_branch_identity,
            signal_branch_identity,
            canonical_basis,
            digest: Arc::from(format!("speculative-branch-binding:sha256:{digest:x}")),
        }
    }

    pub fn binding_identity(&self) -> &BridgeSpeculativeBranchBindingIdentity {
        &self.binding_identity
    }

    pub fn truth_branch_identity(&self) -> &TruthBranchIdentity {
        &self.truth_branch_identity
    }

    pub fn signal_branch_identity(&self) -> &BridgeSignalBranchIdentity {
        &self.signal_branch_identity
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
