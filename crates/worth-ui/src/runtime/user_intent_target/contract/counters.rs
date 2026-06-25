#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiUserIntentTargetCounters {
    page_slot_lookup_count: usize,
    page_slot_scan_count: usize,
    mounted_surface_lookup_count: usize,
    component_lookup_count: usize,
    source_reparse_count: usize,
    artifact_scan_count: usize,
}

impl WorthUiUserIntentTargetCounters {
    pub(in crate::runtime::user_intent_target) fn bound_with_page_slot_lookups(
        page_slot_lookup_count: usize,
    ) -> Self {
        Self {
            page_slot_lookup_count,
            page_slot_scan_count: 0,
            mounted_surface_lookup_count: 1,
            component_lookup_count: 1,
            source_reparse_count: 0,
            artifact_scan_count: 0,
        }
    }

    pub fn page_slot_lookup_count(self) -> usize {
        self.page_slot_lookup_count
    }

    pub fn page_slot_scan_count(self) -> usize {
        self.page_slot_scan_count
    }

    pub fn mounted_surface_lookup_count(self) -> usize {
        self.mounted_surface_lookup_count
    }

    pub fn component_lookup_count(self) -> usize {
        self.component_lookup_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn artifact_scan_count(self) -> usize {
        self.artifact_scan_count
    }
}
