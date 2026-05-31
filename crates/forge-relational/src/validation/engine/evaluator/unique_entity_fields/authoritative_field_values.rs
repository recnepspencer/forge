use forge_foundational::facade::AspectFieldLocator;

use crate::storage::data::{
    authoritative_aspect_value_field_comparison_key, entity_authoritative_aspect_field_value,
    AuthoritativeFieldComparisonKey,
};

use super::super::super::context::InvariantExecutionContext;

pub(super) fn visible_entity_field_value_conflict(
    context: &InvariantExecutionContext<'_>,
    field_locator: &AspectFieldLocator,
    comparison_key: &AuthoritativeFieldComparisonKey,
    include_entity: impl Fn(crate::identity::data::EntityId) -> bool,
) -> bool {
    let state_view = context.state_view();
    for partition_id in state_view.state().partition_ids() {
        if state_view.state().get_partition(partition_id).is_none() {
            continue;
        }
        let Some(slot_count) = state_view.entity_slot_scan_count(partition_id) else {
            continue;
        };
        for slot in 0..slot_count {
            let Some(metadata) = state_view.entity_metadata_for_slot(partition_id, slot) else {
                continue;
            };
            if !include_entity(metadata.entity_id) {
                continue;
            }
            let Some(record) = context.visible_unmasked_entity_record(metadata.entity_id) else {
                continue;
            };
            let Some(value) = entity_authoritative_aspect_field_value(&record, field_locator)
            else {
                continue;
            };
            if &authoritative_aspect_value_field_comparison_key(&value) == comparison_key {
                return true;
            }
        }
    }
    false
}
