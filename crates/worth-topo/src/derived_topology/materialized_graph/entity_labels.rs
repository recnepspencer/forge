use forge_relational::facade::runtime::EntityReadRecord;

pub fn entity_label(record: &EntityReadRecord) -> String {
    crate::relational_aspect_boundary::entity_record_domain_label(record)
        .unwrap_or_else(|| record.kind.kind_name.clone())
}
