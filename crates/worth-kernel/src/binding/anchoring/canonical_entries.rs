use forge_query::facade::ForgeQueryDeclarationCanonicalEntry;
use worth_spatial::facade::bindings::SpatialCanonicalDeclarationField;

use crate::binding::anchoring::AuthorPrimitiveAnchorBindingIntent;

pub(super) fn canonical_query_entries_for_intent(
    intent: &AuthorPrimitiveAnchorBindingIntent,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    match intent {
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToFace(spec, anchor_spec) => {
            extend_with_point_anchor_entries(spec.canonical_declaration_fields(), anchor_spec)
        }
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToEdge(spec, anchor_spec) => {
            extend_with_point_anchor_entries(spec.canonical_declaration_fields(), anchor_spec)
        }
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToCoedge(
            spec,
            anchor_spec,
        ) => extend_with_point_anchor_entries(spec.canonical_declaration_fields(), anchor_spec),
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToFace(
            spec,
            anchor_spec,
        ) => extend_with_direction_anchor_entries(spec.canonical_declaration_fields(), anchor_spec),
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToEdge(
            spec,
            anchor_spec,
        ) => extend_with_direction_anchor_entries(spec.canonical_declaration_fields(), anchor_spec),
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToCoedge(
            spec,
            anchor_spec,
        ) => extend_with_direction_anchor_entries(spec.canonical_declaration_fields(), anchor_spec),
    }
}

fn extend_with_point_anchor_entries(
    mut fields: Vec<SpatialCanonicalDeclarationField>,
    anchor_spec: &worth_spatial::facade::bindings::CarrierOwnedParameterPointAnchorSpec,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    fields.extend(anchor_spec.canonical_declaration_fields());
    into_query_entries(fields)
}

fn extend_with_direction_anchor_entries(
    mut fields: Vec<SpatialCanonicalDeclarationField>,
    anchor_spec: &worth_spatial::facade::bindings::CarrierOwnedParameterDirectionAnchorSpec,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    fields.extend(anchor_spec.canonical_declaration_fields());
    into_query_entries(fields)
}

fn into_query_entries(
    fields: Vec<SpatialCanonicalDeclarationField>,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    fields
        .into_iter()
        .map(|field| ForgeQueryDeclarationCanonicalEntry::text(field.locus(), field.value()))
        .collect()
}
