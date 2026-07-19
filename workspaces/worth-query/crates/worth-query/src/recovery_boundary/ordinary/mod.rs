mod contribution;
mod mapping;

use crate::ordinary_outcome::WorthQueryOrdinaryOutcome;

use super::brief::WorthQueryRecoveryBrief;

pub fn worth_query_recovery_brief_from_ordinary_outcome<T>(
    outcome: &WorthQueryOrdinaryOutcome<T>,
) -> Option<WorthQueryRecoveryBrief> {
    match outcome {
        WorthQueryOrdinaryOutcome::Bound(_) => None,
        WorthQueryOrdinaryOutcome::Ambiguous(posture)
        | WorthQueryOrdinaryOutcome::AspectConflict(posture)
        | WorthQueryOrdinaryOutcome::AuthorityMismatch(posture)
        | WorthQueryOrdinaryOutcome::BasisMismatch(posture)
        | WorthQueryOrdinaryOutcome::Deferred(posture)
        | WorthQueryOrdinaryOutcome::Denied(posture)
        | WorthQueryOrdinaryOutcome::ExplicitNarrowingRequired(posture)
        | WorthQueryOrdinaryOutcome::Failed(posture)
        | WorthQueryOrdinaryOutcome::MissingRequiredAspect(posture)
        | WorthQueryOrdinaryOutcome::RebindRequired(posture)
        | WorthQueryOrdinaryOutcome::Refused(posture)
        | WorthQueryOrdinaryOutcome::Stale(posture)
        | WorthQueryOrdinaryOutcome::Unavailable(posture)
        | WorthQueryOrdinaryOutcome::Unsupported(posture)
        | WorthQueryOrdinaryOutcome::WrongHandle(posture)
        | WorthQueryOrdinaryOutcome::WrongWorld(posture) => {
            Some(mapping::recovery_brief_from_posture(posture))
        }
    }
}
