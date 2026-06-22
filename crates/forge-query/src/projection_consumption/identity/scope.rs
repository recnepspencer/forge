use crate::evidence_identity::ForgeQueryEvidenceIdentityEncoder;
use crate::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag};

pub(crate) fn consumption_scope_encoder(
    identity_family: &str,
) -> ForgeQueryEvidenceIdentityEncoder {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::ProjectionConsumptionIdentity)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            identity_family,
        )
}

pub(crate) fn certification_scope_encoder(
    identity_family: &str,
) -> ForgeQueryEvidenceIdentityEncoder {
    ForgeQueryEvidenceIdentity::compose(
        ForgeQueryEvidenceScope::ProjectionConsumptionCertificationIdentity,
    )
    .field_shape(
        ForgeQueryEvidenceTag::new("identity_family"),
        identity_family,
    )
}

pub(crate) fn scope_encoder(identity_family: &str) -> ForgeQueryEvidenceIdentityEncoder {
    consumption_scope_encoder(identity_family)
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
        consumption_scope_encoder(identity_family)
            .field_value_sequence(ForgeQueryEvidenceTag::new(tag), values),
    )
}

pub(crate) fn compose_certification_sequence_digest(
    identity_family: &str,
    tag: &'static str,
    values: impl IntoIterator<Item = impl AsRef<str>>,
) -> String {
    seal(
        certification_scope_encoder(identity_family)
            .field_value_sequence(ForgeQueryEvidenceTag::new(tag), values),
    )
}

pub(crate) fn compose_labeled_entry_digest(
    identity_family: &'static str,
    entries: &[(&'static str, &str)],
) -> String {
    let mut encoder = certification_scope_encoder(identity_family);
    for (tag, value) in entries {
        encoder = encoder.field_shape(ForgeQueryEvidenceTag::new(tag), *value);
    }
    seal(encoder)
}
