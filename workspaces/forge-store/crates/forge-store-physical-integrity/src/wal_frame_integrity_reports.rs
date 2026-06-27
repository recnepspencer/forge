use crate::{PhysicalScopeBasis, WalFrameIntegrityCounters};
use forge_store_physical_format::{CheckpointAdjacencyPosture, PhysicalReferenceScope};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalTailIntegrityPosture {
    IntactTail,
    TornTail,
    UnsupportedTailIntegrity,
    UnknownTailIntegrity,
    CheckpointAdjacentDamage,
    RecoveryPrecedenceRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalFrameIntegrityInputIdentity {
    scope: PhysicalReferenceScope,
    checkpoint_adjacency: CheckpointAdjacencyPosture,
}

impl WalFrameIntegrityInputIdentity {
    pub(crate) const fn new(
        scope: PhysicalReferenceScope,
        checkpoint_adjacency: CheckpointAdjacencyPosture,
    ) -> Self {
        Self {
            scope,
            checkpoint_adjacency,
        }
    }

    pub const fn scope(self) -> PhysicalReferenceScope {
        self.scope
    }

    pub const fn checkpoint_adjacency(self) -> CheckpointAdjacencyPosture {
        self.checkpoint_adjacency
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalFrameIntegrityReport {
    basis: PhysicalScopeBasis,
    input_identity: WalFrameIntegrityInputIdentity,
    tail_posture: WalTailIntegrityPosture,
    counters: WalFrameIntegrityCounters,
}

impl WalFrameIntegrityReport {
    pub(crate) fn new(
        basis: PhysicalScopeBasis,
        tail_posture: WalTailIntegrityPosture,
        counters: WalFrameIntegrityCounters,
    ) -> Self {
        let input_identity =
            WalFrameIntegrityInputIdentity::new(basis.scope(), basis.checkpoint_adjacency());
        Self {
            basis,
            input_identity,
            tail_posture,
            counters,
        }
    }

    pub const fn basis(&self) -> &PhysicalScopeBasis {
        &self.basis
    }

    pub const fn input_identity(&self) -> WalFrameIntegrityInputIdentity {
        self.input_identity
    }

    pub const fn tail_posture(&self) -> WalTailIntegrityPosture {
        self.tail_posture
    }

    pub const fn counters(&self) -> WalFrameIntegrityCounters {
        self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointRecordIntegrityReport {
    basis: PhysicalScopeBasis,
    input_identity: WalFrameIntegrityInputIdentity,
    tail_posture: WalTailIntegrityPosture,
    counters: WalFrameIntegrityCounters,
}

impl CheckpointRecordIntegrityReport {
    pub(crate) fn new(
        basis: PhysicalScopeBasis,
        tail_posture: WalTailIntegrityPosture,
        counters: WalFrameIntegrityCounters,
    ) -> Self {
        let input_identity =
            WalFrameIntegrityInputIdentity::new(basis.scope(), basis.checkpoint_adjacency());
        Self {
            basis,
            input_identity,
            tail_posture,
            counters,
        }
    }

    pub const fn basis(&self) -> &PhysicalScopeBasis {
        &self.basis
    }

    pub const fn input_identity(&self) -> WalFrameIntegrityInputIdentity {
        self.input_identity
    }

    pub const fn tail_posture(&self) -> WalTailIntegrityPosture {
        self.tail_posture
    }

    pub const fn counters(&self) -> WalFrameIntegrityCounters {
        self.counters
    }
}
