mod bulk_create_entities;
mod bulk_create_relations;
mod create_entity;
mod create_relation;
mod delete_entity;
mod delete_relation;
mod dispatch;
pub(super) mod entity_authoritative_deletion_patch;
mod entity_authoritative_patch_application;
mod entity_field_aspect_patch;
mod entity_field_creation_aspects;
mod relation_field_creation_aspects;
mod replace_entity;
mod struct_field_value_set;
mod update_entity_fields;
mod update_relation_endpoints;

pub(crate) use dispatch::dispatch_intent;
pub(crate) use entity_field_aspect_patch::plan_entity_field_aspect_patch;
