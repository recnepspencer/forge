use sha2::{Digest, Sha256};

use super::{
    ApplicationOperationProgramTarget, ApplicationSchemaIdentity, ApplicationSchemaMember,
};

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
    hash_field(&mut hash, "scheme", "worth-query-application-schema-v1");
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

fn hash_operation_target(hash: &mut Sha256, target: &ApplicationOperationProgramTarget) {
    match target {
        ApplicationOperationProgramTarget::Create { entity } => {
            hash_field(hash, "program-action", "create");
            hash_field(hash, "entity", entity);
        }
        ApplicationOperationProgramTarget::Delete { entity } => {
            hash_field(hash, "program-action", "delete");
            hash_field(hash, "entity", entity);
        }
        ApplicationOperationProgramTarget::Write {
            entity,
            aspect,
            field,
        } => {
            hash_field(hash, "program-action", "write");
            hash_field(hash, "entity", entity);
            hash_field(hash, "aspect", aspect);
            hash_field(hash, "field", field);
        }
        ApplicationOperationProgramTarget::Link { relation, from, to } => {
            hash_field(hash, "program-action", "link");
            hash_field(hash, "relation", relation);
            hash_field(hash, "from", from);
            hash_field(hash, "to", to);
        }
        ApplicationOperationProgramTarget::Unlink { relation, from, to } => {
            hash_field(hash, "program-action", "unlink");
            hash_field(hash, "relation", relation);
            hash_field(hash, "from", from);
            hash_field(hash, "to", to);
        }
        ApplicationOperationProgramTarget::Emit { effect } => {
            hash_field(hash, "program-action", "emit");
            hash_field(hash, "effect", effect);
        }
    }
}

const fn bool_identity(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

fn hash_field(hash: &mut Sha256, label: &str, value: &str) {
    hash.update(label.len().to_le_bytes());
    hash.update(label.as_bytes());
    hash.update(value.len().to_le_bytes());
    hash.update(value.as_bytes());
}

#[cfg(test)]
mod tests {
    use worth_foundational::facade::ScalarAspectType;

    use super::*;

    #[test]
    fn every_application_schema_member_family_changes_identity() {
        let empty = identity(&[]);
        for member in [
            ApplicationSchemaMember::Entity {
                entity: "Entity".to_string(),
            },
            ApplicationSchemaMember::Aspect {
                entity: "Entity".to_string(),
                aspect: "Aspect".to_string(),
            },
            field((ScalarAspectType::UInt64, "u64", None, false, false)),
            ApplicationSchemaMember::Relation {
                relation: "Relation".to_string(),
                from: "From".to_string(),
                to: "To".to_string(),
            },
            ApplicationSchemaMember::Operation {
                operation: "Operation".to_string(),
                input_type: "Input".to_string(),
            },
            ApplicationSchemaMember::OperationProgram {
                operation: "Operation".to_string(),
                target: ApplicationOperationProgramTarget::Create {
                    entity: "Entity".to_string(),
                },
            },
            ApplicationSchemaMember::Policy {
                policy: "Policy".to_string(),
            },
            ApplicationSchemaMember::Currency {
                currency: "USD".to_string(),
            },
            ApplicationSchemaMember::Effect {
                effect: "Effect".to_string(),
                payload_type: "Payload".to_string(),
            },
        ] {
            assert_ne!(identity(&[member]), empty);
        }
    }

    #[test]
    fn every_field_capability_and_type_dimension_changes_identity() {
        let base = identity(&[field((ScalarAspectType::UInt64, "u64", None, false, false))]);
        for changed in [
            field((ScalarAspectType::Int64, "u64", None, false, false)),
            field((ScalarAspectType::UInt64, "Other", None, false, false)),
            field((ScalarAspectType::UInt64, "u64", Some("USD"), false, false)),
            field((ScalarAspectType::UInt64, "u64", None, true, false)),
            field((ScalarAspectType::UInt64, "u64", None, false, true)),
        ] {
            assert_ne!(identity(&[changed]), base);
        }
    }

    #[test]
    fn operation_input_effect_payload_and_schema_version_change_identity() {
        let operation = ApplicationSchemaMember::Operation {
            operation: "Operation".to_string(),
            input_type: "Input".to_string(),
        };
        let changed_operation = ApplicationSchemaMember::Operation {
            operation: "Operation".to_string(),
            input_type: "OtherInput".to_string(),
        };
        assert_ne!(identity(&[operation]), identity(&[changed_operation]));

        let effect = ApplicationSchemaMember::Effect {
            effect: "Effect".to_string(),
            payload_type: "Payload".to_string(),
        };
        let changed_effect = ApplicationSchemaMember::Effect {
            effect: "Effect".to_string(),
            payload_type: "OtherPayload".to_string(),
        };
        assert_ne!(identity(&[effect]), identity(&[changed_effect]));

        let initial = canonical_identity(header(0), &[]);
        let successor = canonical_identity(header(1), &[]);
        assert_ne!(initial, successor);
    }

    #[test]
    fn every_operation_program_action_changes_identity() {
        let base = identity(&[]);
        for target in [
            ApplicationOperationProgramTarget::Create {
                entity: "Entity".to_string(),
            },
            ApplicationOperationProgramTarget::Delete {
                entity: "Entity".to_string(),
            },
            ApplicationOperationProgramTarget::Write {
                entity: "Entity".to_string(),
                aspect: "Aspect".to_string(),
                field: "Field".to_string(),
            },
            ApplicationOperationProgramTarget::Link {
                relation: "Relation".to_string(),
                from: "From".to_string(),
                to: "To".to_string(),
            },
            ApplicationOperationProgramTarget::Unlink {
                relation: "Relation".to_string(),
                from: "From".to_string(),
                to: "To".to_string(),
            },
            ApplicationOperationProgramTarget::Emit {
                effect: "Effect".to_string(),
            },
        ] {
            let program = ApplicationSchemaMember::OperationProgram {
                operation: "Operation".to_string(),
                target,
            };
            assert_ne!(identity(&[program]), base);
        }
    }

    fn identity(members: &[ApplicationSchemaMember]) -> ApplicationSchemaIdentity {
        canonical_identity(header(0), members)
    }

    fn header(minor: u32) -> ApplicationSchemaCanonicalHeader<'static> {
        ApplicationSchemaCanonicalHeader {
            owner: "owner",
            name: "Schema",
            major: 1,
            minor,
        }
    }

    fn field(
        dimensions: (ScalarAspectType, &str, Option<&str>, bool, bool),
    ) -> ApplicationSchemaMember {
        let (scalar_family, value_type, currency, writable, equality_queryable) = dimensions;
        ApplicationSchemaMember::Field {
            entity: "Entity".to_string(),
            aspect: "Aspect".to_string(),
            field: "Field".to_string(),
            scalar_family,
            value_type: value_type.to_string(),
            currency: currency.map(str::to_string),
            writable,
            equality_queryable,
        }
    }
}
