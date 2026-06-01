use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryBridgeContinuationAuthority, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDeclarationSignalCompatibilityContract,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQuerySignalCompatiblePosture,
};
use crate::family_helpers::{
    ForgeQueryGeometryActiveFaceSelectionHelperFamily, ForgeQueryGeometryNeighborhoodHelperFamily,
};

static COUNTING_GEOMETRY_CANONICALIZATION_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.grouped_authoring.geometry"
    }

    fn display_name(&self) -> &'static str {
        "GroupedAuthoringGeometryDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct GeometryWorld(pub(super) &'static str);

impl ForgeQueryDomainOperatingContext<GeometryDomain> for GeometryWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::PreviewSession]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
            ForgeQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("grouped-authoring-world-{}", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct GeometryFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for GeometryFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "GeometryGroupedFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.active_face"],
            &["selection.material_preview"],
            &[],
            &[],
            &[],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_and_bridge()
    }

    fn bridge_continuation_contract(
    ) -> Option<crate::application::ForgeQueryDeclarationBridgeContinuationContract> {
        Some(crate::application::ForgeQueryDeclarationBridgeContinuationContract::preview_session())
    }

    fn signal_compatibility_contract() -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
        Some(ForgeQueryDeclarationSignalCompatibilityContract::runtime_derived_execution())
    }
}

impl ForgeQueryGeometryActiveFaceSelectionHelperFamily<GeometryDomain> for GeometryFamily {}
impl ForgeQueryGeometryNeighborhoodHelperFamily<GeometryDomain> for GeometryFamily {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct RequiredIntentGeometryFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for RequiredIntentGeometryFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "GeometryRequiredIntentGroupedFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.active_face"],
            &["selection.material_preview"],
            &[],
            &[],
            &[],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::required_relational_intent()
    }

    fn bridge_continuation_contract(
    ) -> Option<crate::application::ForgeQueryDeclarationBridgeContinuationContract> {
        Some(crate::application::ForgeQueryDeclarationBridgeContinuationContract::preview_session())
    }

    fn signal_compatibility_contract() -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
        Some(ForgeQueryDeclarationSignalCompatibilityContract::runtime_derived_execution())
    }
}

impl ForgeQueryGeometryNeighborhoodHelperFamily<GeometryDomain> for RequiredIntentGeometryFamily {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct GeometryInput {
    id: &'static str,
    _marker: PhantomData<GeometryFamily>,
}

impl GeometryInput {
    pub(super) fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for GeometryInput {
    type Family = GeometryFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        if self.id.is_empty() {
            Vec::new()
        } else {
            vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct CountingGeometryInput {
    id: &'static str,
    _marker: PhantomData<GeometryFamily>,
}

impl CountingGeometryInput {
    pub(super) fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for CountingGeometryInput {
    type Family = GeometryFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        COUNTING_GEOMETRY_CANONICALIZATION_COUNT.fetch_add(1, Ordering::SeqCst);
        if self.id.is_empty() {
            Vec::new()
        } else {
            vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct RequiredIntentGeometryInput {
    id: &'static str,
    _marker: PhantomData<RequiredIntentGeometryFamily>,
}

impl RequiredIntentGeometryInput {
    pub(super) fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for RequiredIntentGeometryInput {
    type Family = RequiredIntentGeometryFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        if self.id.is_empty() {
            Vec::new()
        } else {
            vec![ForgeQueryDeclarationCanonicalEntry::text("id", self.id)]
        }
    }
}

pub(super) fn admitted_handle(
    world: &'static str,
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(GeometryWorld(world))
        .validate()
        .unwrap()
        .admit()
        .unwrap()
}

pub(super) fn reset_counting_geometry_canonicalization_count() {
    COUNTING_GEOMETRY_CANONICALIZATION_COUNT.store(0, Ordering::SeqCst);
}

pub(super) fn counting_geometry_canonicalization_count() -> usize {
    COUNTING_GEOMETRY_CANONICALIZATION_COUNT.load(Ordering::SeqCst)
}
