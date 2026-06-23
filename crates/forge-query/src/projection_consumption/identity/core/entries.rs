use super::super::scope::{consumption_scope_encoder, seal};
use crate::ForgeQueryEvidenceTag;

use super::super::super::contracts::BoundProjectionFactFamily;
use super::super::super::facts::ProjectionFactRequest;
use super::super::super::source::ProjectionSourceReferenceIdentity;

pub(crate) fn compose_fact_request_entry_digest(request: &ProjectionFactRequest) -> String {
    let mut encoder = consumption_scope_encoder("projection_fact_request_entry_v1")
        .field_shape(ForgeQueryEvidenceTag::new("kind"), request.kind().as_str());
    if let Some(field) = request.field_path() {
        encoder = encoder.field_shape(
            ForgeQueryEvidenceTag::new("field"),
            field.terminal_projection_for_boundary(),
        );
    }
    seal(encoder)
}

pub(crate) fn compose_projection_source_reference_entry_digest(
    identity: &ProjectionSourceReferenceIdentity,
) -> String {
    seal(
        consumption_scope_encoder("projection_source_reference_entry_v1")
            .field_shape(ForgeQueryEvidenceTag::new("label"), identity.label())
            .field_shape(ForgeQueryEvidenceTag::new("identity"), identity.identity()),
    )
}

pub(crate) fn compose_bound_fact_family_entry_digest(
    fact_family: &BoundProjectionFactFamily,
) -> String {
    let mut encoder = consumption_scope_encoder("projection_bound_fact_family_entry_v1")
        .field_shape(
            ForgeQueryEvidenceTag::new("kind"),
            fact_family.kind().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("support_posture"),
            fact_family.support_posture().as_str(),
        );
    if let Some(field_path) = fact_family.field_path() {
        encoder = encoder.field_shape(
            ForgeQueryEvidenceTag::new("field"),
            field_path.terminal_projection_for_boundary(),
        );
    }
    seal(encoder)
}
