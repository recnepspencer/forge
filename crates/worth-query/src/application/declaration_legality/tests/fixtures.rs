use std::marker::PhantomData;

use worth_foundational::facade::{
    FoundationalBoundaryArtifactCategory, FoundationalBoundaryArtifactRole,
    FoundationalBoundaryAvailability, FoundationalBoundaryDeliveryClass,
};

use crate::application::{
    WorthQueryApplicationFacade, WorthQueryBridgeContinuationAuthority, WorthQueryCapabilityFamily,
    WorthQueryConfigSectionFamily, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationBridgeContinuationContract,
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityClass,
    WorthQueryDeclarationLegalityContract, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext, WorthQueryNeighborhoodCapableGrouping,
    WorthQueryRelationalTruthAuthority, WorthQuerySignalCompatiblePosture,
    WorthQueryTemporalDeclarationClause, WorthQueryTemporalDeclarationSupport,
    WorthQueryTemporalDuration,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.legality"
    }

    fn display_name(&self) -> &'static str {
        "GeometryLegalityDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct CollaborativeWorld {
    regime: &'static str,
}

impl CollaborativeWorld {
    fn named(regime: &'static str) -> Self {
        Self { regime }
    }
}

impl WorthQueryDomainOperatingContext<GeometryDomain> for CollaborativeWorld {
    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::HistoricalEvaluation]
    }

    fn required_config_sections(&self) -> &'static [WorthQueryConfigSectionFamily] {
        &[
            WorthQueryConfigSectionFamily::Query,
            WorthQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("geometry.{}", self.regime)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct LegalFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for LegalFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct IllegalRoleFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for IllegalRoleFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::new(
            WorthQueryDeclarationLegalityClass::AuthoritativeHotArtifact,
            FoundationalBoundaryArtifactCategory::Summary,
            FoundationalBoundaryArtifactRole::AuthoritativeCurrent,
            FoundationalBoundaryDeliveryClass::MustBeHot,
            FoundationalBoundaryAvailability::Present,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct IllegalDispositionFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for IllegalDispositionFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::new(
            WorthQueryDeclarationLegalityClass::AuthoritativeHotArtifact,
            FoundationalBoundaryArtifactCategory::Artifact,
            FoundationalBoundaryArtifactRole::AuthoritativeCurrent,
            FoundationalBoundaryDeliveryClass::MustBeHot,
            FoundationalBoundaryAvailability::Deferred,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct DeferredLegalityFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for DeferredLegalityFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::deferred_boundary()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct DurableAdmissionFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for DurableAdmissionFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn required_capability_families() -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::DurableArtifacts]
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct MaskedCoverageFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for MaskedCoverageFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "masked-coverage"
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Declaration<F> {
    edge_ref: &'static str,
    _family: PhantomData<F>,
}

impl<F> Declaration<F> {
    pub(super) fn new(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            _family: PhantomData,
        }
    }
}

macro_rules! impl_declaration_input {
    ($($family:ty),+ $(,)?) => {
        $(
            impl WorthQueryDeclarationInput<GeometryDomain> for Declaration<$family> {
                type Family = $family;

                fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
                    vec![WorthQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
                }
            }
        )+
    };
}

impl_declaration_input!(
    LegalFamily,
    IllegalRoleFamily,
    IllegalDispositionFamily,
    DeferredLegalityFamily,
    DurableAdmissionFamily,
    MaskedCoverageFamily,
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct TemporalCurrentFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for TemporalCurrentFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "temporal-current"
    }

    fn temporal_declaration_support() -> WorthQueryTemporalDeclarationSupport {
        WorthQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(WorthQueryDeclarationBridgeContinuationContract::runtime_route_current())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct TemporalPreviewFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for TemporalPreviewFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "temporal-preview"
    }

    fn temporal_declaration_support() -> WorthQueryTemporalDeclarationSupport {
        WorthQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(WorthQueryDeclarationBridgeContinuationContract::preview_session())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct TemporalHistoricalFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for TemporalHistoricalFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "temporal-historical"
    }

    fn temporal_declaration_support() -> WorthQueryTemporalDeclarationSupport {
        WorthQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(WorthQueryDeclarationBridgeContinuationContract::truth_view_historical())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct TemporalDeclaration<F> {
    edge_ref: &'static str,
    _family: PhantomData<F>,
}

impl<F> TemporalDeclaration<F> {
    pub(super) fn new(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            _family: PhantomData,
        }
    }
}

macro_rules! impl_temporal_input {
    ($($family:ty),+ $(,)?) => {
        $(
            impl WorthQueryDeclarationInput<GeometryDomain> for TemporalDeclaration<$family> {
                type Family = $family;

                fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
                    vec![WorthQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
                }

                fn temporal_declaration_clauses(&self) -> Vec<WorthQueryTemporalDeclarationClause> {
                    vec![WorthQueryTemporalDeclarationClause::stale_after(
                        WorthQueryTemporalDuration::seconds(30),
                    )]
                }
            }
        )+
    };
}

impl_temporal_input!(
    TemporalCurrentFamily,
    TemporalPreviewFamily,
    TemporalHistoricalFamily,
);

pub(super) fn admitted_handle(
    regime: &'static str,
) -> crate::application::WorthQueryInstalledDomainDeclarationContext<
    GeometryDomain,
    CollaborativeWorld,
> {
    WorthQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(CollaborativeWorld::named(regime))
        .validate()
        .expect("context should validate")
        .admit()
        .expect("context should admit")
}
