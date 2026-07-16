use sha2::{Digest, Sha256};
use worth_store_authority::StoreCurrentAuthorityIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExactRecoveryFrontier {
    pub(super) checkpoint_durability: u64,
    pub(super) wal_structural: u64,
    pub(super) local_durable_commit: u64,
    pub(super) client_acknowledged: u64,
    pub(super) replication_acknowledged: u64,
    pub(super) authority_identity: StoreCurrentAuthorityIdentity,
    pub(super) source_lineage: [u8; 32],
    pub(super) identity: [u8; 32],
}

impl ExactRecoveryFrontier {
    pub const fn checkpoint_durability(self) -> u64 {
        self.checkpoint_durability
    }
    pub const fn wal_structural(self) -> u64 {
        self.wal_structural
    }
    pub const fn local_durable_commit(self) -> u64 {
        self.local_durable_commit
    }
    pub const fn client_acknowledged(self) -> u64 {
        self.client_acknowledged
    }
    pub const fn replication_acknowledged(self) -> u64 {
        self.replication_acknowledged
    }
    pub const fn authority_identity(self) -> StoreCurrentAuthorityIdentity {
        self.authority_identity
    }
    pub const fn source_lineage(self) -> [u8; 32] {
        self.source_lineage
    }
    pub const fn identity(self) -> [u8; 32] {
        self.identity
    }

    pub fn compare(self, other: Self) -> FrontierPartialOrder {
        if self.authority_identity != other.authority_identity
            || self.source_lineage != other.source_lineage
        {
            return FrontierPartialOrder::IncomparableAuthorityOrLineage;
        }
        let left = self.dimensions();
        let right = other.dimensions();
        let mut less = false;
        let mut greater = false;
        for index in 0..left.len() {
            less |= left[index] < right[index];
            greater |= left[index] > right[index];
        }
        match (less, greater) {
            (false, false) => FrontierPartialOrder::Equal,
            (true, false) => FrontierPartialOrder::Before,
            (false, true) => FrontierPartialOrder::After,
            (true, true) => FrontierPartialOrder::IncomparableDimensions,
        }
    }

    const fn dimensions(self) -> [u64; 5] {
        [
            self.checkpoint_durability,
            self.wal_structural,
            self.local_durable_commit,
            self.client_acknowledged,
            self.replication_acknowledged,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontierPartialOrder {
    Before,
    Equal,
    After,
    IncomparableDimensions,
    IncomparableAuthorityOrLineage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PitrCandidatePosture {
    Available,
    Degraded,
    Unavailable,
    Indeterminate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PitrRoundingPolicy {
    ExactOnly,
    PreviousAcknowledged,
    NextAcknowledged,
    NearestAcknowledged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryTimelineObservation {
    pub(super) observed_time: i64,
    pub(super) uncertainty_before: u64,
    pub(super) uncertainty_after: u64,
    pub(super) frontier: ExactRecoveryFrontier,
    pub(super) source_identity: [u8; 32],
    pub(super) posture: PitrCandidatePosture,
}

#[derive(Debug, Clone, Copy)]
pub struct RecoveryPhysicsTimelineAuthority;

impl RecoveryPhysicsTimelineAuthority {
    #[allow(clippy::too_many_arguments)]
    pub fn admit_observation(
        observed_time: i64,
        uncertainty_before: u64,
        uncertainty_after: u64,
        checkpoint_durability: u64,
        wal_structural: u64,
        local_durable_commit: u64,
        client_acknowledged: u64,
        replication_acknowledged: u64,
        authority_identity: StoreCurrentAuthorityIdentity,
        source_lineage: [u8; 32],
        source_identity: [u8; 32],
        posture: PitrCandidatePosture,
    ) -> Option<RecoveryTimelineObservation> {
        if source_lineage == [0; 32]
            || source_identity == [0; 32]
            || checkpoint_durability > wal_structural
            || wal_structural > local_durable_commit
            || client_acknowledged > local_durable_commit
            || replication_acknowledged > local_durable_commit
        {
            return None;
        }
        let identity = frontier_identity(
            checkpoint_durability,
            wal_structural,
            local_durable_commit,
            client_acknowledged,
            replication_acknowledged,
            authority_identity,
            source_lineage,
        );
        Some(RecoveryTimelineObservation {
            observed_time,
            uncertainty_before,
            uncertainty_after,
            frontier: ExactRecoveryFrontier {
                checkpoint_durability,
                wal_structural,
                local_durable_commit,
                client_acknowledged,
                replication_acknowledged,
                authority_identity,
                source_lineage,
                identity,
            },
            source_identity,
            posture,
        })
    }
}

fn frontier_identity(
    checkpoint: u64,
    wal: u64,
    local: u64,
    client: u64,
    replication: u64,
    authority: StoreCurrentAuthorityIdentity,
    lineage: [u8; 32],
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-exact-recovery-frontier-v1");
    for value in [checkpoint, wal, local, client, replication] {
        digest.update(value.to_be_bytes());
    }
    digest.update(authority.fingerprint());
    digest.update(lineage);
    digest.finalize().into()
}
