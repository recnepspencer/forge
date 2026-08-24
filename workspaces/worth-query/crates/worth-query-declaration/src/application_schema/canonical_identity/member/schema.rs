use crate::application_schema::canonical_basis::ApplicationSchemaCanonicalBasis;
use crate::application_schema::ApplicationSchemaMember;

use super::field::append_schema_field;
use super::principal_binding::append_principal_binding;

pub(super) fn append_schema_member(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    member: &ApplicationSchemaMember,
) {
    match member {
        ApplicationSchemaMember::Entity { entity } => {
            basis.text(format!("{prefix}.kind"), "entity");
            basis.text(format!("{prefix}.entity"), entity);
        }
        ApplicationSchemaMember::Aspect {
            entity,
            aspect,
            identity,
            revision,
        } => {
            basis.text(format!("{prefix}.kind"), "aspect");
            basis.text(format!("{prefix}.entity"), entity);
            basis.text(format!("{prefix}.aspect"), aspect);
            basis.u64(format!("{prefix}.identity"), identity.0);
            basis.u64(format!("{prefix}.revision"), revision.0);
        }
        ApplicationSchemaMember::Field { .. } => append_schema_field(basis, prefix, member),
        ApplicationSchemaMember::Relation { relation, from, to } => {
            basis.text(format!("{prefix}.kind"), "relation");
            basis.text(format!("{prefix}.relation"), relation);
            basis.text(format!("{prefix}.from"), from);
            basis.text(format!("{prefix}.to"), to);
        }
        ApplicationSchemaMember::PrincipalBinding { .. } => {
            append_principal_binding(basis, prefix, member)
        }
        _ => unreachable!("schema member router supplied a non-schema member"),
    }
}
