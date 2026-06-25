use crate::graph_read_access_plan_adoption::WorthGraphReadAccessResolvedPosture;

use super::{WorthGraphReadAccessUnresolvedSliceKind, WorthGraphReadAccessUnresolvedSliceRow};

pub(crate) fn classify_unresolved_slices(
    postures: &[WorthGraphReadAccessResolvedPosture],
) -> Vec<WorthGraphReadAccessUnresolvedSliceRow> {
    postures
        .iter()
        .map(|posture| {
            let kind = classify_posture(posture);
            WorthGraphReadAccessUnresolvedSliceRow::from_posture(posture, kind)
        })
        .collect()
}

fn classify_posture(
    posture: &WorthGraphReadAccessResolvedPosture,
) -> WorthGraphReadAccessUnresolvedSliceKind {
    if posture.read_family_target() == Some("spatial_planar_boolean_continuation_index") {
        return WorthGraphReadAccessUnresolvedSliceKind::SpatialGraphRead;
    }
    if posture.read_family_target() == Some("broad_boolean_predicate_graph_read") {
        return WorthGraphReadAccessUnresolvedSliceKind::BroadBooleanPredicateRead;
    }
    if posture.source_carried_gap_digest().is_some()
        || posture.posture_family() == "carried_capability_gap"
    {
        return WorthGraphReadAccessUnresolvedSliceKind::CarriedCapabilityGap;
    }
    if posture.denial_kind().is_some() || posture.query_posture() == "denied" {
        return WorthGraphReadAccessUnresolvedSliceKind::DeniedOrRequiredQueryPosture;
    }

    classify_exact_query_posture(posture.query_posture())
}

fn classify_exact_query_posture(query_posture: &str) -> WorthGraphReadAccessUnresolvedSliceKind {
    match query_posture {
        "persistent_index_required" | "paged_streaming_required" | "admitted_paged_streaming" => {
            WorthGraphReadAccessUnresolvedSliceKind::DenseFrontierRead
        }
        "required_support_posture"
        | "async_materialization_required"
        | "store_backed_capability_required"
        | "access_capability_registration_required" => {
            WorthGraphReadAccessUnresolvedSliceKind::DeniedOrRequiredQueryPosture
        }
        "missing_query_read_family_artifact" => {
            WorthGraphReadAccessUnresolvedSliceKind::MissingQueryReadFamilyArtifact
        }
        "inline_indexed" | "bounded_ephemeral_index" | "admitted_plan_candidate" => {
            WorthGraphReadAccessUnresolvedSliceKind::KernelGraphRead
        }
        _ => WorthGraphReadAccessUnresolvedSliceKind::UnknownCoveredGraphRead,
    }
}
