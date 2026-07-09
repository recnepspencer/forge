mod history;
mod kind_matching;
mod lifecycle;
mod slot_sets;

pub(super) use history::visible_metadata;
pub(super) use kind_matching::{
    entity_slot_matches_kind_at_version, relation_slot_matches_kind_at_version,
    slot_kind_matches_current,
};
pub(super) use lifecycle::{historical_lifecycle, lifecycle_storage_visible};
pub(super) use slot_sets::{
    entity_visible_in_partition_at_version, relation_visible_in_partition_at_version,
    visible_slots_in_partition_from_state,
};

#[cfg(test)]
mod tests;
