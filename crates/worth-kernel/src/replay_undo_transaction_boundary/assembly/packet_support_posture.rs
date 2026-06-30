use super::super::ReplayUndoTransactionBoundarySupportPosture;

#[derive(Clone, Copy)]
pub enum ReplayUndoTransactionBoundarySupportSource {
    Ordinary,
    QueryGap {
        owner: &'static str,
        blocker: &'static str,
        removal_trigger: &'static str,
    },
}

pub fn lower_replay_undo_transaction_boundary_support_posture(
    source: ReplayUndoTransactionBoundarySupportSource,
) -> ReplayUndoTransactionBoundarySupportPosture {
    match source {
        ReplayUndoTransactionBoundarySupportSource::Ordinary => {
            ReplayUndoTransactionBoundarySupportPosture::Ordinary
        }
        ReplayUndoTransactionBoundarySupportSource::QueryGap {
            owner,
            blocker,
            removal_trigger,
        } => ReplayUndoTransactionBoundarySupportPosture::QueryGap {
            owner,
            blocker,
            removal_trigger,
        },
    }
}
