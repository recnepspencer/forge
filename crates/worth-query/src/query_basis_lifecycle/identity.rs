use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};

pub(super) fn basis_lifecycle_digest(
    family: &'static str,
    fields: impl IntoIterator<Item = (&'static str, String)>,
) -> String {
    let mut encoder = worth_query_evidence_identity(WorthQueryEvidenceScope::BasisDigest)
        .field_shape(WorthQueryEvidenceTag::new("identity_family"), family);
    for (tag, value) in fields {
        encoder = encoder.field_value(WorthQueryEvidenceTag::new(tag), value);
    }
    encoder.seal().as_str().to_string()
}
