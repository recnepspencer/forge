use forge_query::facade::ForgeQueryDeclarationCanonicalEntry;
use worth_spatial::facade::bindings::{
    LocalTopologyReplacementNeighborhood, SpatialAdmittedPrimitiveBinding,
};

use crate::binding::rebinding::AuthorPrimitiveRebindingIntent;

pub(super) fn canonical_query_entries_for_intent(
    intent: &AuthorPrimitiveRebindingIntent,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    let mut entries = vec![
        ForgeQueryDeclarationCanonicalEntry::text("rebinding_kind", intent.rebinding_kind_label()),
        ForgeQueryDeclarationCanonicalEntry::text(
            "binding_kind",
            binding_kind_label(intent.prior_binding()),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "prior_identity",
            intent.prior_binding().identity().as_str(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "prior_site_identity",
            intent.prior_binding().topology_site_identity(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "neighborhood_family",
            intent.neighborhood().family().rebinding_kind_label(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "candidate_count",
            intent.neighborhood().candidates().len().to_string(),
        ),
    ];
    extend_with_candidates(&mut entries, intent.neighborhood());
    entries
}

fn extend_with_candidates(
    entries: &mut Vec<ForgeQueryDeclarationCanonicalEntry>,
    neighborhood: &LocalTopologyReplacementNeighborhood,
) {
    for (index, candidate) in neighborhood.candidates().iter().enumerate() {
        entries.push(ForgeQueryDeclarationCanonicalEntry::text(
            format!("candidate.{index}.label"),
            candidate.label(),
        ));
        entries.push(ForgeQueryDeclarationCanonicalEntry::text(
            format!("candidate.{index}.identity"),
            candidate.binding().identity().as_str(),
        ));
        entries.push(ForgeQueryDeclarationCanonicalEntry::text(
            format!("candidate.{index}.site_identity"),
            candidate.site_identity(),
        ));
        entries.push(ForgeQueryDeclarationCanonicalEntry::text(
            format!("candidate.{index}.family"),
            candidate.family().rebinding_kind_label(),
        ));
    }
}

fn binding_kind_label(binding: &SpatialAdmittedPrimitiveBinding) -> &'static str {
    match binding.kind() {
        worth_spatial::facade::bindings::SpatialBindingKind::FaceSurface => "face_surface",
        worth_spatial::facade::bindings::SpatialBindingKind::EdgeCurve => "edge_curve",
        worth_spatial::facade::bindings::SpatialBindingKind::CoedgePCurve => "coedge_pcurve",
        worth_spatial::facade::bindings::SpatialBindingKind::VertexGeometry => "vertex_geometry",
    }
}
