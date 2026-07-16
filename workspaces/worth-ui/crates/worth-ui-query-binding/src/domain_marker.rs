use worth_query::facade::domain::{WorthQueryCapabilityFamily, WorthQueryDomainEntryMarker};

const WORTH_UI_ENTRY_CAPABILITIES: &[WorthQueryCapabilityFamily] = &[
    WorthQueryCapabilityFamily::QueryComposition,
    WorthQueryCapabilityFamily::WorkflowOrchestration,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiDomainEntry;

impl WorthQueryDomainEntryMarker for WorthUiDomainEntry {
    fn domain_key(&self) -> &'static str {
        "WORTH.ui.runtime"
    }

    fn display_name(&self) -> &'static str {
        "WorthUiDomainEntry"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        WORTH_UI_ENTRY_CAPABILITIES
    }
}
