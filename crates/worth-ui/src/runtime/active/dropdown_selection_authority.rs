use std::collections::BTreeMap;

use crate::runtime::WorthUiDropdownSelectionState;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthUiDropdownSelectionAuthority {
    selections: BTreeMap<String, WorthUiDropdownSelectionState>,
}

impl WorthUiDropdownSelectionAuthority {
    pub(crate) fn selection_state(
        &self,
        projection_id: &str,
    ) -> Option<&WorthUiDropdownSelectionState> {
        self.selections.get(projection_id)
    }

    pub(crate) fn record_selection_state(
        &mut self,
        projection_id: &str,
        selection_state: &WorthUiDropdownSelectionState,
    ) {
        if selection_state.selected_command_ids().is_empty() {
            self.selections.remove(projection_id);
            return;
        }

        self.selections
            .insert(projection_id.to_owned(), selection_state.clone());
    }
}
