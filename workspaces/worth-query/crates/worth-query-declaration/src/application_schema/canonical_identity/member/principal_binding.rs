use crate::application_schema::canonical_basis::ApplicationSchemaCanonicalBasis;
use crate::application_schema::ApplicationSchemaMember;

pub(super) fn append_principal_binding(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    member: &ApplicationSchemaMember,
) {
    let ApplicationSchemaMember::PrincipalBinding {
        binding,
        mapping_entity,
        identity_aspect,
        identity_field,
        status_aspect,
        status_field,
        target_relation,
        principal_entity,
        principal_identity_aspect,
        principal_identity_field,
        principal_identity_scalar_family,
        principal_identity_value_type,
    } = member
    else {
        unreachable!("principal-binding lowering requires a principal-binding member")
    };
    basis.text(format!("{prefix}.kind"), "principal-binding");
    basis.text(format!("{prefix}.binding"), binding);
    basis.text(format!("{prefix}.mapping-entity"), mapping_entity);
    basis.text(format!("{prefix}.identity-aspect"), identity_aspect);
    basis.text(format!("{prefix}.identity-field"), identity_field);
    basis.text(format!("{prefix}.status-aspect"), status_aspect);
    basis.text(format!("{prefix}.status-field"), status_field);
    basis.text(format!("{prefix}.target-relation"), target_relation);
    basis.text(format!("{prefix}.principal-entity"), principal_entity);
    basis.text(
        format!("{prefix}.principal-identity-aspect"),
        principal_identity_aspect,
    );
    basis.text(
        format!("{prefix}.principal-identity-field"),
        principal_identity_field,
    );
    basis.text(
        format!("{prefix}.principal-identity-scalar-family"),
        principal_identity_scalar_family.canonical_name(),
    );
    basis.text(
        format!("{prefix}.principal-identity-value-type"),
        principal_identity_value_type,
    );
}
