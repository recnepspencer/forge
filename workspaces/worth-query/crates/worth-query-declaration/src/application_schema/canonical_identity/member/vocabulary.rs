use crate::application_schema::canonical_basis::ApplicationSchemaCanonicalBasis;
use crate::application_schema::ApplicationSchemaMember;

pub(super) fn append_vocabulary_member(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    member: &ApplicationSchemaMember,
) {
    match member {
        ApplicationSchemaMember::Unit { unit } => {
            basis.text(format!("{prefix}.kind"), "unit");
            basis.text(format!("{prefix}.unit"), unit);
        }
        ApplicationSchemaMember::Effect {
            effect,
            payload_type,
        } => {
            basis.text(format!("{prefix}.kind"), "effect");
            basis.text(format!("{prefix}.effect"), effect);
            basis.text(format!("{prefix}.payload-type"), payload_type.as_str());
        }
        _ => unreachable!("vocabulary member router supplied another member family"),
    }
}
