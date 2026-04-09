use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::clone_budget::CheapClone;
use crate::input::envelope::TruthBranchIdentity;
use crate::snapshot::TruthSnapshotIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeContinuityAuthorityKind {
    TruthLineageAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeContinuityDigestBasisKind {
    PriorSubscriptionSlice,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeContinuityAuthorityBasis {
    authority_kind: BridgeContinuityAuthorityKind,
    digest_basis_kind: BridgeContinuityDigestBasisKind,
    branch_identity: TruthBranchIdentity,
    snapshot_identity: TruthSnapshotIdentity,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeContinuityAuthorityBasis {
    pub fn new(
        branch_identity: TruthBranchIdentity,
        snapshot_identity: TruthSnapshotIdentity,
    ) -> Self {
        let authority_kind = BridgeContinuityAuthorityKind::TruthLineageAuthority;
        let digest_basis_kind = BridgeContinuityDigestBasisKind::PriorSubscriptionSlice;
        let canonical_basis = Arc::<str>::from(format!(
            "continuity-authority|authority:{authority_kind:?}|digest_basis:{digest_basis_kind:?}|branch:{}|snapshot:{}",
            branch_identity.as_str(),
            snapshot_identity.as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            authority_kind,
            digest_basis_kind,
            branch_identity,
            snapshot_identity,
            canonical_basis,
            digest: Arc::from(format!("continuity-authority:sha256:{digest:x}")),
        }
    }

    pub fn authority_kind(&self) -> BridgeContinuityAuthorityKind {
        self.authority_kind
    }

    pub fn digest_basis_kind(&self) -> BridgeContinuityDigestBasisKind {
        self.digest_basis_kind
    }

    pub fn branch_identity(&self) -> &TruthBranchIdentity {
        &self.branch_identity
    }

    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeLineageContext {
    authority_basis: BridgeContinuityAuthorityBasis,
}

impl BridgeLineageContext {
    pub fn new(authority_basis: BridgeContinuityAuthorityBasis) -> Self {
        Self { authority_basis }
    }

    pub fn for_snapshot_branch(
        branch_identity: TruthBranchIdentity,
        snapshot_identity: TruthSnapshotIdentity,
    ) -> Self {
        Self::new(BridgeContinuityAuthorityBasis::new(
            branch_identity,
            snapshot_identity,
        ))
    }

    pub fn authority_basis(&self) -> &BridgeContinuityAuthorityBasis {
        &self.authority_basis
    }
}

impl CheapClone for BridgeLineageContext {}

#[cfg(test)]
mod tests {
    use super::{
        BridgeContinuityAuthorityBasis, BridgeContinuityAuthorityKind,
        BridgeContinuityDigestBasisKind, BridgeLineageContext,
    };
    use crate::input::envelope::TruthBranchIdentity;
    use crate::snapshot::TruthSnapshotIdentity;

    #[test]
    fn continuity_authority_basis_is_canonical_for_same_branch_and_snapshot() {
        let left = BridgeContinuityAuthorityBasis::new(
            TruthBranchIdentity::new("main"),
            TruthSnapshotIdentity::new("snapshot-a"),
        );
        let right = BridgeContinuityAuthorityBasis::new(
            TruthBranchIdentity::new("main"),
            TruthSnapshotIdentity::new("snapshot-a"),
        );

        assert_eq!(left, right);
        assert_eq!(
            left.authority_kind(),
            BridgeContinuityAuthorityKind::TruthLineageAuthority
        );
        assert_eq!(
            left.digest_basis_kind(),
            BridgeContinuityDigestBasisKind::PriorSubscriptionSlice
        );
    }

    #[test]
    fn lineage_context_preserves_authority_basis() {
        let context = BridgeLineageContext::for_snapshot_branch(
            TruthBranchIdentity::new("analysis"),
            TruthSnapshotIdentity::new("snapshot-b"),
        );

        assert_eq!(
            context.authority_basis().branch_identity().as_str(),
            "analysis"
        );
        assert_eq!(
            context.authority_basis().snapshot_identity().as_str(),
            "snapshot-b"
        );
        assert!(context
            .authority_basis()
            .canonical_basis()
            .contains("authority:TruthLineageAuthority"));
    }
}
