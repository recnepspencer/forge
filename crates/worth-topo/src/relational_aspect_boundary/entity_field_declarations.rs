use forge_foundational::facade::{AspectKey, AspectValue};
use forge_relational::facade::transactions::{AspectFieldPatch, AspectFieldPatchTarget};
use schema::facade::{entity_domain_aspect, entity_domain_field, Aspect, EntityKind, NamingAspect};

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
    AspectFieldPatch::from_target(
        AspectFieldPatchTarget::single(aspect_key, field_key(field)),
        AspectValue::String(value.to_string().into()),
    )
}
