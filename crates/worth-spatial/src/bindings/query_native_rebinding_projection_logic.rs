use crate::bindings::authority::SpatialBindingCompleteness;
use crate::bindings::query_native_rebinding_authoring::AuthorPrimitiveRebindingIntent;
use crate::bindings::rebinding::binding_snapshot::{AnchorSnapshot, BindingSnapshot};
use crate::bindings::rebinding::{
    BindingContinuityClass, BindingMotionSemanticsInput, MotionAwareBindingPosture,
    NeighborhoodBindingFamily, PrimitiveRebindingFactReceipt, RebindingOutcomeClass,
    UnsupportedRebindingReason,
};

use super::rebinding::SpatialRebindingAuthorityError;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum CandidateContinuityRank {
    None,
    DeniedIncomplete,
    AdmittedPartial,
    CorrespondenceOnly,
    AuthoritativeSuccessor,
    Exact,
}

pub(crate) fn projection_receipt_from_intent(
    intent: &AuthorPrimitiveRebindingIntent,
) -> Result<PrimitiveRebindingFactReceipt, SpatialRebindingAuthorityError> {
    match intent {
        AuthorPrimitiveRebindingIntent::ReplaceSurfaceBinding {
            prior_binding,
            neighborhood,
            motion,
        } => projection_receipt_for_kind(
            prior_binding.prior_binding_identity(),
            prior_binding.prior_site_identity(),
            prior_binding.family(),
            prior_binding.snapshot(),
            neighborhood,
            *motion,
            NeighborhoodBindingFamily::FaceSurface,
            NeighborhoodBindingFamily::supports_face_surface_rebinding,
        ),
        AuthorPrimitiveRebindingIntent::ReplaceCurveBinding {
            prior_binding,
            neighborhood,
            motion,
        } => projection_receipt_for_kind(
            prior_binding.prior_binding_identity(),
            prior_binding.prior_site_identity(),
            prior_binding.family(),
            prior_binding.snapshot(),
            neighborhood,
            *motion,
            NeighborhoodBindingFamily::EdgeCurve,
            NeighborhoodBindingFamily::supports_edge_curve_rebinding,
        ),
        AuthorPrimitiveRebindingIntent::ReplacePCurveBinding {
            prior_binding,
            neighborhood,
            motion,
        } => projection_receipt_for_kind(
            prior_binding.prior_binding_identity(),
            prior_binding.prior_site_identity(),
            prior_binding.family(),
            prior_binding.snapshot(),
            neighborhood,
            *motion,
            NeighborhoodBindingFamily::CoedgePCurve,
            NeighborhoodBindingFamily::supports_coedge_pcurve_rebinding,
        ),
        AuthorPrimitiveRebindingIntent::ReplaceGeometryBinding {
            prior_binding,
            neighborhood,
            motion,
        } => projection_receipt_for_kind(
            prior_binding.prior_binding_identity(),
            prior_binding.prior_site_identity(),
            prior_binding.family(),
            prior_binding.snapshot(),
            neighborhood,
            *motion,
            NeighborhoodBindingFamily::VertexGeometry,
            NeighborhoodBindingFamily::supports_vertex_geometry_rebinding,
        ),
    }
}

