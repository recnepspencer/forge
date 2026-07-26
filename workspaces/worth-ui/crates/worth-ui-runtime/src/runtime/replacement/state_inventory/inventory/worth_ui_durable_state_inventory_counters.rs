#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiDurableStateInventoryCounters {
    registered_platform_family_count: usize,
    registered_application_family_count: usize,
    transient_drop_policy_count: usize,
    replacement_classification_count: usize,
}

impl WorthUiDurableStateInventoryCounters {
    pub(crate) fn record_platform_families(&mut self, count: usize) {
        self.registered_platform_family_count += count;
    }

    pub(crate) fn record_application_families(&mut self, count: usize) {
        self.registered_application_family_count += count;
    }

    pub(crate) fn record_transient_drop_policies(&mut self, count: usize) {
        self.transient_drop_policy_count += count;
    }

    pub(crate) fn record_replacement_classifications(&mut self, count: usize) {
        self.replacement_classification_count += count;
    }

    pub(crate) fn registered_platform_family_count(&self) -> usize {
        self.registered_platform_family_count
    }

    pub(crate) fn registered_application_family_count(&self) -> usize {
        self.registered_application_family_count
    }

    pub(crate) fn transient_drop_policy_count(&self) -> usize {
        self.transient_drop_policy_count
    }

    pub(crate) fn replacement_classification_count(&self) -> usize {
        self.replacement_classification_count
    }
}
