use std::marker::PhantomData;

use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryApplicationFacade,
    ForgeQueryAsyncDeclarationClause, ForgeQueryAsyncDeclarationSupport,
    ForgeQueryAsyncFailurePosture, ForgeQueryAsyncLoadingPosture,
    ForgeQueryAsyncRequestIdentityPart, ForgeQueryAsyncSourceFamily, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationFoundationalEvidenceInput,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityChecked,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationLegalityInput,
    ForgeQueryDeclarationRouteContract, ForgeQueryDeclarationRoutePlanInput,
    ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture, ForgeQueryTemporalDeclarationClause,
    ForgeQueryTemporalDeclarationSupport, ForgeQueryTemporalDuration,
};
use crate::runtime::ForgeQueryRuntimeFamilySupportStatus;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.route-plan"
    }

    fn display_name(&self) -> &'static str {
        "GeometryRoutePlanDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
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

impl ForgeQueryDomainOperatingContext<GeometryDomain> for GeometryWorld {
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
        format!("geometry.route-plan.{}", self.regime)
    }
}

macro_rules! define_family {
    ($name:ident, $contract:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(super) struct $name;

        impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for $name {
            type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
            type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
            type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

            fn semantic_family_key() -> &'static str {
                stringify!($name)
            }

            fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
                ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
            }

            fn route_contract() -> ForgeQueryDeclarationRouteContract {
                $contract
            }
        }
    };
}

define_family!(
    RelationalRouteFamily,
    ForgeQueryDeclarationRouteContract::relational_only()
);
define_family!(
    MixedRouteFamily,
    ForgeQueryDeclarationRouteContract::relational_and_bridge()
);
define_family!(
    RequiredIntentFamily,
    ForgeQueryDeclarationRouteContract::required_relational_intent()
);
define_family!(
    ForbiddenIntentFamily,
    ForgeQueryDeclarationRouteContract::relational_intent_forbidden()
);
define_family!(
    DeferredRouteFamily,
    ForgeQueryDeclarationRouteContract::deferred_auto()
);
define_family!(
    FailedRouteFamily,
    ForgeQueryDeclarationRouteContract::unresolved_mixed()
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AspectRichRouteFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct TemporalBridgeRouteFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AsyncBridgeRouteFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for AspectRichRouteFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AspectRichRouteFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.active_edge"],
            &["selection.local_topology"],
            &["selection.material_edit"],
            &["selection.private_authority"],
            &[],
        )
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_slices(
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

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct MissingAspectRouteFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for MissingAspectRouteFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "MissingAspectRouteFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.active_face"],
            &["selection.local_topology"],
            &[],
            &[],
            &[],
        )
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_slices(
            &["selection.active_edge", "selection.local_topology"],
            &[],
            &[],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ConflictAspectRouteFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for ConflictAspectRouteFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "ConflictAspectRouteFamily"
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.active_edge"],
            &["selection.local_topology"],
            &[],
            &[],
            &["selection.material_edit"],
        )
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_slices(
            &[
                "selection.active_edge",
                "selection.local_topology",
                "selection.material_edit",
            ],
            &[],
            &[],
        )
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
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
            impl ForgeQueryDeclarationInput<GeometryDomain> for RouteInput<$family> {
                type Family = $family;

                fn canonical_declaration_entries(
                    &self,
                ) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
                    vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
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

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for TemporalBridgeRouteFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "TemporalBridgeRouteFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn temporal_declaration_support() -> ForgeQueryTemporalDeclarationSupport {
        ForgeQueryTemporalDeclarationSupport::CanonicalIdentityOnly
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

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for AsyncBridgeRouteFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AsyncBridgeRouteFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn async_declaration_support() -> ForgeQueryAsyncDeclarationSupport {
        ForgeQueryAsyncDeclarationSupport::CanonicalIdentityOnly
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for RouteInput<TemporalBridgeRouteFamily> {
    type Family = TemporalBridgeRouteFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text(
            "edge_ref",
            self.edge_ref,
        )]
    }

    fn temporal_declaration_clauses(&self) -> Vec<ForgeQueryTemporalDeclarationClause> {
        vec![ForgeQueryTemporalDeclarationClause::stale_after(
            ForgeQueryTemporalDuration::seconds(30),
        )]
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for RouteInput<AsyncBridgeRouteFamily> {
    type Family = AsyncBridgeRouteFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text(
            "edge_ref",
            self.edge_ref,
        )]
    }

    fn async_resource_declaration_clauses(&self) -> Vec<ForgeQueryAsyncDeclarationClause> {
        vec![ForgeQueryAsyncDeclarationClause::resource_request(
            ForgeQueryAsyncSourceFamily::BridgeResource,
            ForgeQueryAsyncLoadingPosture::Blocking,
            ForgeQueryAsyncFailurePosture::FailClosed,
            vec![ForgeQueryAsyncRequestIdentityPart::text(
                "edge_ref",
                self.edge_ref,
            )],
        )]
    }
}