fn projection_receipt_for_kind(
    prior_binding_identity: &str,
    prior_site_identity: &str,
    prior_family: NeighborhoodBindingFamily,
    prior_snapshot: &BindingSnapshot,
    neighborhood: &crate::bindings::rebinding::LocalTopologyReplacementNeighborhood,
    motion: BindingMotionSemanticsInput,
    requested_family: NeighborhoodBindingFamily,
    predicate: fn(NeighborhoodBindingFamily) -> bool,
) -> Result<PrimitiveRebindingFactReceipt, SpatialRebindingAuthorityError> {
    if !predicate(prior_family) {
        return Ok(unsupported_receipt(
            prior_binding_identity,
            prior_site_identity,
            neighborhood,
            UnsupportedRebindingReason::RequestedRebindingFamilyDoesNotAdmitBindingFamily {
                requested: requested_family,
                actual: prior_family,
            },
        ));
    }
    if neighborhood.family() != prior_family {
        return Err(SpatialRebindingAuthorityError::NeighborhoodFamilyMismatch {
            expected: prior_family,
            actual: neighborhood.family(),
        });
    }

    let motion_posture = evaluate_motion_posture(motion);
    let (continuity_class, selected_candidate_identity, selected_candidate_label, ambiguous) =
        continuity_outcome(prior_snapshot, neighborhood);
    let outcome_class = classify_outcome(
        prior_binding_identity,
        continuity_class,
        selected_candidate_identity.as_deref(),
        &motion_posture,
        ambiguous,
    );

    Ok(PrimitiveRebindingFactReceipt::from_projection_parts(
        prior_binding_identity.to_string(),
        prior_site_identity.to_string(),
        selected_candidate_identity,
        selected_candidate_label,
        neighborhood
            .candidates()
            .iter()
            .map(|candidate| candidate.binding_identity().to_string())
            .collect(),
        neighborhood
            .candidates()
            .iter()
            .map(|candidate| candidate.label().to_string())
            .collect(),
        neighborhood
            .candidates()
            .iter()
            .map(|candidate| candidate.site_identity().to_string())
            .collect(),
        continuity_class,
        motion_posture,
        neighborhood.family(),
        outcome_class,
        None,
    ))
}

fn continuity_outcome(
    prior_snapshot: &BindingSnapshot,
    neighborhood: &crate::bindings::rebinding::LocalTopologyReplacementNeighborhood,
) -> (BindingContinuityClass, Option<String>, Option<String>, bool) {
    let mut best_rank = CandidateContinuityRank::None;
    let mut best_candidates = Vec::new();
    for candidate in neighborhood.candidates() {
        let rank = rebinding_rank(prior_snapshot, candidate.snapshot());
        if rank > best_rank {
            best_rank = rank;
            best_candidates.clear();
            best_candidates.push(candidate);
        } else if rank == best_rank {
            best_candidates.push(candidate);
        }
    }

    let ambiguous = best_rank != CandidateContinuityRank::None && best_candidates.len() > 1;
    let continuity_class = continuity_class(best_rank, ambiguous);
    let selected = if ambiguous {
        None
    } else {
        best_candidates.first().copied()
    };
    let selected_candidate_identity = selected
        .filter(|_| {
            matches!(
                continuity_class,
                BindingContinuityClass::Exact
                    | BindingContinuityClass::AuthoritativeSuccessor
                    | BindingContinuityClass::CorrespondenceOnly
            )
        })
        .map(|candidate| candidate.binding_identity().to_string());
    let selected_candidate_label = selected
        .filter(|_| selected_candidate_identity.is_some())
        .map(|candidate| candidate.label().to_string());

    (
        continuity_class,
        selected_candidate_identity,
        selected_candidate_label,
        ambiguous,
    )
}

fn continuity_class(rank: CandidateContinuityRank, ambiguous: bool) -> BindingContinuityClass {
    if ambiguous {
        return BindingContinuityClass::Ambiguous;
    }
    match rank {
        CandidateContinuityRank::Exact => BindingContinuityClass::Exact,
        CandidateContinuityRank::AuthoritativeSuccessor => {
            BindingContinuityClass::AuthoritativeSuccessor
        }
        CandidateContinuityRank::CorrespondenceOnly => BindingContinuityClass::CorrespondenceOnly,
        CandidateContinuityRank::AdmittedPartial => {
            BindingContinuityClass::InsufficientEvidenceFromAdmittedPartial
        }
        CandidateContinuityRank::DeniedIncomplete => {
            BindingContinuityClass::InsufficientEvidenceFromDeniedIncomplete
        }
        CandidateContinuityRank::None => BindingContinuityClass::None,
    }
}

