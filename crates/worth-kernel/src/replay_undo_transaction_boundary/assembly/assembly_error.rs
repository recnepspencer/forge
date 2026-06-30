#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReplayUndoTransactionBoundaryAssemblyError {
    ReplayUndoTouchedSubjectMismatch,
    ReplayUndoEvidenceLookupPriorProofMismatch {
        replay_prior_proof_digest: String,
        undo_prior_proof_digest: String,
    },
    ReplayUndoStageIndexMismatch {
        replay_stage_index_digest: String,
        undo_stage_index_digest: String,
    },
}
