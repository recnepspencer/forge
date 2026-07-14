use worth_query::facade::foundation::{WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily, WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract, WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext, WorthQueryNeighborhoodCapableGrouping, WorthQueryRelationalTruthAuthority, WorthQuerySignalCompatiblePosture};

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
struct TrimSegmentFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for TrimSegmentFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str { "trim-segment" }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TrimSegmentAtIntersection;

impl WorthQueryDeclarationInput<GeometryDomain> for TrimSegmentAtIntersection {
    type Family = TrimSegmentFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            WorthQueryDeclarationCanonicalEntry::text("segment_ref", "segment:outer-wall-a"),
            WorthQueryDeclarationCanonicalEntry::text("intersection_ref", "intersection:trim-42"),
        ]
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

    let _ = handle.orchestrate_trim_segment(TrimSegmentAtIntersection);
}
