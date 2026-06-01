use forge_foundational::facade::{
    AspectFieldLocator, AspectKey, AspectValue, CanonicalFieldPath, FieldKey, LocatorAuthority,
};
use forge_relational::facade::transactions::AspectFieldPatch;
use schema::facade::platform::aspects::{
    entity_domain_aspect, entity_domain_field, Aspect, NamingAspect,
};
use schema::facade::platform::entities::EntityKind;

use super::field_key;

pub(crate) fn topology_entity_create_fields(kind: EntityKind, structure: &str) -> AspectFieldPatch {
    entity_string_field_patch(
        entity_domain_aspect(kind).aspect_key(),
        entity_domain_field(kind),
        structure,
    )
}

pub(crate) fn persistent_name_create_fields(persistent_name: &str) -> AspectFieldPatch {
    entity_string_field_patch(
        Aspect::Naming(NamingAspect::PersistentName).aspect_key(),
        "persistent_name",
        persistent_name,
    )
}

fn entity_string_field_patch(aspect_key: AspectKey, field: &str, value: &str) -> AspectFieldPatch {
    AspectFieldPatch::from_locator(
        planned_single_field_locator(aspect_key, field_key(field)),
        AspectValue::String(value.to_string().into()),
    )
}

fn planned_single_field_locator(aspect_key: AspectKey, field_key: FieldKey) -> AspectFieldLocator {
    AspectFieldLocator::new(
        LocatorAuthority::Planned,
        aspect_key,
        CanonicalFieldPath::single(field_key),
    )
}
