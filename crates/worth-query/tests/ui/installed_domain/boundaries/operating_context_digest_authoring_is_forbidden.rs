use worth_query::facade::domain::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ConsumerDomain;

impl WorthQueryDomainEntryMarker for ConsumerDomain {
    fn domain_key(&self) -> &'static str {
        "WORTH.consumer.context-identity"
    }

    fn display_name(&self) -> &'static str {
        "Consumer Context Identity"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ConsumerContext;

impl WorthQueryDomainOperatingContext<ConsumerDomain> for ConsumerContext {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[]
    }

    fn context_identity_digest(&self) -> String {
        "consumer-authored-digest".to_owned()
    }
}

fn main() {}
