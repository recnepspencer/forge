use worth_query::facade::domain::{WorthQueryCapabilityFamily, WorthQueryDomainEntryMarker};

const HADWIGER_ENTRY_CAPABILITIES: &[WorthQueryCapabilityFamily] = &[
    WorthQueryCapabilityFamily::QueryComposition,
    WorthQueryCapabilityFamily::WorkflowOrchestration,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HadwigerResearchDomainEntry;

impl WorthQueryDomainEntryMarker for HadwigerResearchDomainEntry {
    fn domain_key(&self) -> &'static str {
        "WORTH.hadwiger.research"
    }

    fn display_name(&self) -> &'static str {
        "HadwigerResearchDomainEntry"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        HADWIGER_ENTRY_CAPABILITIES
    }
}
