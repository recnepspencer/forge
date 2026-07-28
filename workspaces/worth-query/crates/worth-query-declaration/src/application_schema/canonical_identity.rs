use sha2::{Digest, Sha256};

use super::canonical_authorization_identity::hash_authorization_path;
use super::canonical_operation_identity::hash_operation_target;
use super::{ApplicationSchemaIdentity, ApplicationSchemaMember};

pub(super) struct ApplicationSchemaCanonicalHeader<'a> {
    pub owner: &'a str,
    pub name: &'a str,
    pub major: u32,
    pub minor: u32,
}

pub(super) fn canonical_identity(
    header: ApplicationSchemaCanonicalHeader<'_>,
    members: &[ApplicationSchemaMember],
) -> ApplicationSchemaIdentity {
    let mut hash = Sha256::new();
    hash_field(&mut hash, "scheme", "worth-query-application-schema-v3");
    hash_field(&mut hash, "owner", header.owner);
    hash_field(&mut hash, "name", header.name);
    hash_field(&mut hash, "major", &header.major.to_string());
    hash_field(&mut hash, "minor", &header.minor.to_string());
    for member in members {
        hash_member(&mut hash, member);
    }
    ApplicationSchemaIdentity::from_canonical_hash(format!("{:x}", hash.finalize()))
}

fn hash_member(hash: &mut Sha256, member: &ApplicationSchemaMember) {
    match member {
        ApplicationSchemaMember::Entity { entity } => {
            hash_field(hash, "member-kind", "entity");
            hash_field(hash, "entity", entity);
        }
        ApplicationSchemaMember::Aspect { entity, aspect } => {
            hash_field(hash, "member-kind", "aspect");
            hash_field(hash, "entity", entity);
            hash_field(hash, "aspect", aspect);
        }
        ApplicationSchemaMember::Field { .. } => hash_schema_field(hash, member),
        ApplicationSchemaMember::Relation { relation, from, to } => {
            hash_field(hash, "member-kind", "relation");
            hash_field(hash, "relation", relation);
            hash_field(hash, "from", from);
            hash_field(hash, "to", to);
        }
        ApplicationSchemaMember::PrincipalBinding { .. } => {
            hash_principal_binding(hash, member);
        }
        ApplicationSchemaMember::Operation {
            operation,
            input_type,
        } => {
            hash_field(hash, "member-kind", "operation");
            hash_field(hash, "operation", operation);
            hash_field(hash, "input-type", input_type);
        }
        ApplicationSchemaMember::OperationProgram { operation, target } => {
            hash_field(hash, "member-kind", "operation-program");
            hash_field(hash, "operation", operation);
            hash_operation_target(hash, target);
        }
        ApplicationSchemaMember::Policy { policy } => {
            hash_field(hash, "member-kind", "policy");
            hash_field(hash, "policy", policy);
        }
        ApplicationSchemaMember::Ability {
            ability,
            scope_entity,
        } => {
            hash_field(hash, "member-kind", "ability");
            hash_field(hash, "ability", ability);
            hash_field(hash, "scope-entity", scope_entity);
        }
        ApplicationSchemaMember::OperationAbility {
            operation,
            ability,
            scope_entity,
        } => {
            hash_field(hash, "member-kind", "operation-ability");
            hash_field(hash, "operation", operation);
            hash_field(hash, "ability", ability);
            hash_field(hash, "scope-entity", scope_entity);
        }
        ApplicationSchemaMember::AbilityPolicy {
            ability,
            scope_entity,
            policy,
            paths,
        } => {
            hash_field(hash, "member-kind", "ability-policy");
            hash_field(hash, "ability", ability);
            hash_field(hash, "scope-entity", scope_entity);
            hash_field(hash, "policy", policy);
            for path in paths {
                hash_authorization_path(hash, path);
            }
        }
        ApplicationSchemaMember::Currency { currency } => {
            hash_field(hash, "member-kind", "currency");
            hash_field(hash, "currency", currency);
        }
        ApplicationSchemaMember::Effect {
            effect,
            payload_type,
        } => {
            hash_field(hash, "member-kind", "effect");
            hash_field(hash, "effect", effect);
            hash_field(hash, "payload-type", payload_type);
        }
    }
}

fn hash_principal_binding(hash: &mut Sha256, member: &ApplicationSchemaMember) {
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
        unreachable!("hash_principal_binding requires a principal binding")
    };
    hash_field(hash, "member-kind", "principal-binding");
    hash_field(hash, "binding", binding);
    hash_field(hash, "mapping-entity", mapping_entity);
    hash_field(hash, "identity-aspect", identity_aspect);
    hash_field(hash, "identity-field", identity_field);
    hash_field(hash, "status-aspect", status_aspect);
    hash_field(hash, "status-field", status_field);
    hash_field(hash, "target-relation", target_relation);
    hash_field(hash, "principal-entity", principal_entity);
    hash_field(hash, "principal-identity-aspect", principal_identity_aspect);
    hash_field(hash, "principal-identity-field", principal_identity_field);
    hash_field(
        hash,
        "principal-identity-scalar-family",
        principal_identity_scalar_family.canonical_name(),
    );
    hash_field(
        hash,
        "principal-identity-value-type",
        principal_identity_value_type,
    );
}

fn hash_schema_field(hash: &mut Sha256, member: &ApplicationSchemaMember) {
    let ApplicationSchemaMember::Field {
        entity,
        aspect,
        field,
        scalar_family,
        value_type,
        currency,
        writable,
        equality_queryable,
    } = member
    else {
        unreachable!("hash_schema_field requires a field member")
    };
    hash_field(hash, "member-kind", "field");
    hash_field(hash, "entity", entity);
    hash_field(hash, "aspect", aspect);
    hash_field(hash, "field", field);
    hash_field(hash, "scalar-family", scalar_family.canonical_name());
    hash_field(hash, "value-type", value_type);
    hash_field(
        hash,
        "currency",
        currency.as_deref().unwrap_or("no-application-currency"),
    );
    hash_field(hash, "writable", bool_identity(*writable));
    hash_field(
        hash,
        "equality-queryable",
        bool_identity(*equality_queryable),
    );
}

const fn bool_identity(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

pub(super) fn hash_field(hash: &mut Sha256, label: &str, value: &str) {
    hash.update(canonical_length(label).to_le_bytes());
    hash.update(label.as_bytes());
    hash.update(canonical_length(value).to_le_bytes());
    hash.update(value.as_bytes());
}

fn canonical_length(value: &str) -> u64 {
    u64::try_from(value.len()).expect("application schema identifiers are bounded below u64::MAX")
}

#[cfg(test)]
#[path = "canonical_identity_tests.rs"]
mod tests;
