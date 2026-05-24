use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput,
    ForgeQueryDomainEntryMarker,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "example.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[]
    }
}

struct SplitEdgeDeclaration;

impl ForgeQueryDeclarationInput<GeometryDomain> for SplitEdgeDeclaration {
    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        Vec::new()
    }
}

fn main() {}
