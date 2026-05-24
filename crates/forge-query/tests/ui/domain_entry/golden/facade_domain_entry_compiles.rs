use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryDomainEntryMarker,
};

const ENTRY_CAPABILITIES: &[ForgeQueryCapabilityFamily] = &[
    ForgeQueryCapabilityFamily::QueryComposition,
    ForgeQueryCapabilityFamily::QueryContext,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ExampleSpatialDomain;

impl ForgeQueryDomainEntryMarker for ExampleSpatialDomain {
    fn domain_key(&self) -> &'static str {
        "example.spatial"
    }

    fn display_name(&self) -> &'static str {
        "ExampleSpatialDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        ENTRY_CAPABILITIES
    }
}

fn main() {
    let query = ForgeQueryApplicationFacade::runtime_backed_default();
    let support = query.domain_entry_support_snapshot();
    let root = query.domain(ExampleSpatialDomain);
    let checked = query.domain_checked(ExampleSpatialDomain);
    let proof = query.domain_proof_root(ExampleSpatialDomain);

    let _ = support.snapshot_digest();
    let _ = root.domain_key();
    let _ = checked.support_snapshot();
    let _ = proof.display_name();
}
