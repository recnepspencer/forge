use crate::application_schema::canonical_authorization_identity::append_authorization_path;
use crate::application_schema::canonical_basis::ApplicationSchemaCanonicalBasis;
use crate::application_schema::ApplicationSchemaMember;

pub(super) fn append_authorization_member(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    member: &ApplicationSchemaMember,
) {
    match member {
        ApplicationSchemaMember::Policy { policy } => {
            basis.text(format!("{prefix}.kind"), "policy");
            basis.text(format!("{prefix}.policy"), policy);
        }
        ApplicationSchemaMember::Ability {
            ability,
            scope_entity,
        } => {
            basis.text(format!("{prefix}.kind"), "ability");
            basis.text(format!("{prefix}.ability"), ability);
            basis.text(format!("{prefix}.scope-entity"), scope_entity);
        }
        ApplicationSchemaMember::OperationAbility {
            operation,
            ability,
            scope_entity,
        } => {
            basis.text(format!("{prefix}.kind"), "operation-ability");
            basis.text(format!("{prefix}.operation"), operation);
            basis.text(format!("{prefix}.ability"), ability);
            basis.text(format!("{prefix}.scope-entity"), scope_entity);
        }
        ApplicationSchemaMember::AbilityPolicy {
            ability,
            scope_entity,
            policy,
            paths,
        } => {
            basis.text(format!("{prefix}.kind"), "ability-policy");
            basis.text(format!("{prefix}.ability"), ability);
            basis.text(format!("{prefix}.scope-entity"), scope_entity);
            basis.text(format!("{prefix}.policy"), policy);
            basis.usize(format!("{prefix}.path-count"), paths.len());
            for (index, path) in paths.iter().enumerate() {
                append_authorization_path(basis, &format!("{prefix}.path[{index}]"), path);
            }
        }
        _ => unreachable!("authorization member router supplied another member family"),
    }
}
