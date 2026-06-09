#[cfg(test)]
use forge_relational::facade::runtime::EntityReadRecord;

#[cfg(test)]
use crate::relational_aspect_boundary::entity_record_domain_label;

#[cfg(test)]
pub fn entity_label(record: &EntityReadRecord) -> String {
    entity_record_domain_label(record).unwrap_or_else(|| record.kind.kind_name.clone())
}
