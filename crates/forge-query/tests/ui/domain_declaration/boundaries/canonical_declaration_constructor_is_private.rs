use forge_query::facade::{
    ForgeQueryCanonicalDeclarationArtifact, ForgeQueryCapabilityFamily, ForgeQueryDeclarationInput,
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct SplitEdgeDeclaration;

impl ForgeQueryDeclarationInput<GeometryDomain> for SplitEdgeDeclaration {
    fn declaration_family(&self) -> &'static str {
        "split-edge"
    }

    fn canonical_entries(
        &self,
    ) -> Vec<forge_query::facade::ForgeQueryDeclarationCanonicalEntry> {
        Vec::new()
    }
}

fn main() {
    fn fake<T>() -> T {
        panic!()
    }

    let _ = ForgeQueryCanonicalDeclarationArtifact::<GeometryDomain, SplitEdgeDeclaration> {
        handle_identity_digest: String::new(),
        declaration_family: "split-edge",
        input: SplitEdgeDeclaration,
        canonical_entries: Vec::new(),
        declaration_digest: fake(),
        version: fake(),
        _marker: std::marker::PhantomData,
    };
}
