use worth_query::facade::foundation::{WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily, WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationFoundationalEvidenceInput, WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract, WorthQueryDeclarationRouteContract, WorthQueryDeclarationRoutePlanInput, WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext, WorthQueryNeighborhoodCapableGrouping, WorthQueryRelationalTruthAuthority, WorthQuerySignalCompatiblePosture};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str { "example.geometry" }
    fn display_name(&self) -> &'static str { "GeometryDomain" }
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryOperatingContext;

impl WorthQueryDomainOperatingContext<GeometryDomain> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::Relational,
            WorthQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String { "geometry.signal".to_string() }
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

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_only()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SplitEdgeDeclaration;

impl WorthQueryDeclarationInput<GeometryDomain> for SplitEdgeDeclaration {
    type Family = SplitEdgeFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text("edge_ref", "edge:42")]
    }
}

fn main() {
    let query = WorthQueryApplicationFacade::runtime_backed_default();
    let handle = query
        .domain(GeometryDomain)
        .with_operating_context(GeometryOperatingContext)
        .validate()
        .unwrap()
        .admit()
        .unwrap();
    let progressed = handle
        .declare_review_and_progress(SplitEdgeDeclaration)
        .unwrap_or_else(|_| panic!("progression should admit"));
    let evidence = handle
        .describe_foundational(
            WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(progressed.clone()),
        )
        .unwrap_or_else(|_| panic!("evidence should admit"));
    let route_plan = handle
        .plan_routes(WorthQueryDeclarationRoutePlanInput::admitted(progressed, evidence))
        .unwrap_or_else(|_| panic!("route planning should admit"));

    let _ = handle.signal_compatibility(route_plan);
}
