use super::*;
use crate::application_schema::ApplicationFieldPresence;

pub(super) fn field_member(field: &str) -> ApplicationSchemaMember {
    ApplicationSchemaMember::Field {
        entity: "Grant".to_string(),
        aspect: "Facts".to_string(),
        field: field.to_string(),
        presence: ApplicationFieldPresence::Required,
        scalar_family: ScalarAspectType::UInt64,
        value_type: std::any::type_name::<u64>().to_string(),
        unit: None,
        writable: false,
        equality_queryable: true,
    }
}

pub(super) fn resource_field_member(field: &str) -> ApplicationSchemaMember {
    ApplicationSchemaMember::Field {
        entity: "Resource".to_string(),
        aspect: "ResourceFacts".to_string(),
        field: field.to_string(),
        presence: ApplicationFieldPresence::Required,
        scalar_family: ScalarAspectType::UInt64,
        value_type: std::any::type_name::<u64>().to_string(),
        unit: None,
        writable: false,
        equality_queryable: true,
    }
}

pub(super) fn relation_member(relation: &str, from: &str, to: &str) -> ApplicationSchemaMember {
    ApplicationSchemaMember::Relation {
        relation: relation.to_string(),
        from: from.to_string(),
        to: to.to_string(),
    }
}
