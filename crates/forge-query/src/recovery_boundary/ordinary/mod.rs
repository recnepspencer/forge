mod contribution;
mod mapping;

use crate::ordinary_outcome::ForgeQueryOrdinaryOutcome;

use super::brief::ForgeQueryRecoveryBrief;

pub fn forge_query_recovery_brief_from_ordinary_outcome<T>(
    outcome: &ForgeQueryOrdinaryOutcome<T>,
) -> Option<ForgeQueryRecoveryBrief> {
    match outcome {
        ForgeQueryOrdinaryOutcome::Bound(_) => None,
        ForgeQueryOrdinaryOutcome::Ambiguous(posture)
        | ForgeQueryOrdinaryOutcome::AspectConflict(posture)
        | ForgeQueryOrdinaryOutcome::AuthorityMismatch(posture)
        | ForgeQueryOrdinaryOutcome::BasisMismatch(posture)
        | ForgeQueryOrdinaryOutcome::Deferred(posture)
        | ForgeQueryOrdinaryOutcome::Denied(posture)
        | ForgeQueryOrdinaryOutcome::ExplicitNarrowingRequired(posture)
        | ForgeQueryOrdinaryOutcome::Failed(posture)
        | ForgeQueryOrdinaryOutcome::MissingRequiredAspect(posture)
        | ForgeQueryOrdinaryOutcome::RebindRequired(posture)
        | ForgeQueryOrdinaryOutcome::Refused(posture)
        | ForgeQueryOrdinaryOutcome::Stale(posture)
        | ForgeQueryOrdinaryOutcome::Unavailable(posture)
        | ForgeQueryOrdinaryOutcome::Unsupported(posture)
        | ForgeQueryOrdinaryOutcome::WrongHandle(posture)
        | ForgeQueryOrdinaryOutcome::WrongWorld(posture) => {
            Some(mapping::recovery_brief_from_posture(posture))
        }
    }
}
