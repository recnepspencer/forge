use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

pub(super) fn basis_lifecycle_digest(
    family: &'static str,
    fields: impl IntoIterator<Item = (&'static str, String)>,
) -> String {
    let mut encoder = forge_query_evidence_identity(ForgeQueryEvidenceScope::BasisDigest)
        .field_shape(ForgeQueryEvidenceTag::new("identity_family"), family);
    for (tag, value) in fields {
        encoder = encoder.field_value(ForgeQueryEvidenceTag::new(tag), value);
    }
    encoder.seal().as_str().to_string()
}
