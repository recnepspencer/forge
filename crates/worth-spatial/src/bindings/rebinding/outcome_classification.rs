use crate::bindings::admitted_binding::SpatialAdmittedPrimitiveBinding;

use super::{
    candidate_evaluation::ReplacementCandidateEvaluation, continuity::BindingContinuityClass,
    diagnostics::RebindingExplanation, motion_posture::MotionAwareBindingPosture,
    neighborhood::LocalTopologyReplacementNeighborhood,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RebindingOutcomeClass {
    Preserved,
    ExactReattachment,
    ContinuityJustifiedReattachment,
    CorrespondenceOnly,
    Ambiguous,
    Orphaned,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnsupportedRebindingReason {
    RequestedRebindingFamilyDoesNotAdmitBindingFamily {
        requested: super::neighborhood::NeighborhoodBindingFamily,
        actual: super::neighborhood::NeighborhoodBindingFamily,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedRebindingDecision {
    outcome_class: RebindingOutcomeClass,
    selected_binding: Option<SpatialAdmittedPrimitiveBinding>,
    explanation: RebindingExplanation,
}

impl AdmittedRebindingDecision {
    pub(crate) fn new(
        evaluation: &ReplacementCandidateEvaluation,
        motion_posture: MotionAwareBindingPosture,
    ) -> Self {
        let continuity_class = evaluation.continuity().continuity_class();
        let explanation = RebindingExplanation::from_evaluation(evaluation, motion_posture.clone());
        let selected_binding = evaluation
            .selection()
            .selected_candidate_identity()
            .filter(|_| {
                matches!(
                    continuity_class,
                    BindingContinuityClass::Exact
                        | BindingContinuityClass::AuthoritativeSuccessor
                        | BindingContinuityClass::CorrespondenceOnly
                )
            })
            .and_then(|identity| {
                evaluation
                    .neighborhood()
                    .candidates()
                    .iter()
                    .find(|candidate| candidate.binding().identity() == identity)
            })
            .map(|candidate| candidate.binding().clone());
        let preserved_identity = selected_binding
            .as_ref()
            .map(|binding| binding.identity() == evaluation.prior_binding().identity())
            .unwrap_or(false);

        let outcome_class = match motion_posture {
            MotionAwareBindingPosture::Invalidated => RebindingOutcomeClass::Orphaned,
            MotionAwareBindingPosture::Preserved
            | MotionAwareBindingPosture::TransformedWithCarrier
            | MotionAwareBindingPosture::Unresolved => {
                if preserved_identity && continuity_class == BindingContinuityClass::Exact {
                    RebindingOutcomeClass::Preserved
                } else {
                    match continuity_class {
                        BindingContinuityClass::Exact => RebindingOutcomeClass::ExactReattachment,
                        BindingContinuityClass::AuthoritativeSuccessor => {
                            RebindingOutcomeClass::ContinuityJustifiedReattachment
                        }
                        BindingContinuityClass::CorrespondenceOnly => {
                            RebindingOutcomeClass::CorrespondenceOnly
                        }
                        BindingContinuityClass::Ambiguous => RebindingOutcomeClass::Ambiguous,
                        BindingContinuityClass::InsufficientEvidenceFromAdmittedPartial
                        | BindingContinuityClass::InsufficientEvidenceFromDeniedIncomplete
                        | BindingContinuityClass::None => RebindingOutcomeClass::Orphaned,
                    }
                }
            }
        };

        Self {
            outcome_class,
            selected_binding,
            explanation,
        }
    }

    pub(crate) fn unsupported(
        prior_binding: &SpatialAdmittedPrimitiveBinding,
        neighborhood: &LocalTopologyReplacementNeighborhood,
        reason: UnsupportedRebindingReason,
    ) -> Self {
        Self {
            outcome_class: RebindingOutcomeClass::Unsupported,
            selected_binding: None,
            explanation: RebindingExplanation::unsupported(prior_binding, neighborhood, reason),
        }
    }

    pub fn outcome_class(&self) -> RebindingOutcomeClass {
        self.outcome_class
    }

    pub fn selected_binding(&self) -> Option<&SpatialAdmittedPrimitiveBinding> {
        self.selected_binding.as_ref()
    }

    pub fn explanation(&self) -> &RebindingExplanation {
        &self.explanation
    }
}
