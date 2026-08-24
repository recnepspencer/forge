//! Branch-qualified transaction preparation and publication surface.

pub use crate::mvcc::{
    BranchBoundRelationalTransaction, RelationalBranchObservation,
    RelationalBranchTransactionAdmissionDenial, RelationalMutationInvariantEvidence,
    RelationalTransactionEntityRead, RelationalTransactionFootprint, RelationalTransactionIntent,
    RelationalTransactionReadLocus, RelationalTransactionRelationRead,
    RelationalTransactionRelationValue, RelationalTransactionWriteLocus,
    ValidatedMutationFootprint, ValidatedMutationFootprintNotRequested,
    ValidatedMutationFootprintProjection, ValidatedMutationFootprintWork, ValidatedMutationTouch,
    ValidatedMutationTouchProjectionError, ValidatedMutationTouchProjectionWork,
    ValidatedMutationTouches, ValidatedRelationalProposal,
};
pub use crate::transactions::data::{
    CommitConflict, CommitResult, ConflictClass, TransactionCommitError, WorkerIntentBatch,
};
