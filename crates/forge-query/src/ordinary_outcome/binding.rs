use crate::binding_pipeline::{ForgeQueryBindingChecked, ForgeQueryBindingOutcome};

use super::{
    ForgeQueryOrdinaryBindingCheckedTopologyKind, ForgeQueryOrdinaryCheckedTopology,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
};

pub(crate) fn ordinary_outcome_from_binding_outcome<T>(
    checked: ForgeQueryBindingChecked<T>,
) -> ForgeQueryOrdinaryOutcome<T> {
    let (outcome, _binding_digest, linked_artifacts) = checked.into_parts();
    let topology =
        |kind| ForgeQueryOrdinaryCheckedTopology::binding(kind, linked_artifacts.clone());
    match outcome {
        ForgeQueryBindingOutcome::Bound(value) => ForgeQueryOrdinaryOutcome::Bound(value),
        ForgeQueryBindingOutcome::Ambiguous(value) => {
            ForgeQueryOrdinaryOutcome::Ambiguous(ForgeQueryOrdinaryPosture::new(
                value.reason(),
                ForgeQueryOrdinaryPostureKind::Ambiguous,
                ForgeQueryOrdinaryNextStep::NarrowInput,
                topology(ForgeQueryOrdinaryBindingCheckedTopologyKind::Ambiguous),
            ))
        }
        ForgeQueryBindingOutcome::Unavailable(value) => {
            ForgeQueryOrdinaryOutcome::Unavailable(ForgeQueryOrdinaryPosture::new(
                value.reason(),
                ForgeQueryOrdinaryPostureKind::Unavailable,
                ForgeQueryOrdinaryNextStep::GatherAvailability,
                topology(ForgeQueryOrdinaryBindingCheckedTopologyKind::Unavailable),
            ))
        }
        ForgeQueryBindingOutcome::WrongWorld(value) => {
            ForgeQueryOrdinaryOutcome::WrongWorld(ForgeQueryOrdinaryPosture::new(
                value.reason(),
                ForgeQueryOrdinaryPostureKind::WrongWorld,
                ForgeQueryOrdinaryNextStep::CorrectWorld,
                topology(ForgeQueryOrdinaryBindingCheckedTopologyKind::WrongWorld),
            ))
        }
        ForgeQueryBindingOutcome::WrongHandle(value) => {
            ForgeQueryOrdinaryOutcome::WrongHandle(ForgeQueryOrdinaryPosture::new(
                value.reason(),
                ForgeQueryOrdinaryPostureKind::WrongHandle,
                ForgeQueryOrdinaryNextStep::CorrectHandle,
                topology(ForgeQueryOrdinaryBindingCheckedTopologyKind::WrongHandle),
            ))
        }
        ForgeQueryBindingOutcome::Stale(value) => {
            ForgeQueryOrdinaryOutcome::Stale(ForgeQueryOrdinaryPosture::new(
                value.reason(),
                ForgeQueryOrdinaryPostureKind::Stale,
                ForgeQueryOrdinaryNextStep::RefreshBasis,
                topology(ForgeQueryOrdinaryBindingCheckedTopologyKind::Stale),
            ))
        }
        ForgeQueryBindingOutcome::RebindRequired(value) => {
            ForgeQueryOrdinaryOutcome::RebindRequired(ForgeQueryOrdinaryPosture::new(
                value.reason(),
                ForgeQueryOrdinaryPostureKind::RebindRequired,
                ForgeQueryOrdinaryNextStep::RebindContext,
                topology(ForgeQueryOrdinaryBindingCheckedTopologyKind::RebindRequired),
            ))
        }
        ForgeQueryBindingOutcome::MissingRequiredAspect(value) => {
            ForgeQueryOrdinaryOutcome::MissingRequiredAspect(ForgeQueryOrdinaryPosture::new(
                value.reason(),
                ForgeQueryOrdinaryPostureKind::MissingRequiredAspect,
                ForgeQueryOrdinaryNextStep::NarrowInput,
                topology(ForgeQueryOrdinaryBindingCheckedTopologyKind::MissingRequiredAspect),
            ))
        }
        ForgeQueryBindingOutcome::AspectConflict(value) => {
            ForgeQueryOrdinaryOutcome::AspectConflict(ForgeQueryOrdinaryPosture::new(
                value.reason(),
                ForgeQueryOrdinaryPostureKind::AspectConflict,
                ForgeQueryOrdinaryNextStep::InspectCheckedLane,
                topology(ForgeQueryOrdinaryBindingCheckedTopologyKind::AspectConflict),
            ))
        }
        ForgeQueryBindingOutcome::AuthorityMismatch(value) => {
            ForgeQueryOrdinaryOutcome::AuthorityMismatch(ForgeQueryOrdinaryPosture::new(
                value.reason(),
                ForgeQueryOrdinaryPostureKind::AuthorityMismatch,
                ForgeQueryOrdinaryNextStep::InspectProofLane,
                topology(ForgeQueryOrdinaryBindingCheckedTopologyKind::AuthorityMismatch),
            ))
        }
        ForgeQueryBindingOutcome::BasisMismatch(value) => {
            ForgeQueryOrdinaryOutcome::BasisMismatch(ForgeQueryOrdinaryPosture::new(
                value.reason(),
                ForgeQueryOrdinaryPostureKind::BasisMismatch,
                ForgeQueryOrdinaryNextStep::RefreshBasis,
                topology(ForgeQueryOrdinaryBindingCheckedTopologyKind::BasisMismatch),
            ))
        }
        ForgeQueryBindingOutcome::ExplicitNarrowingRequired(value) => {
            ForgeQueryOrdinaryOutcome::ExplicitNarrowingRequired(ForgeQueryOrdinaryPosture::new(
                value.reason(),
                ForgeQueryOrdinaryPostureKind::ExplicitNarrowingRequired,
                ForgeQueryOrdinaryNextStep::NarrowInput,
                topology(ForgeQueryOrdinaryBindingCheckedTopologyKind::ExplicitNarrowingRequired),
            ))
        }
        ForgeQueryBindingOutcome::Unsupported(value) => {
            ForgeQueryOrdinaryOutcome::Unsupported(ForgeQueryOrdinaryPosture::new(
                value.reason(),
                ForgeQueryOrdinaryPostureKind::Unsupported,
                ForgeQueryOrdinaryNextStep::CheckSupport,
                topology(ForgeQueryOrdinaryBindingCheckedTopologyKind::Unsupported),
            ))
        }
    }
}
