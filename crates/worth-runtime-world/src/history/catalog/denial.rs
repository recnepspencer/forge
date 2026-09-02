use crate::identity::{CompositeCommitIdentity, RuntimeWorldOwnerIdentity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CompositeHistoryCatalogDenial {
    ForeignOwner {
        expected: RuntimeWorldOwnerIdentity,
        actual: RuntimeWorldOwnerIdentity,
    },
    ForeignParent {
        expected: RuntimeWorldOwnerIdentity,
        actual: RuntimeWorldOwnerIdentity,
    },
    DuplicateCommit,
    RootAlreadyInstalled,
    MissingParent(CompositeCommitIdentity),
    CommitCapacityExhausted {
        maximum: usize,
    },
    ArithmeticOverflow,
    MetadataCapacityExhausted {
        maximum: usize,
        used: usize,
        requested: usize,
    },
    DependencyCountOverflow(CompositeCommitIdentity),
    ProtectionCountOverflow(CompositeCommitIdentity),
    UnknownProtectionTarget(CompositeCommitIdentity),
    ReservationMissing,
    ReservationCommitMismatch,
    ReservationParentMismatch,
    ReservationChargeMismatch,
}
