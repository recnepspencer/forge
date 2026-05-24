use forge_foundational::facade::CanonicalEquivalenceBasis;
use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalCompatiblePosture,
    ForgeQuerySingleOnlyGrouping,
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
struct SplitEdgeDeclaration(&'static str);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SplitEdgeFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for SplitEdgeFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for SplitEdgeDeclaration {
    type Family = SplitEdgeFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.0)]
    }
}

fn main() {
    let query = ForgeQueryApplicationFacade::runtime_backed_default();
    let handle = query
        .domain(GeometryDomain)
        .with_operating_context(GeometryOperatingContext)
        .validate()
        .unwrap()
        .admit()
        .unwrap();

    let left = handle.declare(SplitEdgeDeclaration("edge:42")).unwrap();
    let right = SplitEdgeDeclaration("edge:42");

    let _ = left.compare_under(&right, CanonicalEquivalenceBasis::ExactCanonicalBasis);
}
