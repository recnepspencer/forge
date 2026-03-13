use std::collections::HashSet;

use crate::identity::data::EntityId;
use crate::indexes::logic::IndexAccess;

pub(crate) struct InvariantIndexView<'runtime> {
    access: IndexAccess<'runtime>,
}

impl<'runtime> InvariantIndexView<'runtime> {
    pub(crate) fn new(access: IndexAccess<'runtime>) -> Self {
        Self { access }
    }

    pub(crate) fn conflicts_with_entity_value(
        &self,
        field: &str,
        value: &str,
        entity_id: Option<EntityId>,
    ) -> bool {
        self.access
            .entity_unique_field_ids(field, value)
            .is_some_and(|existing| {
                existing
                    .iter()
                    .any(|existing_id| entity_id != Some(*existing_id))
            })
    }

    pub(crate) fn conflicts_with_entity_value_outside(
        &self,
        field: &str,
        value: &str,
        excluded: &HashSet<EntityId>,
    ) -> bool {
        self.access
            .entity_unique_field_ids(field, value)
            .is_some_and(|existing| {
                existing
                    .iter()
                    .any(|existing_id| !excluded.contains(existing_id))
            })
    }

    pub(crate) fn first_duplicate_entity_value(&self, field: &str) -> Option<&str> {
        self.access
            .entity_unique_field_entries(field)?
            .iter()
            .find_map(|(value, entity_ids)| (entity_ids.len() > 1).then_some(value.as_str()))
    }
}
