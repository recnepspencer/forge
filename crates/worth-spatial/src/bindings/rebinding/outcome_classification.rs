use crate::bindings::authority::SpatialBindingKind;
#[cfg(test)]
use crate::bindings::query_native_rebinding_prior_fact::PrimitiveRebindingPriorBindingFact;

#[cfg(test)]
use super::{
    candidate_evaluation::ReplacementCandidateEvaluation, diagnostics::RebindingExplanation,
    neighborhood::LocalTopologyReplacementNeighborhood,
};
use super::{continuity::BindingContinuityClass, motion_posture::MotionAwareBindingPosture};

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveRebindingFactReceipt {
    prior_binding_identity: String,
    prior_site_identity: String,
    selected_candidate_identity: Option<String>,
    selected_candidate_label: Option<String>,
    candidate_identities: Vec<String>,
    candidate_labels: Vec<String>,
    candidate_site_identities: Vec<String>,
    continuity_class: BindingContinuityClass,
    motion_posture: MotionAwareBindingPosture,
    neighborhood_family: super::neighborhood::NeighborhoodBindingFamily,
    outcome_class: RebindingOutcomeClass,
    unsupported_reason: Option<UnsupportedRebindingReason>,
}

impl PrimitiveRebindingFactReceipt {
    pub(crate) fn from_projection_parts(
        prior_binding_identity: String,
        prior_site_identity: String,
        selected_candidate_identity: Option<String>,
        selected_candidate_label: Option<String>,
        candidate_identities: Vec<String>,
        candidate_labels: Vec<String>,
        candidate_site_identities: Vec<String>,
        continuity_class: BindingContinuityClass,
        motion_posture: MotionAwareBindingPosture,
        neighborhood_family: super::neighborhood::NeighborhoodBindingFamily,
        outcome_class: RebindingOutcomeClass,
        unsupported_reason: Option<UnsupportedRebindingReason>,
    ) -> Self {
        Self {
            prior_binding_identity,
            prior_site_identity,
            selected_candidate_identity,
            selected_candidate_label,
            candidate_identities,
            candidate_labels,
            candidate_site_identities,
            continuity_class,
            motion_posture,
            neighborhood_family,
            outcome_class,
            unsupported_reason,
        }
    }

    pub fn prior_binding_identity(&self) -> &str {
        &self.prior_binding_identity
    }

    pub fn prior_site_identity(&self) -> &str {
        &self.prior_site_identity
    }

    pub fn selected_candidate_identity(&self) -> Option<&str> {
        self.selected_candidate_identity.as_deref()
    }

    pub fn selected_candidate_label(&self) -> Option<&str> {
        self.selected_candidate_label.as_deref()
    }

    pub fn candidate_identities(&self) -> &[String] {
        &self.candidate_identities
    }

    pub fn candidate_labels(&self) -> &[String] {
        &self.candidate_labels
    }

    pub fn candidate_site_identities(&self) -> &[String] {
        &self.candidate_site_identities
    }

    pub fn continuity_class(&self) -> BindingContinuityClass {
        self.continuity_class
    }

    pub fn motion_posture(&self) -> MotionAwareBindingPosture {
        self.motion_posture.clone()
    }

    pub fn neighborhood_family(&self) -> super::neighborhood::NeighborhoodBindingFamily {
        self.neighborhood_family
    }

    pub fn outcome_class(&self) -> RebindingOutcomeClass {
        self.outcome_class
    }

