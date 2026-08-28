#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiSelectionDelta {
    added: Box<[super::UiSelectionStableKey]>,
    removed: Box<[super::UiSelectionStableKey]>,
    selected_count: usize,
    candidates_visited: u32,
    revision: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiSelectionReconciliationReceipt {
    delta: UiSelectionDelta,
    order_changed: bool,
    missing_keys_preserved_for_partial_catalog: usize,
}

impl UiSelectionDelta {
    pub(super) fn new(
        added: Vec<super::UiSelectionStableKey>,
        removed: Vec<super::UiSelectionStableKey>,
        selected_count: usize,
        candidates_visited: u32,
        revision: u64,
    ) -> Self {
        Self {
            added: added.into_boxed_slice(),
            removed: removed.into_boxed_slice(),
            selected_count,
            candidates_visited,
            revision,
        }
    }

    pub(crate) fn added(&self) -> &[super::UiSelectionStableKey] {
        &self.added
    }
    pub(crate) fn removed(&self) -> &[super::UiSelectionStableKey] {
        &self.removed
    }
    pub(crate) const fn selected_count(&self) -> usize {
        self.selected_count
    }
    pub(crate) const fn candidates_visited(&self) -> u32 {
        self.candidates_visited
    }
    pub(crate) const fn revision(&self) -> u64 {
        self.revision
    }
}

impl UiSelectionReconciliationReceipt {
    pub(super) const fn new(
        delta: UiSelectionDelta,
        order_changed: bool,
        missing_keys_preserved_for_partial_catalog: usize,
    ) -> Self {
        Self {
            delta,
            order_changed,
            missing_keys_preserved_for_partial_catalog,
        }
    }

    pub(crate) const fn delta(&self) -> &UiSelectionDelta {
        &self.delta
    }
    pub(crate) const fn order_changed(&self) -> bool {
        self.order_changed
    }
    pub(crate) const fn missing_keys_preserved_for_partial_catalog(&self) -> usize {
        self.missing_keys_preserved_for_partial_catalog
    }
}
