use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationFamilyTaxonomy, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationPrimaryAuthorityFamily, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryGroupedDeclarationPosture,
    ForgeQuerySignalCompatibilityPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "example.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GeometryDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryOperatingContext;

impl ForgeQueryDomainOperatingContext<GeometryDomain> for GeometryOperatingContext {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        "geometry.collaborative".to_string()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SplitEdgeDeclaration;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SplitEdgeFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for SplitEdgeFamily {
    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn taxonomy() -> ForgeQueryDeclarationFamilyTaxonomy {
        ForgeQueryDeclarationFamilyTaxonomy::new(
            ForgeQueryDeclarationPrimaryAuthorityFamily::RelationalTruth,
            ForgeQuerySignalCompatibilityPosture::Compatible,
            ForgeQueryGroupedDeclarationPosture::SingleOnly,
        )
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for SplitEdgeDeclaration {
    type Family = SplitEdgeFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", "edge:42")]
    }
}

fn main() {
    let query = ForgeQueryApplicationFacade::runtime_backed_default();
    let validated = query
        .domain(GeometryDomain)
        .with_operating_context(GeometryOperatingContext)
        .validate()
        .unwrap();

    let _ = validated.declare(SplitEdgeDeclaration);
}
