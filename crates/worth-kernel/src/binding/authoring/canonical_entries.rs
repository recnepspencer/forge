use forge_query::facade::ForgeQueryDeclarationCanonicalEntry;
use worth_spatial::facade::bindings::{
    AnchorDirectionRole, CarrierOwnedParameterDirectionAnchorSpec,
    CarrierOwnedParameterPointAnchorSpec, FaceSurfaceBindingSpec,
};

use crate::binding::authoring::AuthorPrimitiveBindingIntent;

pub(super) fn canonical_query_entries_for_intent(
    intent: &AuthorPrimitiveBindingIntent,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    match intent {
        AuthorPrimitiveBindingIntent::AttachSurfaceToFace(spec) => {
            canonical_face_entries("face_surface", spec)
        }
        AuthorPrimitiveBindingIntent::AttachCurveToEdge(spec) => canonical_edge_entries(spec),
        AuthorPrimitiveBindingIntent::AttachPCurveToCoedge(spec) => canonical_coedge_entries(spec),
        AuthorPrimitiveBindingIntent::AttachVertexGeometry(spec) => canonical_vertex_entries(spec),
        AuthorPrimitiveBindingIntent::AttachParameterSpacePointToFace(spec, anchor_spec) => {
            extend_with_point_anchor(canonical_face_entries("face_surface", spec), anchor_spec)
        }
        AuthorPrimitiveBindingIntent::AttachParameterSpacePointToEdge(spec, anchor_spec) => {
            extend_with_point_anchor(canonical_edge_entries(spec), anchor_spec)
        }
        AuthorPrimitiveBindingIntent::AttachParameterSpacePointToCoedge(spec, anchor_spec) => {
            extend_with_point_anchor(canonical_coedge_entries(spec), anchor_spec)
        }
        AuthorPrimitiveBindingIntent::AttachParameterSpaceDirectionToFace(spec, anchor_spec) => {
            extend_with_direction_anchor(canonical_face_entries("face_surface", spec), anchor_spec)
        }
        AuthorPrimitiveBindingIntent::AttachParameterSpaceDirectionToEdge(spec, anchor_spec) => {
            extend_with_direction_anchor(canonical_edge_entries(spec), anchor_spec)
        }
        AuthorPrimitiveBindingIntent::AttachParameterSpaceDirectionToCoedge(spec, anchor_spec) => {
            extend_with_direction_anchor(canonical_coedge_entries(spec), anchor_spec)
        }
    }
}

