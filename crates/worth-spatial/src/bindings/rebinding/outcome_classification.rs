use crate::bindings::authority::SpatialAdmittedPrimitiveBinding;

use super::{
    candidate_evaluation::ReplacementCandidateEvaluation, continuity::BindingContinuityClass,
    diagnostics::RebindingExplanation, motion_posture::MotionAwareBindingPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RebindingOutcomeClass {
    Preserved,
    ExactReattachment,
    ContinuityJustifiedReattachment,
    CorrespondenceOnly,
    Ambiguous,
    Orphaned,
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
            .continuity()
            .candidate_identity()
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
                    .find(|candidate| candidate.binding().identity().as_str() == identity)
            })
            .map(|candidate| candidate.binding().clone());

        let outcome_class = match motion_posture {
            MotionAwareBindingPosture::Preserved => RebindingOutcomeClass::Preserved,
            MotionAwareBindingPosture::Invalidated => RebindingOutcomeClass::Orphaned,
            MotionAwareBindingPosture::RequiresRebinding => match continuity_class {
                BindingContinuityClass::Exact => RebindingOutcomeClass::ExactReattachment,
                BindingContinuityClass::AuthoritativeSuccessor => {
                    RebindingOutcomeClass::ContinuityJustifiedReattachment
                }
                BindingContinuityClass::CorrespondenceOnly => {
                    RebindingOutcomeClass::CorrespondenceOnly
                }
                BindingContinuityClass::Ambiguous => RebindingOutcomeClass::Ambiguous,
                BindingContinuityClass::InsufficientEvidence | BindingContinuityClass::None => {
                    RebindingOutcomeClass::Orphaned
                }
            },
        };

        Self {
            outcome_class,
            selected_binding,
            explanation,
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
