use forge_query::facade::{ForgeQueryCapabilityFamily, ForgeQueryDeclarationFamilyMarker, ForgeQueryDomainEntryMarker};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str { "example.geometry" }
    fn display_name(&self) -> &'static str { "GeometryDomain" }
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] { &[] }
}

struct SplitEdgeFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for SplitEdgeFamily {
    fn semantic_family_key() -> &'static str {
        "split-edge"
    }
}

fn main() {}
