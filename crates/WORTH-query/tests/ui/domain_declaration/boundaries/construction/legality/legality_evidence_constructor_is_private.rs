use worth_foundational::facade::evaluate_boundary_surface_disposition_legality;
use worth_query::facade::{
    WorthQueryAdmittedWorldBasis, WorthQueryCapabilityFamily, WorthQueryDeclarationCanonicalEntry,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput,
    WorthQueryDeclarationLegalityContract, WorthQueryDeclarationLegalityEvidence,
    WorthQueryDomainEntryMarker, WorthQueryNeighborhoodCapableGrouping,
    WorthQueryRelationalTruthAuthority, WorthQuerySignalCompatiblePosture,
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
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

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
        vec![WorthQueryDeclarationCanonicalEntry::text("edge_ref", "edge:42")]
    }
}

fn main() {
    fn fake<T>() -> T {
        panic!()
    }

    let _ = WorthQueryDeclarationLegalityEvidence::<GeometryDomain, SplitEdgeDeclaration> {
        declaration: fake(),
        support_report: fake(),
        legality_contract: WorthQueryDeclarationLegalityContract::authoritative_hot_artifact(),
        role_claim_category: fake(),
        role_claim_role: fake(),
        surface_disposition: evaluate_boundary_surface_disposition_legality(
            fake(),
            fake(),
        )
        .unwrap(),
        world_basis: unsafe { std::mem::zeroed::<WorthQueryAdmittedWorldBasis>() },
        legality_digest: String::new(),
    };
}
