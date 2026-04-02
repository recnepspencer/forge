mod bulk_create_entities;
mod bulk_create_relations;
mod create_entity;
mod create_relation;
mod delete_entity;
mod delete_relation;
mod dispatch;
mod replace_entity;
mod update_entity;
mod update_entity_fields;

pub(crate) use dispatch::dispatch_intent;
