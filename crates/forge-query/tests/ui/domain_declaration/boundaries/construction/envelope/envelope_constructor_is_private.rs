use forge_foundational::facade::CanonicalDerivedDigest;
use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationEnvelopeClass, ForgeQueryDeclarationEnvelopeEvidenceOrigin,
    ForgeQueryDeclarationEnvelopeExplanation, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationReceiptDenialCause, ForgeQueryDeclarationRouteContract,
    ForgeQueryDeclarationRoutePlanDenialCause, ForgeQueryDomainEntryMarker,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalCompatiblePosture,
    ForgeQuerySingleOnlyGrouping,
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

    let _ = ForgeQueryDeclarationEnvelope::<GeometryDomain, SplitEdgeDeclaration>::from_issued(
        fake(),
        fake::<CanonicalDerivedDigest>(),
        ForgeQueryDeclarationEnvelopeExplanation::new(
            "x",
            ForgeQueryDeclarationEnvelopeEvidenceOrigin::AdmittedProgression,
            None,
            Vec::new(),
            Some("x".to_string()),
            Some(ForgeQueryDeclarationRoutePlanDenialCause::NoAllowedRoutes),
            "x".to_string(),
            Some(ForgeQueryDeclarationReceiptDenialCause::MissingRoutePlan),
        ),
    );
    let _ = ForgeQueryDeclarationEnvelopeClass::CoveredCrossing;
}
