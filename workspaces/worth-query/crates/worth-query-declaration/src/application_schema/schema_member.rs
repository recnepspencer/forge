use worth_foundational::facade::ScalarAspectType;

use super::authorization_policy::ApplicationAuthorizationPath;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationOperationProgramTarget {
    Create {
        entity: String,
    },
    Delete {
        entity: String,
    },
    Write {
        entity: String,
        aspect: String,
        field: String,
    },
    Link {
        relation: String,
        from: String,
        to: String,
    },
    Unlink {
        relation: String,
        from: String,
        to: String,
    },
    Emit {
        effect: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationSchemaMember {
    Entity {
        entity: String,
    },
    Aspect {
        entity: String,
        aspect: String,
    },
    Field {
        entity: String,
        aspect: String,
        field: String,
        scalar_family: ScalarAspectType,
        value_type: String,
        currency: Option<String>,
        writable: bool,
        equality_queryable: bool,
    },
    Relation {
        relation: String,
        from: String,
        to: String,
    },
    PrincipalBinding {
        binding: String,
        mapping_entity: String,
        identity_aspect: String,
        identity_field: String,
        status_aspect: String,
        status_field: String,
        target_relation: String,
        principal_entity: String,
        principal_identity_aspect: String,
        principal_identity_field: String,
        principal_identity_scalar_family: ScalarAspectType,
        principal_identity_value_type: String,
    },
    Operation {
        operation: String,
        input_type: String,
    },
    OperationProgram {
        operation: String,
        target: ApplicationOperationProgramTarget,
    },
    Policy {
        policy: String,
    },
    Ability {
        ability: String,
        scope_entity: String,
    },
    OperationAbility {
        operation: String,
        ability: String,
        scope_entity: String,
    },
    AbilityPolicy {
        ability: String,
        scope_entity: String,
        policy: String,
        paths: Vec<ApplicationAuthorizationPath>,
    },
    Currency {
        currency: String,
    },
    Effect {
        effect: String,
        payload_type: String,
    },
}
