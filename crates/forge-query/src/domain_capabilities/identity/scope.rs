use crate::evidence_identity::ForgeQueryEvidenceIdentityEncoder;
use crate::projection_consumption::ProjectionFactRequest;
use crate::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag};

pub(crate) fn domain_capability_scope_encoder(
    identity_family: &str,
) -> ForgeQueryEvidenceIdentityEncoder {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::DomainCapabilityIdentity)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            identity_family,
        )
}

pub(crate) fn domain_capability_certification_scope_encoder(
    identity_family: &str,
) -> ForgeQueryEvidenceIdentityEncoder {
    ForgeQueryEvidenceIdentity::compose(
        ForgeQueryEvidenceScope::DomainCapabilityCertificationIdentity,
    )
    .field_shape(
        ForgeQueryEvidenceTag::new("identity_family"),
        identity_family,
    )
}

pub(crate) fn seal(encoder: ForgeQueryEvidenceIdentityEncoder) -> String {
    encoder.seal().as_str().to_string()
}

pub(crate) fn compose_sequence_digest(
    identity_family: &str,
    tag: &'static str,
    values: impl IntoIterator<Item = impl AsRef<str>>,
) -> String {
    seal(
        domain_capability_scope_encoder(identity_family)
            .field_value_sequence(ForgeQueryEvidenceTag::new(tag), values),
    )
}

pub(crate) fn compose_certification_sequence_digest(
    identity_family: &str,
    tag: &'static str,
    values: impl IntoIterator<Item = impl AsRef<str>>,
) -> String {
    seal(
        domain_capability_certification_scope_encoder(identity_family)
            .field_value_sequence(ForgeQueryEvidenceTag::new(tag), values),
    )
}

pub(crate) fn compose_labeled_entry_digest(
    identity_family: &'static str,
    entries: &[(&'static str, &str)],
) -> String {
    let mut encoder = domain_capability_certification_scope_encoder(identity_family);
    for (tag, value) in entries {
        encoder = encoder.field_shape(ForgeQueryEvidenceTag::new(tag), *value);
    }
    seal(encoder)
}

pub(crate) fn compose_fact_request_entry_digest(request: &ProjectionFactRequest) -> String {
    let mut encoder = domain_capability_scope_encoder("projection_fact_request_entry_v1")
        .field_shape(ForgeQueryEvidenceTag::new("kind"), request.kind().as_str());
    if let Some(field) = request.field_key() {
        encoder = encoder.field_shape(ForgeQueryEvidenceTag::new("field"), field);
    }
    seal(encoder)
}
