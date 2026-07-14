use worth_query::facade::foundation::{WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily, WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract, WorthQueryDeclarationRouteContract, WorthQueryDescriptiveOnlyAuthority, WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext, WorthQueryNeighborhoodCapableGrouping, WorthQuerySignalCompatiblePosture};

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
        &[WorthQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[WorthQueryConfigSectionFamily::Query, WorthQueryConfigSectionFamily::Relational]
    }

    fn context_identity_digest(&self) -> String { "geometry.collaborative".to_string() }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DescribeOnlyFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for DescribeOnlyFamily {
    type PrimaryAuthority = WorthQueryDescriptiveOnlyAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str { "describe-only" }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_only()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DescribeOnlyDeclaration;

impl WorthQueryDeclarationInput<GeometryDomain> for DescribeOnlyDeclaration {
    type Family = DescribeOnlyFamily;

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
        .declare_review_and_progress(DescribeOnlyDeclaration)
        .unwrap_or_else(|_| panic!("progression should admit"));

    let _ = handle.route_relational_truth_from_progressed(progressed);
}
