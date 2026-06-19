use forge_query::facade::{ForgeQueryCapabilityFamily, ForgeQueryDomainEntryMarker};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionQueryDomain;

impl ForgeQueryDomainEntryMarker for PrimitiveConstructionQueryDomain {
    fn domain_key(&self) -> &'static str {
        "worth.kernel.primitive_construction"
    }

    fn display_name(&self) -> &'static str {
        "WorthKernelPrimitiveConstructionDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::QueryContext,
        ]
    }
}
