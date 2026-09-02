use crate::identity::{CompositeCommitIdentity, RuntimeWorldOwnerIdentity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CompositeHistoryCatalogDenial {
    ForeignOwner {
        expected: RuntimeWorldOwnerIdentity,
        actual: RuntimeWorldOwnerIdentity,
    },
    DuplicateCommit,
    RootAlreadyInstalled,
    MissingParent(CompositeCommitIdentity),
    CommitCapacityExhausted {
        maximum: usize,
    },
    MetadataCapacityExhausted {
        maximum: usize,
        used: usize,
        requested: usize,
    },
    MetadataSizeOverflow {
        requested: usize,
    },
    ReservationMissing,
    ReservationCommitMismatch,
    ReservationParentMismatch,
    ReservationMetadataTooSmall {
        reserved: usize,
        actual: usize,
    },
}