    pub fn unsupported_reason(&self) -> Option<UnsupportedRebindingReason> {
        self.unsupported_reason
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveRebindingRetainedFactSource {
    binding_kind: SpatialBindingKind,
    receipt: PrimitiveRebindingFactReceipt,
}

impl PrimitiveRebindingRetainedFactSource {
    pub fn new(receipt: PrimitiveRebindingFactReceipt) -> Self {
        Self {
            binding_kind: binding_kind_from_family(receipt.neighborhood_family()),
            receipt,
        }
    }

    pub fn binding_kind(&self) -> SpatialBindingKind {
        self.binding_kind
    }

    pub fn receipt(&self) -> &PrimitiveRebindingFactReceipt {
        &self.receipt
    }
}

#[cfg(test)]
pub(crate) fn rebinding_fact_receipt_from_evaluation(
    evaluation: &ReplacementCandidateEvaluation,
    motion_posture: MotionAwareBindingPosture,
) -> PrimitiveRebindingFactReceipt {
    let outcome_class = classify_rebinding_outcome(evaluation, &motion_posture);
    let explanation = RebindingExplanation::from_evaluation(evaluation, motion_posture);
    fact_receipt_from_explanation(&explanation, outcome_class)
}

#[cfg(test)]
pub(crate) fn unsupported_rebinding_fact_receipt(
    prior_binding: &PrimitiveRebindingPriorBindingFact,
    neighborhood: &LocalTopologyReplacementNeighborhood,
    reason: UnsupportedRebindingReason,
) -> PrimitiveRebindingFactReceipt {
    let explanation = RebindingExplanation::unsupported(prior_binding, neighborhood, reason);
    fact_receipt_from_explanation(&explanation, RebindingOutcomeClass::Unsupported)
}

impl
    From<
        crate::bindings::query_native_rebinding_projection::PrimitiveRebindingProjectionFactReceipt,
    > for PrimitiveRebindingRetainedFactSource
{
    fn from(
        facts: crate::bindings::query_native_rebinding_projection::PrimitiveRebindingProjectionFactReceipt,
    ) -> Self {
        Self {
            binding_kind: facts.binding_kind(),
            receipt: PrimitiveRebindingFactReceipt {
                prior_binding_identity: facts.prior_binding_identity().to_string(),
                prior_site_identity: facts.prior_site_identity().to_string(),
                selected_candidate_identity: facts
                    .selected_candidate_identity()
                    .map(ToOwned::to_owned),
                selected_candidate_label: facts.selected_candidate_label().map(ToOwned::to_owned),
                candidate_identities: facts.candidate_identities().to_vec(),
                candidate_labels: facts.candidate_labels().to_vec(),
                candidate_site_identities: facts.candidate_site_identities().to_vec(),
                continuity_class: facts.continuity_class(),
                motion_posture: facts.motion_posture(),
                neighborhood_family: facts.neighborhood_family(),
                outcome_class: facts.outcome_class(),
                unsupported_reason: facts.unsupported_reason(),
            },
        }
    }
}

#[cfg(test)]
fn classify_rebinding_outcome(
    evaluation: &ReplacementCandidateEvaluation,
    motion_posture: &MotionAwareBindingPosture,
) -> RebindingOutcomeClass {
    let continuity_class = evaluation.continuity().continuity_class();
    let selected_candidate_identity = evaluation.selection().selected_candidate_identity();
    let preserved_identity = selected_candidate_identity
        .map(|identity| identity == evaluation.prior_binding().prior_binding_identity())
        .unwrap_or(false);

    match motion_posture {
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
    }
}

#[cfg(test)]
fn fact_receipt_from_explanation(
    explanation: &RebindingExplanation,
    outcome_class: RebindingOutcomeClass,
) -> PrimitiveRebindingFactReceipt {
    PrimitiveRebindingFactReceipt::from_projection_parts(
        explanation.prior_identity().to_string(),
        explanation.prior_site_identity().to_string(),
        explanation
            .selected_candidate_identity()
            .map(ToString::to_string),
        explanation
            .selected_candidate_label()
            .map(ToString::to_string),
        explanation.candidate_identities().to_vec(),
        explanation.candidate_labels().to_vec(),
        explanation.candidate_site_identities().to_vec(),
        explanation.continuity_class().clone(),
        explanation.motion_posture().clone(),
        explanation.neighborhood_family(),
        outcome_class,
        explanation.unsupported_reason(),
    )
}

fn binding_kind_from_family(
    family: super::neighborhood::NeighborhoodBindingFamily,
) -> SpatialBindingKind {
    match family {
        super::neighborhood::NeighborhoodBindingFamily::FaceSurface
        | super::neighborhood::NeighborhoodBindingFamily::FaceSurfacePointAnchor
        | super::neighborhood::NeighborhoodBindingFamily::FaceSurfaceDirectionAnchor => {
            SpatialBindingKind::FaceSurface
        }
        super::neighborhood::NeighborhoodBindingFamily::EdgeCurve
        | super::neighborhood::NeighborhoodBindingFamily::EdgeCurvePointAnchor
        | super::neighborhood::NeighborhoodBindingFamily::EdgeCurveDirectionAnchor => {
            SpatialBindingKind::EdgeCurve
        }
        super::neighborhood::NeighborhoodBindingFamily::CoedgePCurve
        | super::neighborhood::NeighborhoodBindingFamily::CoedgePCurvePointAnchor
        | super::neighborhood::NeighborhoodBindingFamily::CoedgePCurveDirectionAnchor => {
            SpatialBindingKind::CoedgePCurve
        }
        super::neighborhood::NeighborhoodBindingFamily::VertexGeometry => {
            SpatialBindingKind::VertexGeometry
        }
    }
}
