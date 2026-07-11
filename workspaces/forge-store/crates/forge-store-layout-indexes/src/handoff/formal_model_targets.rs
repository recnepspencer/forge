use super::{S9FormalModelTarget, S9_DOWNSTREAM_PROTOCOL_DESTINATIONS};

/// Future runtime owner expected to contribute the authoritative protocol row
/// in S.9. Layout indexes names the destination; it does not mint that row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum S9ProtocolTargetOwner {
    Wal,
    RecoveryPhysics,
    PhysicalFormat,
    BlobChunks,
    Maintenance,
    PhysicalIsolation,
    BufferPool,
    PhysicalIntegrity,
    Operations,
    Replication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S9DownstreamProtocolTarget {
    target: S9FormalModelTarget,
    expected_owners: &'static [S9ProtocolTargetOwner],
}

impl S9DownstreamProtocolTarget {
    const fn new(
        target: S9FormalModelTarget,
        expected_owners: &'static [S9ProtocolTargetOwner],
    ) -> Self {
        Self {
            target,
            expected_owners,
        }
    }

    pub const fn target(self) -> S9FormalModelTarget {
        self.target
    }

    pub const fn expected_owners(self) -> &'static [S9ProtocolTargetOwner] {
        self.expected_owners
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S9DownstreamProtocolTargetInventory {
    rows: &'static [S9DownstreamProtocolTarget],
}

impl S9DownstreamProtocolTargetInventory {
    pub(crate) const fn canonical() -> Self {
        Self {
            rows: &DOWNSTREAM_PROTOCOL_TARGETS,
        }
    }

    pub const fn rows(self) -> &'static [S9DownstreamProtocolTarget] {
        self.rows
    }

    pub fn contains(self, target: S9FormalModelTarget) -> bool {
        self.rows.iter().any(|row| row.target() == target)
    }

    pub fn declares_all_destinations(self) -> bool {
        self.rows.len() == S9_DOWNSTREAM_PROTOCOL_DESTINATIONS.len()
            && self
                .rows
                .iter()
                .all(|row| !row.expected_owners().is_empty())
            && S9_DOWNSTREAM_PROTOCOL_DESTINATIONS
                .into_iter()
                .all(|target| self.contains(target))
    }
}

const DOWNSTREAM_PROTOCOL_TARGETS: [S9DownstreamProtocolTarget; 6] = [
    S9DownstreamProtocolTarget::new(
        S9FormalModelTarget::WalCheckpointPageFlushOrdering,
        &[
            S9ProtocolTargetOwner::Wal,
            S9ProtocolTargetOwner::RecoveryPhysics,
            S9ProtocolTargetOwner::PhysicalFormat,
        ],
    ),
    S9DownstreamProtocolTarget::new(
        S9FormalModelTarget::RecoverySourcePrecedence,
        &[S9ProtocolTargetOwner::RecoveryPhysics],
    ),
    S9DownstreamProtocolTarget::new(
        S9FormalModelTarget::CompactionCutover,
        &[
            S9ProtocolTargetOwner::Wal,
            S9ProtocolTargetOwner::BlobChunks,
            S9ProtocolTargetOwner::Maintenance,
            S9ProtocolTargetOwner::PhysicalIsolation,
        ],
    ),
    S9DownstreamProtocolTarget::new(
        S9FormalModelTarget::PhysicalLeaseReclaimBarrier,
        &[
            S9ProtocolTargetOwner::BufferPool,
            S9ProtocolTargetOwner::PhysicalIsolation,
        ],
    ),
    S9DownstreamProtocolTarget::new(
        S9FormalModelTarget::RepairQuarantine,
        &[
            S9ProtocolTargetOwner::PhysicalIntegrity,
            S9ProtocolTargetOwner::Operations,
        ],
    ),
    S9DownstreamProtocolTarget::new(
        S9FormalModelTarget::ReplicationImportAdmission,
        &[
            S9ProtocolTargetOwner::Replication,
            S9ProtocolTargetOwner::Operations,
        ],
    ),
];
