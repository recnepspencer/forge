use super::assembly::ReplayUndoTransactionBoundaryAssemblyError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReplayUndoTransactionBoundaryError {
    Assembly(ReplayUndoTransactionBoundaryAssemblyError),
    HiddenReplayMutationGap {
        claim_scope_digest: String,
        expected_scope_digest: String,
    },
    HiddenUndoMutationGap {
        claim_scope_digest: String,
        expected_scope_digest: String,
    },
}

impl From<ReplayUndoTransactionBoundaryAssemblyError> for ReplayUndoTransactionBoundaryError {
    fn from(value: ReplayUndoTransactionBoundaryAssemblyError) -> Self {
        Self::Assembly(value)
    }
}
