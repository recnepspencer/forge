use worth_foundational::facade::CanonicalDerivedDigest;
use worth_query::facade::{
    WorthQueryCapabilityFamily, WorthQueryDeclarationEnvelope,
    WorthQueryDeclarationEnvelopeClass, WorthQueryDeclarationEnvelopeEvidenceOrigin,
    WorthQueryDeclarationEnvelopeExplanation, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract,
    WorthQueryDeclarationReceiptDenialCause, WorthQueryDeclarationRouteContract,
    WorthQueryDeclarationRoutePlanDenialCause, WorthQueryDomainEntryMarker,
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
    ) -> Vec<worth_query::facade::WorthQueryDeclarationCanonicalEntry> {
        Vec::new()
    }
}

fn main() {
    fn fake<T>() -> T { panic!() }

    let _ = WorthQueryDeclarationEnvelope::<GeometryDomain, SplitEdgeDeclaration>::from_issued(
        fake(),
        fake::<CanonicalDerivedDigest>(),
        WorthQueryDeclarationEnvelopeExplanation::new(
            "x",
            WorthQueryDeclarationEnvelopeEvidenceOrigin::AdmittedProgression,
            None,
            Vec::new(),
            Some("x".to_string()),
            Some(WorthQueryDeclarationRoutePlanDenialCause::NoAllowedRoutes),
            "x".to_string(),
            Some(WorthQueryDeclarationReceiptDenialCause::MissingRoutePlan),
        ),
    );
    let _ = WorthQueryDeclarationEnvelopeClass::CoveredCrossing;
}
