use worth_query::facade::{WORTHQueryCapabilityFamily, WORTHQueryDomainEntryMarker};

const HADWIGER_ENTRY_CAPABILITIES: &[WORTHQueryCapabilityFamily] = &[
    WORTHQueryCapabilityFamily::QueryComposition,
    WORTHQueryCapabilityFamily::WorkflowOrchestration,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HadwigerResearchDomainEntry;

impl WORTHQueryDomainEntryMarker for HadwigerResearchDomainEntry {
    fn domain_key(&self) -> &'static str {
        "WORTH.hadwiger.research"
    }

    fn display_name(&self) -> &'static str {
        "HadwigerResearchDomainEntry"
    }

    fn required_capability_families(&self) -> &'static [WORTHQueryCapabilityFamily] {
        HADWIGER_ENTRY_CAPABILITIES
    }
}
