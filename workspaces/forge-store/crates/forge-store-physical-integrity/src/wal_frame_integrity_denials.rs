use crate::{
    ChecksumAlgorithmMismatchDenial, PhysicalScopeBasis, WalFrameIntegrityCounters,
    WalTailIntegrityPosture,
};
use forge_store_physical_format::{
    CheckpointAdjacencyPosture, PhysicalGenerationOwner, PhysicalReferenceScope,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalFrameDamageDenialKind {
    WrongPhysicalFamily,
    HeaderWitnessMismatch,
    ChecksumFailure,
    TornWalFrame,
    MismatchedLength,
    UnsupportedAlgorithm,
    UnknownTailIntegrity,
    CheckpointAdjacentCorruption,
    WrongCheckpointAdjacency,
    RecoveryPrecedenceRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalFrameDamageDenial {
    kind: WalFrameDamageDenialKind,
    basis: Option<PhysicalScopeBasis>,
    tail_posture: WalTailIntegrityPosture,
    locality: Option<PhysicalGenerationOwner>,
    checkpoint_adjacent_damage: Option<CheckpointAdjacentDamageDenial>,
    checksum_denial: Option<ChecksumAlgorithmMismatchDenial>,
    expected_length: Option<usize>,
    actual_length: Option<usize>,
    counters: WalFrameIntegrityCounters,
}

impl WalFrameDamageDenial {
    pub(crate) fn new(
        kind: WalFrameDamageDenialKind,
        tail_posture: WalTailIntegrityPosture,
        counters: WalFrameIntegrityCounters,
    ) -> Self {
        Self {
            kind,
            basis: None,
            tail_posture,
            locality: None,
            checkpoint_adjacent_damage: None,
            checksum_denial: None,
            expected_length: None,
            actual_length: None,
            counters,
        }
    }

    pub(crate) fn with_basis(mut self, basis: PhysicalScopeBasis) -> Self {
        self.locality = Some(basis.scope().owner());
        self.basis = Some(basis);
        self
    }

    pub(crate) const fn with_checksum_denial(
        mut self,
        denial: ChecksumAlgorithmMismatchDenial,
    ) -> Self {
        self.checksum_denial = Some(denial);
        self
    }

    pub(crate) const fn with_lengths(mut self, expected: usize, actual: usize) -> Self {
        self.expected_length = Some(expected);
        self.actual_length = Some(actual);
        self
    }

    pub(crate) const fn with_checkpoint_adjacent_damage(
        mut self,
        damage: CheckpointAdjacentDamageDenial,
    ) -> Self {
        self.checkpoint_adjacent_damage = Some(damage);
        self
    }

    pub const fn kind(&self) -> WalFrameDamageDenialKind {
        self.kind
    }

    pub const fn basis(&self) -> Option<&PhysicalScopeBasis> {
        self.basis.as_ref()
    }

    pub const fn tail_posture(&self) -> WalTailIntegrityPosture {
        self.tail_posture
    }

    pub const fn locality(&self) -> Option<PhysicalGenerationOwner> {
        self.locality
    }

    pub const fn checkpoint_adjacent_damage(&self) -> Option<CheckpointAdjacentDamageDenial> {
        self.checkpoint_adjacent_damage
    }

    pub const fn checksum_denial(&self) -> Option<ChecksumAlgorithmMismatchDenial> {
        self.checksum_denial
    }

    pub const fn expected_length(&self) -> Option<usize> {
        self.expected_length
    }

    pub const fn actual_length(&self) -> Option<usize> {
        self.actual_length
    }

    pub const fn counters(&self) -> WalFrameIntegrityCounters {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckpointAdjacentDamageDenial {
    scope: PhysicalReferenceScope,
    posture: CheckpointAdjacencyPosture,
}

impl CheckpointAdjacentDamageDenial {
    pub(crate) const fn new(
        scope: PhysicalReferenceScope,
        posture: CheckpointAdjacencyPosture,
    ) -> Self {
        Self { scope, posture }
    }

    pub const fn scope(self) -> PhysicalReferenceScope {
        self.scope
    }

    pub const fn posture(self) -> CheckpointAdjacencyPosture {
        self.posture
    }
}
