use super::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};
use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfig,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCapabilityStatus,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclaredFamilyChecked,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalConfig,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.capability"
    }

    fn display_name(&self) -> &'static str {
        "GeometryCapabilityDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct QueryOnlyWorld;

impl ForgeQueryDomainOperatingContext<GeometryDomain> for QueryOnlyWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryContext]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[ForgeQueryConfigSectionFamily::Query]
    }

    fn context_identity_digest(&self) -> String {
        "query-only-world".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DurableFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for DurableFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "durable-family"
    }

    fn required_capability_families() -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::DurableArtifacts]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DurableDeclaration;

impl ForgeQueryDeclarationInput<GeometryDomain> for DurableDeclaration {
    type Family = DurableFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text(
            "edge_ref", "edge:42",
        )]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct HistoricalFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for HistoricalFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "historical-family"
    }

    fn required_capability_families() -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections() -> &'static [ForgeQueryConfigSectionFamily] {
        &[ForgeQueryConfigSectionFamily::Relational]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct HistoricalDeclaration;

impl ForgeQueryDeclarationInput<GeometryDomain> for HistoricalDeclaration {
    type Family = HistoricalFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text(
            "edge_ref", "edge:99",
        )]
    }
}

fn admitted_query_only_handle(
    config: ForgeQueryConfig,
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, QueryOnlyWorld> {
    ForgeQueryApplicationFacade::new(config)
        .expect("config should construct a facade")
        .domain(GeometryDomain)
        .with_operating_context(QueryOnlyWorld)
        .validate()
        .expect("query-only context should validate")
        .admit()
        .expect("query-only context should admit")
}

#[test]
fn family_support_and_checked_declaration_agree_on_deferred_denial() {
    let handle = admitted_query_only_handle(ForgeQueryConfig::runtime_backed_default());
    let support = handle.family_support::<DurableFamily>();
    assert_eq!(
        support.declare_status(),
        ForgeQueryDeclarationCapabilityStatus::DeferredDebt
    );

    match handle.declare_checked(DurableDeclaration) {
        ForgeQueryDeclaredFamilyChecked::Deferred(denial) => {
            assert_eq!(
                denial.capability_status(),
                ForgeQueryDeclarationCapabilityStatus::DeferredDebt
            );
        }
        other => panic!(
            "expected deferred denial, got {:?}",
            std::mem::discriminant(&other)
        ),
    }
}

#[test]
fn family_support_and_checked_declaration_agree_on_invalid_context_denial() {
    let handle = admitted_query_only_handle(
        ForgeQueryConfig::runtime_backed_default()
            .with_relational(ForgeQueryRelationalConfig::disabled()),
    );
    let support = handle.family_support::<HistoricalFamily>();
    assert_eq!(
        support.declare_status(),
        ForgeQueryDeclarationCapabilityStatus::InvalidContext
    );

    match handle.declare_checked(HistoricalDeclaration) {
        ForgeQueryDeclaredFamilyChecked::InvalidContext(denial) => {
            assert_eq!(
                denial.capability_status(),
                ForgeQueryDeclarationCapabilityStatus::InvalidContext
            );
        }
        other => panic!(
            "expected invalid-context denial, got {:?}",
            std::mem::discriminant(&other)
        ),
    }
}
