use forge_query::facade::{ForgeQueryCapabilityFamily, ForgeQueryDomainEntryMarker};

const HADWIGER_ENTRY_CAPABILITIES: &[ForgeQueryCapabilityFamily] = &[
    ForgeQueryCapabilityFamily::QueryComposition,
    ForgeQueryCapabilityFamily::WorkflowOrchestration,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HadwigerResearchDomainEntry;

impl ForgeQueryDomainEntryMarker for HadwigerResearchDomainEntry {
    fn domain_key(&self) -> &'static str {
        "forge.hadwiger.research"
    }

    fn display_name(&self) -> &'static str {
        "HadwigerResearchDomainEntry"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        HADWIGER_ENTRY_CAPABILITIES
    }
}
