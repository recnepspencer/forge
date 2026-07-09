use worth_query::facade::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryDomainEntryMarker,
};

const ENTRY_CAPABILITIES: &[WorthQueryCapabilityFamily] = &[
    WorthQueryCapabilityFamily::QueryComposition,
    WorthQueryCapabilityFamily::QueryContext,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleSpatialDomain;

impl WorthQueryDomainEntryMarker for ExampleSpatialDomain {
    fn domain_key(&self) -> &'static str {
        "example.spatial"
    }

    fn display_name(&self) -> &'static str {
        "ExampleSpatialDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        ENTRY_CAPABILITIES
    }
}

fn main() {
    let query = WorthQueryApplicationFacade::runtime_backed_default();
    let support = query.domain_entry_support_snapshot();
    let root = query.domain(ExampleSpatialDomain);
    let checked = query.domain_checked(ExampleSpatialDomain);
    let proof = query.domain_proof_root(ExampleSpatialDomain);

    let _ = support.snapshot_digest();
    let _ = root.domain_key();
    let _ = checked.support_snapshot();
    let _ = proof.display_name();
}
