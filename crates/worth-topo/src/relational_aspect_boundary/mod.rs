mod entity_authoritative_reads;
mod entity_field_declarations;

pub(crate) use entity_authoritative_reads::{
    entity_record_domain_label, entity_record_string_aspect,
};
pub(crate) use entity_field_declarations::{
    persistent_name_create_fields, topology_entity_create_fields,
};

use forge_foundational::facade::FieldKey;

fn field_key(label: &str) -> FieldKey {
    FieldKey::new(label).expect("worth topology aspect field must be foundational")
}
