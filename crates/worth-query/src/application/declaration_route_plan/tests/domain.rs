use std::marker::PhantomData;

use crate::application::{
    WorthQueryApplicationFacade, WorthQueryAsyncDeclarationClause,
    WorthQueryAsyncDeclarationSupport, WorthQueryAsyncFailurePosture,
    WorthQueryAsyncLoadingPosture, WorthQueryAsyncRequestIdentityPart, WorthQueryAsyncSourceFamily,
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationCanonicalEntry,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationFoundationalEvidenceInput,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityChecked,
    WorthQueryDeclarationLegalityContract, WorthQueryDeclarationLegalityInput,
    WorthQueryDeclarationRouteContract, WorthQueryDeclarationRoutePlanInput,
    WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
    WorthQueryInstalledDomainDeclarationContext, WorthQueryNeighborhoodCapableGrouping,
    WorthQueryRelationalTruthAuthority, WorthQuerySignalCompatiblePosture,
    WorthQueryTemporalDeclarationClause, WorthQueryTemporalDeclarationSupport,
    WorthQueryTemporalDuration,
};
use crate::runtime::WorthQueryRuntimeFamilySupportStatus;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.route-plan"
    }

    fn display_name(&self) -> &'static str {
        "GeometryRoutePlanDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct GeometryWorld {
    regime: &'static str,
}

impl GeometryWorld {
    pub(super) fn named(regime: &'static str) -> Self {
        Self { regime }
    }
}

impl WorthQueryDomainOperatingContext<GeometryDomain> for GeometryWorld {
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
        format!("geometry.route-plan.{}", self.regime)
    }
}

macro_rules! define_family {
    ($name:ident, $contract:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(super) struct $name;

        impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for $name {
            type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
            type SignalCompatibility = WorthQuerySignalCompatiblePosture;
            type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

            fn semantic_family_key() -> &'static str {
                stringify!($name)
            }

            fn legality_contract() -> WorthQueryDeclarationLegalityContract {
                WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
            }

            fn route_contract() -> WorthQueryDeclarationRouteContract {
                $contract
            }
        }
    };
}

define_family!(
    RelationalRouteFamily,
    WorthQueryDeclarationRouteContract::relational_only()
);
define_family!(
    MixedRouteFamily,
    WorthQueryDeclarationRouteContract::relational_and_bridge()
);
define_family!(
    RequiredIntentFamily,
    WorthQueryDeclarationRouteContract::required_relational_intent()
);
define_family!(
    ForbiddenIntentFamily,
    WorthQueryDeclarationRouteContract::relational_intent_forbidden()
);
define_family!(
    DeferredRouteFamily,
    WorthQueryDeclarationRouteContract::deferred_auto()
);
define_family!(
    FailedRouteFamily,
    WorthQueryDeclarationRouteContract::unresolved_mixed()
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AspectRichRouteFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct TemporalBridgeRouteFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AsyncBridgeRouteFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AspectRichRouteFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AspectRichRouteFamily"
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(
            &["selection.active_edge"],
            &["selection.local_topology"],
            &["selection.material_edit"],
            &["selection.private_authority"],
            &[],
        )
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        WorthQueryDeclarationAspectCoverage::from_slices(
            &[
                "selection.active_edge",
                "selection.local_topology",
                "selection.material_edit",
                "selection.private_authority",
            ],
            &["selection.private_authority"],
            &[],
        )
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_only()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct MissingAspectRouteFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for MissingAspectRouteFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "MissingAspectRouteFamily"
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(
            &["selection.active_face"],
            &["selection.local_topology"],
            &[],
            &[],
            &[],
        )
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        WorthQueryDeclarationAspectCoverage::from_slices(
            &["selection.active_edge", "selection.local_topology"],
            &[],
            &[],
        )
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_only()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ConflictAspectRouteFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for ConflictAspectRouteFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "ConflictAspectRouteFamily"
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(
            &["selection.active_edge"],
            &["selection.local_topology"],
            &[],
            &[],
            &["selection.material_edit"],
        )
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        WorthQueryDeclarationAspectCoverage::from_slices(
            &[
                "selection.active_edge",
                "selection.local_topology",
                "selection.material_edit",
            ],
            &[],
            &[],
        )
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_only()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(super) struct RouteInput<F> {
    edge_ref: &'static str,
    _marker: PhantomData<F>,
}

impl<F> RouteInput<F> {
    pub(super) fn new(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            _marker: PhantomData,
        }
    }
}

macro_rules! impl_declaration_input {
    ($($family:ty),+ $(,)?) => {
        $(
            impl WorthQueryDeclarationInput<GeometryDomain> for RouteInput<$family> {
                type Family = $family;

                fn canonical_declaration_entries(
                    &self,
                ) -> Vec<WorthQueryDeclarationCanonicalEntry> {
                    vec![WorthQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
                }
            }
        )+
    };
}

impl_declaration_input!(
    RelationalRouteFamily,
    MixedRouteFamily,
    RequiredIntentFamily,
    ForbiddenIntentFamily,
    DeferredRouteFamily,
    FailedRouteFamily,
    AspectRichRouteFamily,
    MissingAspectRouteFamily,
    ConflictAspectRouteFamily,
);

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for TemporalBridgeRouteFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "TemporalBridgeRouteFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn temporal_declaration_support() -> WorthQueryTemporalDeclarationSupport {
        WorthQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }
}

impl<F> Clone for RouteInput<F> {
    fn clone(&self) -> Self {
        Self {
            edge_ref: self.edge_ref,
            _marker: PhantomData,
        }
    }
}

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AsyncBridgeRouteFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AsyncBridgeRouteFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn async_declaration_support() -> WorthQueryAsyncDeclarationSupport {
        WorthQueryAsyncDeclarationSupport::CanonicalIdentityOnly
    }
}

