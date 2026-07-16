use std::path::PathBuf;

use sha2::{Digest, Sha256};
use worth_store_authority::{StoreCurrentAuthorityIdentity, StoreCurrentAuthorityWitness};
use worth_store_physical_isolation::{OldReachabilityPreservation, PublicationRootCandidate};
use worth_store_recovery_physics::ExactRecoveryFrontier;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryAuthorityFrontier {
    checkpoint: u64,
    wal: u64,
    local_durable: u64,
    client_acknowledged: u64,
    replication_acknowledged: u64,
    authority: StoreCurrentAuthorityIdentity,
    lineage: [u8; 32],
    identity: [u8; 32],
}

impl RecoveryAuthorityFrontier {
    #[allow(clippy::too_many_arguments)]
    pub fn observed(
        current: &StoreCurrentAuthorityWitness,
        checkpoint: u64,
        wal: u64,
        local_durable: u64,
        client_acknowledged: u64,
        replication_acknowledged: u64,
        lineage: [u8; 32],
    ) -> Option<Self> {
        Self::admit(
            checkpoint,
            wal,
            local_durable,
            client_acknowledged,
            replication_acknowledged,
            current.authority_identity(),
            lineage,
        )
    }

    pub(crate) fn from_exact(frontier: ExactRecoveryFrontier) -> Self {
        Self::admit(
            frontier.checkpoint_durability(),
            frontier.wal_structural(),
            frontier.local_durable_commit(),
            frontier.client_acknowledged(),
            frontier.replication_acknowledged(),
            frontier.authority_identity(),
            frontier.source_lineage(),
        )
        .expect("recovery-physics exact frontier is valid")
    }

    pub(crate) fn from_staged(
        checkpoint: u64,
        wal: u64,
        acknowledged: u64,
        authority: StoreCurrentAuthorityIdentity,
        lineage: [u8; 32],
    ) -> Self {
        Self::admit(
            checkpoint,
            wal,
            acknowledged,
            acknowledged,
            0,
            authority,
            lineage,
        )
        .expect("post-verified staged frontier is valid")
    }

    #[allow(clippy::too_many_arguments)]
    fn admit(
        checkpoint: u64,
        wal: u64,
        local: u64,
        client: u64,
        replication: u64,
        authority: StoreCurrentAuthorityIdentity,
        lineage: [u8; 32],
    ) -> Option<Self> {
        if checkpoint > wal
            || wal > local
            || client > local
            || replication > local
            || lineage == [0; 32]
        {
            return None;
        }
        let mut digest = Sha256::new();
        digest.update(b"worth-store-recovery-authority-frontier-v1");
        for value in [checkpoint, wal, local, client, replication] {
            digest.update(value.to_be_bytes());
        }
        digest.update(authority.fingerprint());
        digest.update(lineage);
        Some(Self {
            checkpoint,
            wal,
            local_durable: local,
            client_acknowledged: client,
            replication_acknowledged: replication,
            authority,
            lineage,
            identity: digest.finalize().into(),
        })
    }
    pub const fn identity(self) -> [u8; 32] {
        self.identity
    }
    pub const fn authority(self) -> StoreCurrentAuthorityIdentity {
        self.authority
    }
    pub const fn lineage(self) -> [u8; 32] {
        self.lineage
    }
}

#[derive(Debug, Clone)]
pub struct CurrentRecoveryAuthoritySnapshot {
    pub(super) publication_directory: PathBuf,
    pub(super) current_root: PublicationRootCandidate,
    pub(super) old_reachability: OldReachabilityPreservation,
    pub(super) frontier: RecoveryAuthorityFrontier,
}

impl CurrentRecoveryAuthoritySnapshot {
    pub fn observe(
        current: &StoreCurrentAuthorityWitness,
        publication_directory: impl Into<PathBuf>,
        current_root: PublicationRootCandidate,
        old_reachability: OldReachabilityPreservation,
        frontier: RecoveryAuthorityFrontier,
    ) -> Result<Self, RecoveryCutoverDenial> {
        if current_root.root().store_authority_identity() != current.authority_identity()
            || frontier.authority() != current.authority_identity()
        {
            return Err(RecoveryCutoverDenial::StaleCurrentAuthority);
        }
        Ok(Self {
            publication_directory: publication_directory.into(),
            current_root,
            old_reachability,
            frontier,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryAuthorityDelta {
    local_durable_loss: u64,
    client_acknowledged_loss: u64,
    replication_acknowledged_loss: u64,
    divergent_lineage: bool,
    authority_changed: bool,
    identity: [u8; 32],
}

impl RecoveryAuthorityDelta {
    pub(crate) fn between(
        current: RecoveryAuthorityFrontier,
        candidate: RecoveryAuthorityFrontier,
    ) -> Self {
        let local_durable_loss = current
            .local_durable
            .saturating_sub(candidate.local_durable);
        let client_acknowledged_loss = current
            .client_acknowledged
            .saturating_sub(candidate.client_acknowledged);
        let replication_acknowledged_loss = current
            .replication_acknowledged
            .saturating_sub(candidate.replication_acknowledged);
        let divergent_lineage = current.lineage != candidate.lineage;
        let authority_changed = current.authority != candidate.authority;
        let mut digest = Sha256::new();
        digest.update(b"worth-store-recovery-authority-delta-v1");
        digest.update(current.identity);
        digest.update(candidate.identity);
        digest.update(local_durable_loss.to_be_bytes());
        digest.update(client_acknowledged_loss.to_be_bytes());
        digest.update(replication_acknowledged_loss.to_be_bytes());
        digest.update([u8::from(divergent_lineage), u8::from(authority_changed)]);
        Self {
            local_durable_loss,
            client_acknowledged_loss,
            replication_acknowledged_loss,
            divergent_lineage,
            authority_changed,
            identity: digest.finalize().into(),
        }
    }
    pub const fn local_durable_loss(self) -> u64 {
        self.local_durable_loss
    }
    pub const fn client_acknowledged_loss(self) -> u64 {
        self.client_acknowledged_loss
    }
    pub const fn replication_acknowledged_loss(self) -> u64 {
        self.replication_acknowledged_loss
    }
    pub const fn divergent_lineage(self) -> bool {
        self.divergent_lineage
    }
    pub const fn authority_changed(self) -> bool {
        self.authority_changed
    }
    pub const fn identity(self) -> [u8; 32] {
        self.identity
    }
}

#[derive(Debug)]
pub enum RecoveryCutoverDenial {
    PostVerification(worth_store_offline_verifier::StagedRecoveryPostVerificationDenial),
    StaleCurrentAuthority,
    PostVerifiedMediaMismatch,
    InvalidAuthorityPosture,
    OwnerVerificationMismatch,
    AuthorityAdmissionPolicy(worth_store_authority::RecoveryAuthorityAdmissionPolicyDenial),
    Authority(worth_store_authority::RecoveryWriteFenceDenial),
    Isolation(worth_store_physical_isolation::RecoveryPublicationDenial),
    OwnerDag(crate::OwnerPlanDagDenial),
    InvalidFootprint,
}
