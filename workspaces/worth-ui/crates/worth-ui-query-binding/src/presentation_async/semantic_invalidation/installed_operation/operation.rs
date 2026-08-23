use worth_query::facade::{domain, read};

const CAPABILITIES: &[domain::WorthQueryCapabilityFamily] = &[
    domain::WorthQueryCapabilityFamily::QueryRead,
    domain::WorthQueryCapabilityFamily::QueryComposition,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPresentationAsyncDomainEntry;

impl domain::WorthQueryDomainEntryMarker for WorthUiPresentationAsyncDomainEntry {
    fn domain_key(&self) -> &'static str {
        "WORTH.ui.presentation-async"
    }

    fn display_name(&self) -> &'static str {
        "WorthUiPresentationAsyncDomainEntry"
    }

    fn required_capability_families(&self) -> &'static [domain::WorthQueryCapabilityFamily] {
        CAPABILITIES
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiPresentationAsyncOperation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiPresentationAsyncOperationFamily;

impl
    domain::WorthQueryExecutableDomainOperation<
        WorthUiPresentationAsyncDomainEntry,
        WorthUiPresentationAsyncOperationFamily,
    > for WorthUiPresentationAsyncOperation
{
    type Input = ();
    type Output = read::WorthQueryReadCompletion;
    type Publication = domain::WorthQueryPublishingOperation;
    type Execution = domain::WorthQueryDirectOperation;
}
