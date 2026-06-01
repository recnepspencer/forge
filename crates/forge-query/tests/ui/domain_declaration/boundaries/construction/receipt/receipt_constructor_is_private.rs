use forge_foundational::facade::{
    FoundationalBoundaryEvidenceCompletedReceiptArtifact, FoundationalBoundaryReceiptSurface,
    FoundationalMaterializedBoundaryArtifact,
};
use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationReceipt, ForgeQueryDeclarationReceiptClass,
    ForgeQueryDeclarationReceiptExplanation, ForgeQueryDeclarationReceiptKind,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str { "example.geometry" }
    fn display_name(&self) -> &'static str { "GeometryDomain" }
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] { &[] }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SplitEdgeDeclaration;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SplitEdgeFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for SplitEdgeFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str { "split-edge" }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for SplitEdgeDeclaration {
    type Family = SplitEdgeFamily;

    fn canonical_declaration_entries(
        &self,
    ) -> Vec<forge_query::facade::ForgeQueryDeclarationCanonicalEntry> {
        Vec::new()
    }
}

fn main() {
    fn fake<T>() -> T { panic!() }

    let _ = ForgeQueryDeclarationReceipt::<GeometryDomain, SplitEdgeDeclaration>::new(
        ForgeQueryDeclarationReceiptClass::CoveredCrossing,
        ForgeQueryDeclarationReceiptKind::Relational,
        None,
        Some(fake()),
        None,
        ForgeQueryDeclarationReceiptExplanation::new("x", None, Vec::new(), "x".to_string()),
        None::<FoundationalBoundaryEvidenceCompletedReceiptArtifact>,
        fake::<FoundationalMaterializedBoundaryArtifact<FoundationalBoundaryReceiptSurface>>(),
        fake(),
    );
}
