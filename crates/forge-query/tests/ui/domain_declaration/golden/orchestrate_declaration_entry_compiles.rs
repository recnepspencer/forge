use forge_query::facade::{
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationProgressionContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str { "example.geometry.orchestration" }
    fn display_name(&self) -> &'static str { "GeometryOrchestrationDomain" }
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryOperatingContext;

impl ForgeQueryDomainOperatingContext<GeometryDomain> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
            ForgeQueryCapabilityFamily::QueryComposition,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        "geometry.orchestration".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TrimSegmentFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for TrimSegmentFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str { "trim-segment-at-intersection" }
    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }
    fn progression_contract(
        _handle_identity_digest: &str,
        _operating_context_identity_digest: &str,
    ) -> ForgeQueryDeclarationProgressionContract {
        ForgeQueryDeclarationProgressionContract::admitted_current()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TrimSegmentAtIntersection {
    segment_ref: &'static str,
    intersection_ref: &'static str,
}

impl ForgeQueryDeclarationInput<GeometryDomain> for TrimSegmentAtIntersection {
    type Family = TrimSegmentFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            ForgeQueryDeclarationCanonicalEntry::text("segment_ref", self.segment_ref),
            ForgeQueryDeclarationCanonicalEntry::text(
                "intersection_ref",
                self.intersection_ref,
            ),
        ]
    }
}

fn accepts_orchestration_surface(
    handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
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
