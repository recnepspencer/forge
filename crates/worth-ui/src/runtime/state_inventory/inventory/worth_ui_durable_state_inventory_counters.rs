#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiDurableStateInventoryCounters {
    registered_platform_family_count: usize,
    registered_hook_family_count: usize,
    rejected_family_count: usize,
    duplicate_family_count: usize,
    transient_drop_policy_count: usize,
    replacement_classification_count: usize,
}

impl WorthUiDurableStateInventoryCounters {
    pub(crate) fn record_platform_family(&mut self) {
        self.registered_platform_family_count += 1;
    }

    pub(crate) fn record_hook_family(&mut self) {
        self.registered_hook_family_count += 1;
    }

    pub(crate) fn record_rejected_family(&mut self) {
        self.rejected_family_count += 1;
    }

    pub(crate) fn record_duplicate_family(&mut self) {
        self.duplicate_family_count += 1;
        self.record_rejected_family();
    }

    pub(crate) fn record_transient_drop_policy(&mut self) {
        self.transient_drop_policy_count += 1;
    }

    pub(crate) fn record_replacement_classifications(&mut self, count: usize) {
        self.replacement_classification_count += count;
    }

    pub fn registered_platform_family_count(&self) -> usize {
        self.registered_platform_family_count
    }

    pub fn registered_hook_family_count(&self) -> usize {
        self.registered_hook_family_count
    }

    pub fn rejected_family_count(&self) -> usize {
        self.rejected_family_count
    }

    pub fn duplicate_family_count(&self) -> usize {
        self.duplicate_family_count
    }

    pub fn transient_drop_policy_count(&self) -> usize {
        self.transient_drop_policy_count
    }

    pub fn replacement_classification_count(&self) -> usize {
        self.replacement_classification_count
    }
}
