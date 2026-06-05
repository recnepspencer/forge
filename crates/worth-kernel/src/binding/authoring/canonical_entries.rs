use forge_query::facade::ForgeQueryDeclarationCanonicalEntry;
use worth_spatial::facade::bindings::SpatialCanonicalDeclarationField;

use crate::binding::authoring::AuthorPrimitiveBindingIntent;

pub(super) fn canonical_query_entries_for_intent(
    intent: &AuthorPrimitiveBindingIntent,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    match intent {
        AuthorPrimitiveBindingIntent::AttachSurfaceToFace(spec) => {
            into_query_entries(spec.canonical_declaration_fields())
        }
        AuthorPrimitiveBindingIntent::AttachCurveToEdge(spec) => {
            into_query_entries(spec.canonical_declaration_fields())
        }
        AuthorPrimitiveBindingIntent::AttachPCurveToCoedge(spec) => {
            into_query_entries(spec.canonical_declaration_fields())
        }
        AuthorPrimitiveBindingIntent::AttachVertexGeometry(spec) => {
            into_query_entries(spec.canonical_declaration_fields())
        }
    }
}

fn into_query_entries(
    fields: Vec<SpatialCanonicalDeclarationField>,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    fields
        .into_iter()
        .map(|field| ForgeQueryDeclarationCanonicalEntry::text(field.locus(), field.value()))
        .collect()
}
