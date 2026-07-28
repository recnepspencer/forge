use worth_foundational::facade::ScalarAspectType;

use super::{
    canonical_identity, ApplicationSchemaCanonicalHeader, ApplicationSchemaIdentity,
    ApplicationSchemaMember,
};
use crate::application_schema::ApplicationOperationProgramTarget;

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
        principal_binding("PrincipalBinding"),
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
fn every_principal_binding_dimension_changes_identity() {
    let base = principal_binding("PrincipalBinding");
    let base_identity = identity(std::slice::from_ref(&base));
    macro_rules! changed {
        ($field:ident, $value:literal) => {{
            let mut member = base.clone();
            let ApplicationSchemaMember::PrincipalBinding { $field, .. } = &mut member else {
                unreachable!("principal binding fixture changed member family")
            };
            *$field = $value.to_string();
            member
        }};
    }
    for member in [
        changed!(binding, "OtherBinding"),
        changed!(mapping_entity, "OtherMapping"),
        changed!(identity_aspect, "OtherIdentityAspect"),
        changed!(identity_field, "OtherIdentityField"),
        changed!(status_aspect, "OtherStatusAspect"),
        changed!(status_field, "OtherStatusField"),
        changed!(target_relation, "OtherTargetRelation"),
        changed!(principal_entity, "OtherPrincipal"),
        changed!(principal_identity_aspect, "OtherPrincipalIdentityAspect"),
        changed!(principal_identity_field, "OtherPrincipalIdentityField"),
        changed!(principal_identity_value_type, "OtherPrincipalIdentityValue"),
    ] {
        assert_ne!(identity(&[member]), base_identity);
    }
    let mut changed_family = base;
    let ApplicationSchemaMember::PrincipalBinding {
        principal_identity_scalar_family,
        ..
    } = &mut changed_family
    else {
        unreachable!("principal binding fixture changed member family")
    };
    *principal_identity_scalar_family = ScalarAspectType::Bool;
    assert_ne!(identity(&[changed_family]), base_identity);
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

fn principal_binding(binding: &str) -> ApplicationSchemaMember {
    ApplicationSchemaMember::PrincipalBinding {
        binding: binding.to_string(),
        mapping_entity: "ExternalPrincipalMapping".to_string(),
        identity_aspect: "ExternalIdentity".to_string(),
        identity_field: "ExternalIdentityField".to_string(),
        status_aspect: "ExternalIdentity".to_string(),
        status_field: "ExternalMappingStatus".to_string(),
        target_relation: "ExternalPrincipalTarget".to_string(),
        principal_entity: "Principal".to_string(),
        principal_identity_aspect: "PrincipalIdentity".to_string(),
        principal_identity_field: "PrincipalIdentityField".to_string(),
        principal_identity_scalar_family: ScalarAspectType::UInt64,
        principal_identity_value_type: "u64".to_string(),
    }
}