fn canonical_face_entries(
    binding_kind: &'static str,
    spec: &FaceSurfaceBindingSpec,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    vec![
        ForgeQueryDeclarationCanonicalEntry::text("binding_kind", binding_kind),
        ForgeQueryDeclarationCanonicalEntry::text(
            "site_identity",
            spec.site().topology_face_identity(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "topology_birth_class",
            spec.birth_contract().topology_birth_class(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "geometry_digest",
            spec.geometry_identity().scaffold_geometry_digest().as_str(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "support_plane_count",
            spec.birth_contract()
                .support_contract()
                .support_plane_count()
                .to_string(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "face_count",
            spec.birth_contract()
                .topology_contract()
                .face_count()
                .to_string(),
        ),
    ]
}

fn canonical_edge_entries(
    spec: &worth_spatial::facade::bindings::EdgeCurveBindingSpec,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    vec![
        ForgeQueryDeclarationCanonicalEntry::text("binding_kind", "edge_curve"),
        ForgeQueryDeclarationCanonicalEntry::text(
            "site_identity",
            spec.site().topology_edge_identity(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "topology_birth_class",
            spec.birth_contract().topology_birth_class(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "geometry_digest",
            spec.geometry_identity().scaffold_geometry_digest().as_str(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "edge_count",
            spec.birth_contract()
                .topology_contract()
                .edge_count()
                .to_string(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "vertex_count",
            spec.birth_contract()
                .topology_contract()
                .vertex_count()
                .to_string(),
        ),
    ]
}

fn canonical_coedge_entries(
    spec: &worth_spatial::facade::bindings::CoedgePCurveBindingSpec,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    vec![
        ForgeQueryDeclarationCanonicalEntry::text("binding_kind", "coedge_pcurve"),
        ForgeQueryDeclarationCanonicalEntry::text(
            "site_identity",
            spec.site().topology_coedge_identity(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "topology_birth_class",
            spec.birth_contract().topology_birth_class(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "geometry_digest",
            spec.geometry_identity().scaffold_geometry_digest().as_str(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "loop_count",
            spec.birth_contract()
                .topology_contract()
                .loop_count()
                .to_string(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "support_plane_count",
            spec.birth_contract()
                .support_contract()
                .support_plane_count()
                .to_string(),
        ),
    ]
}

fn canonical_vertex_entries(
    spec: &worth_spatial::facade::bindings::VertexGeometryBindingSpec,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    vec![
        ForgeQueryDeclarationCanonicalEntry::text("binding_kind", "vertex_geometry"),
        ForgeQueryDeclarationCanonicalEntry::text(
            "site_identity",
            spec.site().topology_vertex_identity(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "topology_birth_class",
            spec.birth_contract().topology_birth_class(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "geometry_digest",
            spec.geometry_identity().scaffold_geometry_digest().as_str(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text(
            "vertex_count",
            spec.birth_contract()
                .topology_contract()
                .vertex_count()
                .to_string(),
        ),
        ForgeQueryDeclarationCanonicalEntry::text("provenance_kind", spec.provenance().as_str()),
        ForgeQueryDeclarationCanonicalEntry::text(
            "tolerance_regime",
            spec.tolerance_regime().as_str(),
        ),
    ]
}

fn extend_with_point_anchor(
    mut entries: Vec<ForgeQueryDeclarationCanonicalEntry>,
    anchor_spec: &CarrierOwnedParameterPointAnchorSpec,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    entries.push(ForgeQueryDeclarationCanonicalEntry::text(
        "anchor_kind",
        "parameter_space_point",
    ));
    entries.push(ForgeQueryDeclarationCanonicalEntry::text(
        "anchor_carrier_kind",
        anchor_spec.ownership().carrier_kind().as_str(),
    ));
    entries.push(ForgeQueryDeclarationCanonicalEntry::text(
        "anchor_carrier_identity",
        anchor_spec.ownership().carrier_identity(),
    ));
    entries.push(ForgeQueryDeclarationCanonicalEntry::text(
        "anchor_parameter_u_bits",
        format!("{:016x}", anchor_spec.parameter().u().to_bits()),
    ));
    entries.push(ForgeQueryDeclarationCanonicalEntry::text(
        "anchor_parameter_v_bits",
        format!("{:016x}", anchor_spec.parameter().v().to_bits()),
    ));
    entries
}

fn extend_with_direction_anchor(
    mut entries: Vec<ForgeQueryDeclarationCanonicalEntry>,
    anchor_spec: &CarrierOwnedParameterDirectionAnchorSpec,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    entries.push(ForgeQueryDeclarationCanonicalEntry::text(
        "anchor_kind",
        "parameter_space_direction",
    ));
    entries.push(ForgeQueryDeclarationCanonicalEntry::text(
        "anchor_carrier_kind",
        anchor_spec.ownership().carrier_kind().as_str(),
    ));
    entries.push(ForgeQueryDeclarationCanonicalEntry::text(
        "anchor_carrier_identity",
        anchor_spec.ownership().carrier_identity(),
    ));
    entries.push(ForgeQueryDeclarationCanonicalEntry::text(
        "anchor_parameter_u_bits",
        format!("{:016x}", anchor_spec.parameter().u().to_bits()),
    ));
    entries.push(ForgeQueryDeclarationCanonicalEntry::text(
        "anchor_parameter_v_bits",
        format!("{:016x}", anchor_spec.parameter().v().to_bits()),
    ));
    entries.push(ForgeQueryDeclarationCanonicalEntry::text(
        "anchor_direction_role",
        direction_role_as_str(anchor_spec.role()),
    ));
    entries
}

fn direction_role_as_str(role: AnchorDirectionRole) -> &'static str {
    match role {
        AnchorDirectionRole::Tangent => "tangent",
        AnchorDirectionRole::Normal => "normal",
        AnchorDirectionRole::TangentU => "tangent_u",
        AnchorDirectionRole::TangentV => "tangent_v",
    }
}