pub(super) fn admitted_handle(
    regime: &'static str,
) -> ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(GeometryWorld::named(regime))
        .validate()
        .expect("route-plan world should validate")
        .admit()
        .expect("route-plan world should admit")
}

pub(super) fn progressed<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld>,
    declaration: RouteInput<F>,
) -> crate::application::ForgeQueryAdmittedDeclarationProgression<GeometryDomain, RouteInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    RouteInput<F>: ForgeQueryDeclarationInput<GeometryDomain>,
{
    handle
        .declare_review_and_progress(declaration)
        .unwrap_or_else(|_| panic!("route-plan progression should admit"))
}

pub(super) fn route_input<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld>,
    declaration: RouteInput<F>,
) -> ForgeQueryDeclarationRoutePlanInput<GeometryDomain, RouteInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    RouteInput<F>: ForgeQueryDeclarationInput<GeometryDomain>,
{
    let progressed = progressed(handle, declaration);
    let evidence = handle
        .describe_foundational(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
        )
        .unwrap_or_else(|_| panic!("same-handle foundational evidence should materialize"));
    ForgeQueryDeclarationRoutePlanInput::admitted(progressed, evidence)
}

pub(super) fn future_supported_route_input<F>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld>,
    declaration: RouteInput<F>,
) -> ForgeQueryDeclarationRoutePlanInput<GeometryDomain, RouteInput<F>>
where
    F: ForgeQueryDeclarationFamilyMarker<GeometryDomain>,
    RouteInput<F>: ForgeQueryDeclarationInput<GeometryDomain, Family = F>,
{
    let canonical = handle
        .declare(declaration.clone())
        .unwrap_or_else(|_| panic!("future declaration should canonicalize"));
    let support_report = handle.family_support::<F>();
    let legal = match crate::application::review_declaration_legality(
        handle.handle_identity_digest(),
        ForgeQueryDeclarationLegalityInput::new(
            canonical,
            support_report,
            F::legality_contract(),
            handle.retained_world_basis(),
            Some(ForgeQueryRuntimeFamilySupportStatus::Supported),
            Some(ForgeQueryRuntimeFamilySupportStatus::Supported),
        ),
    ) {
        ForgeQueryDeclarationLegalityChecked::Legal(legal) => legal,
        ForgeQueryDeclarationLegalityChecked::Illegal(_) => {
            panic!("future declaration should become legal under supported runtime test posture")
        }
    };
    let progressed = handle
        .progress_declaration(legal)
        .unwrap_or_else(|_| panic!("future declaration progression should admit"));
    let evidence = handle
        .describe_foundational(
            ForgeQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
        )
        .unwrap_or_else(|_| panic!("future foundational evidence should materialize"));
    ForgeQueryDeclarationRoutePlanInput::admitted(progressed, evidence)
}
