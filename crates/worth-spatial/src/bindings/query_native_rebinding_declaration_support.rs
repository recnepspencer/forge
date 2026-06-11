use forge_query::facade::ForgeQueryDeclarationCanonicalEntry;

use crate::bindings::authority::SpatialBindingKind;
use crate::bindings::query_native_rebinding_authoring::AuthorPrimitiveRebindingIntent;

pub(crate) fn canonical_query_entries_for_intent(
    intent: &AuthorPrimitiveRebindingIntent,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    let mut entries = vec![
        ForgeQueryDeclarationCanonicalEntry::text(
            "rebinding.kind.label",
            intent.rebinding_kind_label(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "rebinding.kind.binding_kind",
            binding_kind_label(intent.prior_binding_fact().binding_kind()),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "rebinding.prior.binding_identity",
            intent.prior_binding_fact().prior_binding_identity(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "rebinding.prior.site_identity",
            intent.prior_binding_fact().prior_site_identity(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "rebinding.neighborhood.family",
            intent.neighborhood().family().rebinding_kind_label(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "rebinding.neighborhood.candidate_count",
            intent.neighborhood().candidates().len().to_string(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "rebinding.neighborhood.motion_input",
            format!("{:?}", intent.motion()),
        ),
    ];
    for (index, candidate) in intent.neighborhood().candidates().iter().enumerate() {
        entries.push(ForgeQueryDeclarationCanonicalEntry::text(
            format!("rebinding.candidates.{index}.label"),
            candidate.label(),
        ));
        entries.push(ForgeQueryDeclarationCanonicalEntry::text(
            format!("rebinding.candidates.{index}.identity"),
            candidate.binding_identity(),
        ));
        entries.push(ForgeQueryDeclarationCanonicalEntry::text(
            format!("rebinding.candidates.{index}.site_identity"),
            candidate.site_identity(),
        ));
        entries.push(ForgeQueryDeclarationCanonicalEntry::text(
            format!("rebinding.candidates.{index}.family"),
            candidate.family().rebinding_kind_label(),
        ));
    }
    entries
}

fn binding_kind_label(binding_kind: SpatialBindingKind) -> &'static str {
    match binding_kind {
        SpatialBindingKind::FaceSurface => "face_surface",
        SpatialBindingKind::EdgeCurve => "edge_curve",
        SpatialBindingKind::CoedgePCurve => "coedge_pcurve",
        SpatialBindingKind::VertexGeometry => "vertex_geometry",
    }
}