fn rebinding_rank(prior: &BindingSnapshot, candidate: &BindingSnapshot) -> CandidateContinuityRank {
    if prior.family != candidate.family {
        return CandidateContinuityRank::None;
    }
    let completeness_rank = completeness_rank(candidate.completeness);
    if completeness_rank != CandidateContinuityRank::Exact {
        return completeness_rank;
    }
    if prior.geometry_digest == candidate.geometry_digest
        && same_anchor_semantics(prior.anchor.as_ref(), candidate.anchor.as_ref())
    {
        return CandidateContinuityRank::Exact;
    }
    if prior.birth_class == candidate.birth_class
        && prior.anchor.is_none()
        && same_anchor_semantics(prior.anchor.as_ref(), candidate.anchor.as_ref())
    {
        return CandidateContinuityRank::AuthoritativeSuccessor;
    }
    if prior.birth_class == candidate.birth_class {
        return CandidateContinuityRank::CorrespondenceOnly;
    }
    CandidateContinuityRank::AdmittedPartial
}

fn completeness_rank(completeness: SpatialBindingCompleteness) -> CandidateContinuityRank {
    match completeness {
        SpatialBindingCompleteness::Complete => CandidateContinuityRank::Exact,
        SpatialBindingCompleteness::AdmittedPartial(_) => CandidateContinuityRank::AdmittedPartial,
        SpatialBindingCompleteness::DeniedIncomplete(_) => {
            CandidateContinuityRank::DeniedIncomplete
        }
    }
}

fn same_anchor_semantics(
    prior: Option<&AnchorSnapshot>,
    candidate: Option<&AnchorSnapshot>,
) -> bool {
    match (prior, candidate) {
        (None, None) => true,
        (Some(prior), Some(candidate)) => prior.same_semantics(candidate),
        _ => false,
    }
}

fn evaluate_motion_posture(motion: BindingMotionSemanticsInput) -> MotionAwareBindingPosture {
    match motion {
        BindingMotionSemanticsInput::Move => MotionAwareBindingPosture::TransformedWithCarrier,
        BindingMotionSemanticsInput::Rotate { angle_radians } => {
            if angle_radians.abs() <= f64::EPSILON {
                MotionAwareBindingPosture::Preserved
            } else {
                MotionAwareBindingPosture::TransformedWithCarrier
            }
        }
        BindingMotionSemanticsInput::Reorient => MotionAwareBindingPosture::Unresolved,
        BindingMotionSemanticsInput::InvalidatedByLocalTopologyReplacement => {
            MotionAwareBindingPosture::Invalidated
        }
        BindingMotionSemanticsInput::UnresolvedWithoutMotionWorkflow => {
            MotionAwareBindingPosture::Unresolved
        }
    }
}

fn classify_outcome(
    prior_binding_identity: &str,
    continuity_class: BindingContinuityClass,
    selected_candidate_identity: Option<&str>,
    motion_posture: &MotionAwareBindingPosture,
    ambiguous: bool,
) -> RebindingOutcomeClass {
    if ambiguous {
        return RebindingOutcomeClass::Ambiguous;
    }
    match motion_posture {
        MotionAwareBindingPosture::Invalidated => RebindingOutcomeClass::Orphaned,
        MotionAwareBindingPosture::Preserved
        | MotionAwareBindingPosture::TransformedWithCarrier
        | MotionAwareBindingPosture::Unresolved => {
            let preserved_identity = selected_candidate_identity
                .map(|identity| identity == prior_binding_identity)
                .unwrap_or(false);
            if preserved_identity && continuity_class == BindingContinuityClass::Exact {
                return RebindingOutcomeClass::Preserved;
            }
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

fn unsupported_receipt(
    prior_binding_identity: &str,
    prior_site_identity: &str,
    neighborhood: &crate::bindings::rebinding::LocalTopologyReplacementNeighborhood,
    reason: UnsupportedRebindingReason,
) -> PrimitiveRebindingFactReceipt {
    PrimitiveRebindingFactReceipt::from_projection_parts(
        prior_binding_identity.to_string(),
        prior_site_identity.to_string(),
        None,
        None,
        neighborhood
            .candidates()
            .iter()
            .map(|candidate| candidate.binding_identity().to_string())
            .collect(),
        neighborhood
            .candidates()
            .iter()
            .map(|candidate| candidate.label().to_string())
            .collect(),
        neighborhood
            .candidates()
            .iter()
            .map(|candidate| candidate.site_identity().to_string())
            .collect(),
        BindingContinuityClass::None,
        MotionAwareBindingPosture::Unresolved,
        neighborhood.family(),
        RebindingOutcomeClass::Unsupported,
        Some(reason),
    )
}
