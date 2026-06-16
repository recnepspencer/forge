use super::entries::compose_fact_request_entry_digest;
use super::super::scope::{consumption_scope_encoder, seal};
use crate::ForgeQueryEvidenceTag;

use super::super::super::declaration::ProjectionConsumptionBindingContext;
use super::super::super::facts::ProjectionFactRequest;
use super::super::super::source::ProjectionConsumptionSource;

pub(crate) fn compose_declaration_digest<'a>(
    source: &ProjectionConsumptionSource,
    binding: &ProjectionConsumptionBindingContext,
    requested: impl IntoIterator<Item = &'a ProjectionFactRequest>,
) -> String {
    let mut encoder = consumption_scope_encoder("projection_consumption_declaration_v1")
        .field_shape(
            ForgeQueryEvidenceTag::new("source_family"),
            source.family().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_identity"),
            source.source_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("result_shape"),
            binding.result_shape_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("authorized_projection"),
            binding.authorized_projection_identity(),
        );
    if let Some(query_digest) = source.query_digest() {
        encoder = encoder.field_shape(ForgeQueryEvidenceTag::new("query"), query_digest);
    }
    if let Some(basis_digest) = source.basis_digest() {
        encoder = encoder.field_shape(ForgeQueryEvidenceTag::new("basis"), basis_digest);
    }
    if let Some(result_digest) = source.result_digest() {
        encoder = encoder.field_shape(ForgeQueryEvidenceTag::new("result"), result_digest);
    }
    let fact_requests = requested
        .into_iter()
        .map(compose_fact_request_entry_digest)
        .collect::<Vec<_>>();
    seal(encoder.field_value_sequence(ForgeQueryEvidenceTag::new("requested_fact"), fact_requests))
}
