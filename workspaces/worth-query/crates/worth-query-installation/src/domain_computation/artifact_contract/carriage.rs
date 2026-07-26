#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactClonePosture {
    Forbidden,
    Declared {
        mechanism: WorthQueryArtifactCloneMechanism,
        boundary: WorthQueryArtifactCloneBoundary,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactCloneMechanism {
    DeepClone,
    ProviderDefinedCopy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactCloneBoundary {
    ConcurrentObserver,
    Isolation,
    Retry,
    Temporal,
    ProviderTransfer,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactMovePosture {
    Required,
    Forbidden,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactBorrowPosture {
    Forbidden,
    SharedReadOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactProviderTransferPosture {
    Forbidden,
    MoveOwnership,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactSerializationPosture {
    Forbidden,
    CanonicalProjectionOnly,
    DomainPayloadWithSchema,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactCarriageContract {
    movement: WorthQueryArtifactMovePosture,
    borrowing: WorthQueryArtifactBorrowPosture,
    clone: WorthQueryArtifactClonePosture,
    provider_transfer: WorthQueryArtifactProviderTransferPosture,
    serialization: WorthQueryArtifactSerializationPosture,
}

impl WorthQueryArtifactCarriageContract {
    pub const fn new(
        movement: WorthQueryArtifactMovePosture,
        borrowing: WorthQueryArtifactBorrowPosture,
        clone: WorthQueryArtifactClonePosture,
        provider_transfer: WorthQueryArtifactProviderTransferPosture,
        serialization: WorthQueryArtifactSerializationPosture,
    ) -> Self {
        Self {
            movement,
            borrowing,
            clone,
            provider_transfer,
            serialization,
        }
    }

    pub const fn move_only_provider_transfer() -> Self {
        Self {
            movement: WorthQueryArtifactMovePosture::Required,
            borrowing: WorthQueryArtifactBorrowPosture::Forbidden,
            clone: WorthQueryArtifactClonePosture::Forbidden,
            provider_transfer: WorthQueryArtifactProviderTransferPosture::MoveOwnership,
            serialization: WorthQueryArtifactSerializationPosture::CanonicalProjectionOnly,
        }
    }

    pub const fn movement(self) -> WorthQueryArtifactMovePosture {
        self.movement
    }

    pub const fn borrowing(self) -> WorthQueryArtifactBorrowPosture {
        self.borrowing
    }

    pub const fn clone_posture(self) -> WorthQueryArtifactClonePosture {
        self.clone
    }

    pub const fn provider_transfer(self) -> WorthQueryArtifactProviderTransferPosture {
        self.provider_transfer
    }

    pub const fn serialization(self) -> WorthQueryArtifactSerializationPosture {
        self.serialization
    }

    pub(crate) const fn is_coherent(self) -> bool {
        let can_cross_stage = matches!(self.movement, WorthQueryArtifactMovePosture::Required)
            || matches!(
                self.borrowing,
                WorthQueryArtifactBorrowPosture::SharedReadOnly
            )
            || matches!(self.clone, WorthQueryArtifactClonePosture::Declared { .. })
            || !matches!(
                self.serialization,
                WorthQueryArtifactSerializationPosture::Forbidden
            );
        let transfer_is_movable =
            !matches!(
                self.provider_transfer,
                WorthQueryArtifactProviderTransferPosture::MoveOwnership
            ) || matches!(self.movement, WorthQueryArtifactMovePosture::Required);
        let provider_transfer_clone_has_transfer = !matches!(
            self.clone,
            WorthQueryArtifactClonePosture::Declared {
                boundary: WorthQueryArtifactCloneBoundary::ProviderTransfer,
                ..
            }
        ) || matches!(
            self.provider_transfer,
            WorthQueryArtifactProviderTransferPosture::MoveOwnership
        );
        can_cross_stage && transfer_is_movable && provider_transfer_clone_has_transfer
    }
}
