use worth_query::facade::{
    WorthQueryCapabilityFamily, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationFoundationalEvidence, WorthQueryDeclarationInput,
    WorthQueryDeclarationLegalityContract, WorthQueryDomainEntryMarker,
    WorthQueryRelationalTruthAuthority, WorthQuerySignalCompatiblePosture,
    WorthQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str { "example.geometry" }
    fn display_name(&self) -> &'static str { "GeometryDomain" }
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] { &[] }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SplitEdgeFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for SplitEdgeFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str { "split-edge" }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SplitEdgeDeclaration;

impl WorthQueryDeclarationInput<GeometryDomain> for SplitEdgeDeclaration {
    type Family = SplitEdgeFamily;

    fn canonical_declaration_entries(
        &self,
    ) -> Vec<worth_query::facade::WorthQueryDeclarationCanonicalEntry> {
        Vec::new()
    }
}

fn main() {
    let _ctor = WorthQueryDeclarationFoundationalEvidence::<GeometryDomain, SplitEdgeDeclaration>::new;
}
