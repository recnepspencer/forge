use std::marker::PhantomData;

use forge_foundational::facade::{
    FoundationalBoundaryArtifactCategory, FoundationalBoundaryArtifactRole,
    FoundationalBoundaryAvailability, FoundationalBoundaryDeliveryClass,
};

use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryBridgeContinuationAuthority, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDeclarationBridgeContinuationContract,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityClass,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalCompatiblePosture,
    ForgeQueryTemporalDeclarationClause, ForgeQueryTemporalDeclarationSupport,
    ForgeQueryTemporalDuration,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.legality"
    }

    fn display_name(&self) -> &'static str {
        "GeometryLegalityDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
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

impl ForgeQueryDomainOperatingContext<GeometryDomain> for CollaborativeWorld {
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
        format!("geometry.{}", self.regime)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct LegalFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for LegalFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.active_edge"],
            &["selection.active_edge"],
            &["selection.active_edge"],
            &[],
            &["selection.material_edit"],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct IllegalRoleFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for IllegalRoleFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::new(
            ForgeQueryDeclarationLegalityClass::AuthoritativeHotArtifact,
            FoundationalBoundaryArtifactCategory::Summary,
            FoundationalBoundaryArtifactRole::AuthoritativeCurrent,
            FoundationalBoundaryDeliveryClass::MustBeHot,
            FoundationalBoundaryAvailability::Present,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct IllegalDispositionFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for IllegalDispositionFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::new(
            ForgeQueryDeclarationLegalityClass::AuthoritativeHotArtifact,
            FoundationalBoundaryArtifactCategory::Artifact,
            FoundationalBoundaryArtifactRole::AuthoritativeCurrent,
            FoundationalBoundaryDeliveryClass::MustBeHot,
            FoundationalBoundaryAvailability::Deferred,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct DeferredLegalityFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for DeferredLegalityFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::deferred_boundary()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct DurableAdmissionFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for DurableAdmissionFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "split-edge"
    }

    fn required_capability_families() -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::DurableArtifacts]
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct MaskedCoverageFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for MaskedCoverageFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "masked-coverage"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.active_edge"],
            &["selection.active_edge"],
            &["selection.active_edge"],
            &[],
            &[],
        )
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_slices(
            &["selection.active_edge"],
            &["selection.active_edge"],
            &[],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
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
            impl ForgeQueryDeclarationInput<GeometryDomain> for Declaration<$family> {
                type Family = $family;

                fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
                    vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
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

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for TemporalCurrentFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "temporal-current"
    }

    fn temporal_declaration_support() -> ForgeQueryTemporalDeclarationSupport {
        ForgeQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn bridge_continuation_contract() -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
        Some(ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct TemporalPreviewFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for TemporalPreviewFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "temporal-preview"
    }

    fn temporal_declaration_support() -> ForgeQueryTemporalDeclarationSupport {
        ForgeQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn bridge_continuation_contract() -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
        Some(ForgeQueryDeclarationBridgeContinuationContract::preview_session())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct TemporalHistoricalFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for TemporalHistoricalFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "temporal-historical"
    }

    fn temporal_declaration_support() -> ForgeQueryTemporalDeclarationSupport {
        ForgeQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn bridge_continuation_contract() -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
        Some(ForgeQueryDeclarationBridgeContinuationContract::truth_view_historical())
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
            impl ForgeQueryDeclarationInput<GeometryDomain> for TemporalDeclaration<$family> {
                type Family = $family;

                fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
                    vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
                }

                fn temporal_declaration_clauses(&self) -> Vec<ForgeQueryTemporalDeclarationClause> {
                    vec![ForgeQueryTemporalDeclarationClause::stale_after(
                        ForgeQueryTemporalDuration::seconds(30),
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
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, CollaborativeWorld>
{
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(CollaborativeWorld::named(regime))
        .validate()
        .expect("context should validate")
        .admit()
        .expect("context should admit")
}
