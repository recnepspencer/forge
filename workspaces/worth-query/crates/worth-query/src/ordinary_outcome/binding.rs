use crate::binding_pipeline::{WorthQueryBindingChecked, WorthQueryBindingOutcome};

use super::{
    WorthQueryOrdinaryBindingCheckedTopologyKind, WorthQueryOrdinaryCheckedTopology,
    WorthQueryOrdinaryNextStep, WorthQueryOrdinaryOutcome, WorthQueryOrdinaryPosture,
    WorthQueryOrdinaryPostureKind,
};

#[cfg(test)]
pub(crate) fn ordinary_outcome_from_binding_outcome<T>(
    checked: WorthQueryBindingChecked<T>,
) -> WorthQueryOrdinaryOutcome<T> {
    let (outcome, _binding_digest, linked_artifacts) = checked.into_parts();
    let topology =
        |kind| WorthQueryOrdinaryCheckedTopology::binding(kind, linked_artifacts.clone());
    match outcome {
        WorthQueryBindingOutcome::Bound(value) => WorthQueryOrdinaryOutcome::Bound(value),
        WorthQueryBindingOutcome::Ambiguous(value) => {
            WorthQueryOrdinaryOutcome::Ambiguous(WorthQueryOrdinaryPosture::new(
                value.reason(),
                WorthQueryOrdinaryPostureKind::Ambiguous,
                WorthQueryOrdinaryNextStep::NarrowInput,
                topology(WorthQueryOrdinaryBindingCheckedTopologyKind::Ambiguous),
            ))
        }
        WorthQueryBindingOutcome::Unavailable(value) => {
            WorthQueryOrdinaryOutcome::Unavailable(WorthQueryOrdinaryPosture::new(
                value.reason(),
                WorthQueryOrdinaryPostureKind::Unavailable,
                WorthQueryOrdinaryNextStep::GatherAvailability,
                topology(WorthQueryOrdinaryBindingCheckedTopologyKind::Unavailable),
            ))
        }
        WorthQueryBindingOutcome::WrongWorld(value) => {
            WorthQueryOrdinaryOutcome::WrongWorld(WorthQueryOrdinaryPosture::new(
                value.reason(),
                WorthQueryOrdinaryPostureKind::WrongWorld,
                WorthQueryOrdinaryNextStep::CorrectWorld,
                topology(WorthQueryOrdinaryBindingCheckedTopologyKind::WrongWorld),
            ))
        }
        WorthQueryBindingOutcome::WrongHandle(value) => {
            WorthQueryOrdinaryOutcome::WrongHandle(WorthQueryOrdinaryPosture::new(
                value.reason(),
                WorthQueryOrdinaryPostureKind::WrongHandle,
                WorthQueryOrdinaryNextStep::CorrectHandle,
                topology(WorthQueryOrdinaryBindingCheckedTopologyKind::WrongHandle),
            ))
        }
        WorthQueryBindingOutcome::Stale(value) => {
            WorthQueryOrdinaryOutcome::Stale(WorthQueryOrdinaryPosture::new(
                value.reason(),
                WorthQueryOrdinaryPostureKind::Stale,
                WorthQueryOrdinaryNextStep::RefreshBasis,
                topology(WorthQueryOrdinaryBindingCheckedTopologyKind::Stale),
            ))
        }
        WorthQueryBindingOutcome::RebindRequired(value) => {
            WorthQueryOrdinaryOutcome::RebindRequired(WorthQueryOrdinaryPosture::new(
                value.reason(),
                WorthQueryOrdinaryPostureKind::RebindRequired,
                WorthQueryOrdinaryNextStep::RebindContext,
                topology(WorthQueryOrdinaryBindingCheckedTopologyKind::RebindRequired),
            ))
        }
        WorthQueryBindingOutcome::MissingRequiredAspect(value) => {
            WorthQueryOrdinaryOutcome::MissingRequiredAspect(WorthQueryOrdinaryPosture::new(
                value.reason(),
                WorthQueryOrdinaryPostureKind::MissingRequiredAspect,
                WorthQueryOrdinaryNextStep::NarrowInput,
                topology(WorthQueryOrdinaryBindingCheckedTopologyKind::MissingRequiredAspect),
            ))
        }
        WorthQueryBindingOutcome::AspectConflict(value) => {
            WorthQueryOrdinaryOutcome::AspectConflict(WorthQueryOrdinaryPosture::new(
                value.reason(),
                WorthQueryOrdinaryPostureKind::AspectConflict,
                WorthQueryOrdinaryNextStep::InspectCheckedLane,
                topology(WorthQueryOrdinaryBindingCheckedTopologyKind::AspectConflict),
            ))
        }
        WorthQueryBindingOutcome::AuthorityMismatch(value) => {
            WorthQueryOrdinaryOutcome::AuthorityMismatch(WorthQueryOrdinaryPosture::new(
                value.reason(),
                WorthQueryOrdinaryPostureKind::AuthorityMismatch,
                WorthQueryOrdinaryNextStep::InspectProofLane,
                topology(WorthQueryOrdinaryBindingCheckedTopologyKind::AuthorityMismatch),
            ))
        }
        WorthQueryBindingOutcome::BasisMismatch(value) => {
            WorthQueryOrdinaryOutcome::BasisMismatch(WorthQueryOrdinaryPosture::new(
                value.reason(),
                WorthQueryOrdinaryPostureKind::BasisMismatch,
                WorthQueryOrdinaryNextStep::RefreshBasis,
                topology(WorthQueryOrdinaryBindingCheckedTopologyKind::BasisMismatch),
            ))
        }
        WorthQueryBindingOutcome::ExplicitNarrowingRequired(value) => {
            WorthQueryOrdinaryOutcome::ExplicitNarrowingRequired(WorthQueryOrdinaryPosture::new(
                value.reason(),
                WorthQueryOrdinaryPostureKind::ExplicitNarrowingRequired,
                WorthQueryOrdinaryNextStep::NarrowInput,
                topology(WorthQueryOrdinaryBindingCheckedTopologyKind::ExplicitNarrowingRequired),
            ))
        }
        WorthQueryBindingOutcome::Unsupported(value) => {
            WorthQueryOrdinaryOutcome::Unsupported(WorthQueryOrdinaryPosture::new(
                value.reason(),
                WorthQueryOrdinaryPostureKind::Unsupported,
                WorthQueryOrdinaryNextStep::CheckSupport,
                topology(WorthQueryOrdinaryBindingCheckedTopologyKind::Unsupported),
            ))
        }
    }
}