impl WorthQueryDeclarationInput<GeometryDomain> for RouteInput<TemporalBridgeRouteFamily> {
    type Family = TemporalBridgeRouteFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text(
            "edge_ref",
            self.edge_ref,
        )]
    }

    fn temporal_declaration_clauses(&self) -> Vec<WorthQueryTemporalDeclarationClause> {
        vec![WorthQueryTemporalDeclarationClause::stale_after(
            WorthQueryTemporalDuration::seconds(30),
        )]
    }
}

impl WorthQueryDeclarationInput<GeometryDomain> for RouteInput<AsyncBridgeRouteFamily> {
    type Family = AsyncBridgeRouteFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text(
            "edge_ref",
            self.edge_ref,
        )]
    }

    fn async_resource_declaration_clauses(&self) -> Vec<WorthQueryAsyncDeclarationClause> {
        vec![WorthQueryAsyncDeclarationClause::resource_request(
            WorthQueryAsyncSourceFamily::BridgeResource,
            WorthQueryAsyncLoadingPosture::Blocking,
            WorthQueryAsyncFailurePosture::FailClosed,
            vec![WorthQueryAsyncRequestIdentityPart::text(
                "edge_ref",
                self.edge_ref,
            )],
        )]
    }
}

pub(super) fn admitted_handle(
    regime: &'static str,
) -> WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld> {
    crate::application::domain_test_support::installed_declaration_context(
        GeometryDomain,
        GeometryWorld::named(regime),
        [
            crate::application::domain_test_support::family::<GeometryDomain, RelationalRouteFamily>(
            ),
            crate::application::domain_test_support::family::<GeometryDomain, MixedRouteFamily>(),
            crate::application::domain_test_support::family::<GeometryDomain, RequiredIntentFamily>(
            ),
            crate::application::domain_test_support::family::<GeometryDomain, ForbiddenIntentFamily>(
            ),
            crate::application::domain_test_support::family::<GeometryDomain, DeferredRouteFamily>(
            ),
            crate::application::domain_test_support::family::<GeometryDomain, FailedRouteFamily>(),
            crate::application::domain_test_support::family::<GeometryDomain, AspectRichRouteFamily>(
            ),
            crate::application::domain_test_support::family::<
                GeometryDomain,
                MissingAspectRouteFamily,
            >(),
            crate::application::domain_test_support::family::<
                GeometryDomain,
                ConflictAspectRouteFamily,
            >(),
            crate::application::domain_test_support::family::<
                GeometryDomain,
                TemporalBridgeRouteFamily,
            >(),
            crate::application::domain_test_support::family::<GeometryDomain, AsyncBridgeRouteFamily>(
            ),
        ],
    )
}

pub(super) fn progressed<F>(
    handle: &WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld>,
    declaration: RouteInput<F>,
) -> crate::application::WorthQueryAdmittedDeclarationProgression<GeometryDomain, RouteInput<F>>
where
    F: WorthQueryDeclarationFamilyMarker<GeometryDomain>,
    RouteInput<F>: WorthQueryDeclarationInput<GeometryDomain>,
{
    handle
        .declare_review_and_progress(declaration)
        .unwrap_or_else(|_| panic!("route-plan progression should admit"))
}

pub(super) fn route_input<F>(
    handle: &WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld>,
    declaration: RouteInput<F>,
) -> WorthQueryDeclarationRoutePlanInput<GeometryDomain, RouteInput<F>>
where
    F: WorthQueryDeclarationFamilyMarker<GeometryDomain>,
    RouteInput<F>: WorthQueryDeclarationInput<GeometryDomain>,
{
    let progressed = progressed(handle, declaration);
    let evidence = handle
        .describe_foundational(
            WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
        )
        .unwrap_or_else(|_| panic!("same-handle foundational evidence should materialize"));
    WorthQueryDeclarationRoutePlanInput::admitted(progressed, evidence)
}

pub(super) fn future_supported_route_input<F>(
    handle: &WorthQueryInstalledDomainDeclarationContext<GeometryDomain, GeometryWorld>,
    declaration: RouteInput<F>,
) -> WorthQueryDeclarationRoutePlanInput<GeometryDomain, RouteInput<F>>
where
    F: WorthQueryDeclarationFamilyMarker<GeometryDomain>,
    RouteInput<F>: WorthQueryDeclarationInput<GeometryDomain, Family = F>,
{
    let canonical = handle
        .declare(declaration.clone())
        .unwrap_or_else(|_| panic!("future declaration should canonicalize"));
    let support_report = handle.family_support::<F>();
    let legal = match crate::application::review_declaration_legality(
        handle.handle_identity_digest(),
        WorthQueryDeclarationLegalityInput::new(
            canonical,
            support_report,
            F::legality_contract(),
            handle.retained_world_basis(),
            Some(WorthQueryRuntimeFamilySupportStatus::Supported),
            Some(WorthQueryRuntimeFamilySupportStatus::Supported),
        ),
    ) {
        WorthQueryDeclarationLegalityChecked::Legal(legal) => legal,
        WorthQueryDeclarationLegalityChecked::Illegal(_) => {
            panic!("future declaration should become legal under supported runtime test posture")
        }
    };
    let progressed = handle
        .progress_declaration(legal)
        .unwrap_or_else(|_| panic!("future declaration progression should admit"));
    let evidence = handle
        .describe_foundational(
            WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
        )
        .unwrap_or_else(|_| panic!("future foundational evidence should materialize"));
    WorthQueryDeclarationRoutePlanInput::admitted(progressed, evidence)
}
