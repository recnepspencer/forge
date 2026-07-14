use worth_query::facade::foundation::{WorthQueryCapabilityFamily, WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "example.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[]
    }
}

struct SplitEdgeDeclaration;

impl WorthQueryDeclarationInput<GeometryDomain> for SplitEdgeDeclaration {
    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        Vec::new()
    }
}

fn main() {}
