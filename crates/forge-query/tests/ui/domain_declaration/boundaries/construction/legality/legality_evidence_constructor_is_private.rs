use forge_foundational::facade::evaluate_boundary_surface_disposition_legality;
use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationLegalityEvidence,
    ForgeQueryDomainEntryMarker, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str { "example.geometry" }
    fn display_name(&self) -> &'static str { "GeometryDomain" }
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] { &[] }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SplitEdgeFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for SplitEdgeFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str { "split-edge" }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SplitEdgeDeclaration;

impl ForgeQueryDeclarationInput<GeometryDomain> for SplitEdgeDeclaration {
    type Family = SplitEdgeFamily;

    fn canonical_declaration_entries(
        &self,
    ) -> Vec<forge_query::facade::ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", "edge:42")]
    }
}

fn main() {
    fn fake<T>() -> T {
        panic!()
    }

    let _ = ForgeQueryDeclarationLegalityEvidence::<GeometryDomain, SplitEdgeDeclaration> {
        declaration: fake(),
        support_report: fake(),
        legality_contract: ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact(),
        role_claim_category: fake(),
        role_claim_role: fake(),
        surface_disposition: evaluate_boundary_surface_disposition_legality(
            fake(),
            fake(),
        )
        .unwrap(),
        legality_digest: String::new(),
    };
}
