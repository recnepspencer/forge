use worth_foundational::facade::{
    FoundationalBoundaryEvidenceCompletedReceiptArtifact, FoundationalBoundaryReceiptSurface,
    FoundationalMaterializedBoundaryArtifact,
};
use worth_query::facade::foundation::{WorthQueryCapabilityFamily, WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract, WorthQueryDeclarationReceipt, WorthQueryDeclarationReceiptClass, WorthQueryDeclarationReceiptExplanation, WorthQueryDeclarationReceiptKind, WorthQueryDeclarationRouteContract, WorthQueryDomainEntryMarker, WorthQueryRelationalTruthAuthority, WorthQuerySignalCompatiblePosture, WorthQuerySingleOnlyGrouping};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str { "example.geometry" }
    fn display_name(&self) -> &'static str { "GeometryDomain" }
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] { &[] }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SplitEdgeDeclaration;

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

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_only()
    }
}

impl WorthQueryDeclarationInput<GeometryDomain> for SplitEdgeDeclaration {
    type Family = SplitEdgeFamily;

    fn canonical_declaration_entries(
        &self,
    ) -> Vec<worth_query::facade::foundation::WorthQueryDeclarationCanonicalEntry> {
        Vec::new()
    }
}

fn main() {
    fn fake<T>() -> T { panic!() }

    let _ = WorthQueryDeclarationReceipt::<GeometryDomain, SplitEdgeDeclaration>::new(
        WorthQueryDeclarationReceiptClass::CoveredCrossing,
        WorthQueryDeclarationReceiptKind::Relational,
        None,
        Some(fake()),
        None,
        WorthQueryDeclarationReceiptExplanation::new("x", None, Vec::new(), "x".to_string()),
        None::<FoundationalBoundaryEvidenceCompletedReceiptArtifact>,
        fake::<FoundationalMaterializedBoundaryArtifact<FoundationalBoundaryReceiptSurface>>(),
        fake(),
    );
}
