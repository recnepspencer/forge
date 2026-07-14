use worth_query::facade::foundation::{WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily, WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract, WorthQueryDeclarationProgressionContract, WorthQueryDeclarationRouteContract, WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext, WorthQueryNeighborhoodCapableGrouping, WorthQueryRelationalTruthAuthority, WorthQuerySignalNotCompatiblePosture};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str { "example.geometry.orchestration" }
    fn display_name(&self) -> &'static str { "GeometryOrchestrationDomain" }
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryOperatingContext;

impl WorthQueryDomainOperatingContext<GeometryDomain> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[
            WorthQueryCapabilityFamily::HistoricalEvaluation,
            WorthQueryCapabilityFamily::QueryComposition,
        ]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        "geometry.orchestration".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TrimSegmentFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for TrimSegmentFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalNotCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str { "trim-segment-at-intersection" }
    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_only()
    }
    fn progression_contract(
        _handle_identity_digest: &str,
        _operating_context_identity_digest: &str,
    ) -> WorthQueryDeclarationProgressionContract {
        WorthQueryDeclarationProgressionContract::admitted_current()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TrimSegmentAtIntersection {
    segment_ref: &'static str,
    intersection_ref: &'static str,
}

impl WorthQueryDeclarationInput<GeometryDomain> for TrimSegmentAtIntersection {
    type Family = TrimSegmentFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            WorthQueryDeclarationCanonicalEntry::text("segment_ref", self.segment_ref),
            WorthQueryDeclarationCanonicalEntry::text(
                "intersection_ref",
                self.intersection_ref,
            ),
        ]
    }
}

fn accepts_orchestration_surface(
    handle: &worth_query::facade::foundation::WorthQueryAdmittedConfiguredDomainHandle<
        GeometryDomain,
        GeometryOperatingContext,
    >,
) {
    let _ = handle.orchestrate_declaration_entry(TrimSegmentAtIntersection {
        segment_ref: "segment:outer-wall-a",
        intersection_ref: "intersection:trim-42",
    });
}

fn main() {}
