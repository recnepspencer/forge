mod entity_records;
mod relation_records;

pub(super) use entity_records::{
    materialize_authoritative_entity_record_at_version,
    materialize_current_authoritative_entity_record,
};
pub(super) use relation_records::{
    materialize_authoritative_relation_record_at_version,
    materialize_current_authoritative_relation_record,
};
