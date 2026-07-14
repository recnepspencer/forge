use super::{WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationInput};
use crate::application::{
    assert_declaration_aspect_projections, WorthQueryApplicationFacade, WorthQueryCapabilityFamily,
    WorthQueryConfig, WorthQueryConfigSectionFamily, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationAspectFit,
    WorthQueryDeclarationCapabilityStatus, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationLegalityContract, WorthQueryDeclaredFamilyChecked,
    WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
    WorthQueryNeighborhoodCapableGrouping, WorthQueryRelationalConfig,
    WorthQueryRelationalTruthAuthority, WorthQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.capability"
    }

    fn display_name(&self) -> &'static str {
        "GeometryCapabilityDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct QueryOnlyWorld;

impl WorthQueryDomainOperatingContext<GeometryDomain> for QueryOnlyWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryContext]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[WorthQueryConfigSectionFamily::Query]
    }

    fn context_identity_digest(&self) -> String {
        "query-only-world".to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DurableFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for DurableFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "durable-family"
    }

    fn required_capability_families() -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::DurableArtifacts]
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(
            &["selection.active_edge"],
            &["selection.active_edge"],
            &["selection.active_edge"],
            &[],
            &["selection.material_edit"],
        )
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DurableDeclaration;

impl WorthQueryDeclarationInput<GeometryDomain> for DurableDeclaration {
    type Family = DurableFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text(
            "edge_ref", "edge:42",
        )]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct HistoricalFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for HistoricalFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "historical-family"
    }

    fn required_capability_families() -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections() -> &'static [WorthQueryConfigSectionFamily] {
        &[WorthQueryConfigSectionFamily::Relational]
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct HistoricalDeclaration;

impl WorthQueryDeclarationInput<GeometryDomain> for HistoricalDeclaration {
    type Family = HistoricalFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text(
            "edge_ref", "edge:99",
        )]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MaskedCoverageFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for MaskedCoverageFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "masked-coverage-family"
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(
            &["selection.active_edge"],
            &["selection.active_edge"],
            &["selection.active_edge"],
            &[],
            &[],
        )
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        WorthQueryDeclarationAspectCoverage::from_slices(
            &["selection.active_edge"],
            &["selection.active_edge"],
            &[],
        )
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

fn admitted_query_only_handle(
    _config: WorthQueryConfig,
) -> crate::application::WorthQueryInstalledDomainDeclarationContext<GeometryDomain, QueryOnlyWorld>
{
    crate::application::domain_test_support::installed_declaration_context(
        GeometryDomain,
        QueryOnlyWorld,
        [
            crate::application::domain_test_support::family::<GeometryDomain, DurableFamily>(),
            crate::application::domain_test_support::family::<GeometryDomain, HistoricalFamily>(),
            crate::application::domain_test_support::family::<GeometryDomain, MaskedCoverageFamily>(
            ),
        ],
    )
}

#[test]
fn family_support_and_checked_declaration_agree_on_deferred_denial() {
    let handle = admitted_query_only_handle(WorthQueryConfig::runtime_backed_default());
    let support = handle.family_support::<DurableFamily>();
    assert_eq!(
        support.declare_status(),
        WorthQueryDeclarationCapabilityStatus::DeferredDebt
    );

    match handle.declare_checked(DurableDeclaration) {
        WorthQueryDeclaredFamilyChecked::Deferred(denial) => {
            assert_eq!(
                denial.capability_status(),
                WorthQueryDeclarationCapabilityStatus::DeferredDebt
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
        WorthQueryConfig::runtime_backed_default()
            .with_relational(WorthQueryRelationalConfig::disabled()),
    );
    let support = handle.family_support::<HistoricalFamily>();
    assert_eq!(
        support.declare_status(),
        WorthQueryDeclarationCapabilityStatus::InvalidContext
    );

    match handle.declare_checked(HistoricalDeclaration) {
        WorthQueryDeclaredFamilyChecked::InvalidContext(denial) => {
            assert_eq!(
                denial.capability_status(),
                WorthQueryDeclarationCapabilityStatus::InvalidContext
            );
        }
        other => panic!(
            "expected invalid-context denial, got {:?}",
            std::mem::discriminant(&other)
        ),
    }
}

#[test]
fn family_support_report_exposes_aspect_contract_alongside_family_admission() {
    let handle = admitted_query_only_handle(WorthQueryConfig::runtime_backed_default());
    let support = handle.family_support::<DurableFamily>();

    assert_declaration_aspect_projections(
        support.aspect_contract().required(),
        &["selection.active_edge"],
    );
    assert_declaration_aspect_projections(
        support.aspect_contract().incompatible(),
        &["selection.material_edit"],
    );
    assert_eq!(
        support
            .row(crate::application::WorthQueryDeclarationCapabilityVerb::Declare)
            .expect("declare row should exist")
            .aspect_fit(),
        WorthQueryDeclarationAspectFit::Exact
    );
}

#[test]
fn family_support_report_can_expose_masked_semantic_slices_without_losing_family_admission() {
    let handle = admitted_query_only_handle(WorthQueryConfig::runtime_backed_default());
    let support = handle.family_support::<MaskedCoverageFamily>();

    assert_eq!(
        support.declare_status(),
        WorthQueryDeclarationCapabilityStatus::Admitted
    );
    assert_declaration_aspect_projections(
        support.aspect_coverage().masked(),
        &["selection.active_edge"],
    );
    assert_eq!(
        support
            .row(crate::application::WorthQueryDeclarationCapabilityVerb::Declare)
            .expect("declare row should exist")
            .aspect_fit(),
        WorthQueryDeclarationAspectFit::MissingRequired
    );
}
