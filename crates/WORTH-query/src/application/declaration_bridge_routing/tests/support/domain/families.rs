use std::marker::PhantomData;

use crate::application::{
    WorthQueryAsyncDeclarationClause, WorthQueryAsyncDeclarationSupport,
    WorthQueryAsyncFailurePosture, WorthQueryAsyncLoadingPosture,
    WorthQueryAsyncRequestIdentityPart, WorthQueryAsyncSourceFamily,
    WorthQueryBridgeContinuationAuthority, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationBridgeContinuationContract,
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract,
    WorthQueryDeclarationRouteContract, WorthQueryMixedAuthority,
    WorthQueryNeighborhoodCapableGrouping, WorthQueryRelationalTruthAuthority,
    WorthQuerySignalCompatiblePosture, WorthQueryTemporalDeclarationClause,
    WorthQueryTemporalDeclarationSupport, WorthQueryTemporalDuration,
};

use super::runtime::{bridge_aspect_contract, bridge_aspect_coverage, GeometryDomain};

macro_rules! define_family {
    ($name:ident, $route_contract:expr, $bridge_contract:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(crate) struct $name;

        impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for $name {
            type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
            type SignalCompatibility = WorthQuerySignalCompatiblePosture;
            type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

            fn semantic_family_key() -> &'static str {
                stringify!($name)
            }

            fn legality_contract() -> WorthQueryDeclarationLegalityContract {
                WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
            }

            fn route_contract() -> WorthQueryDeclarationRouteContract {
                $route_contract
            }

            fn bridge_continuation_contract(
            ) -> Option<WorthQueryDeclarationBridgeContinuationContract> {
                Some($bridge_contract)
            }

            fn aspect_contract() -> WorthQueryDeclarationAspectContract {
                bridge_aspect_contract()
            }

            fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
                bridge_aspect_coverage()
            }
        }
    };
}

define_family!(
    RuntimeRouteFamily,
    WorthQueryDeclarationRouteContract::bridge_only(),
    WorthQueryDeclarationBridgeContinuationContract::runtime_route_current()
);
define_family!(
    TruthViewCurrentFamily,
    WorthQueryDeclarationRouteContract::bridge_only(),
    WorthQueryDeclarationBridgeContinuationContract::truth_view_current()
);
define_family!(
    TruthViewHistoricalFamily,
    WorthQueryDeclarationRouteContract::bridge_only(),
    WorthQueryDeclarationBridgeContinuationContract::truth_view_historical()
);
define_family!(
    PreviewSessionFamily,
    WorthQueryDeclarationRouteContract::bridge_only(),
    WorthQueryDeclarationBridgeContinuationContract::preview_session()
);
define_family!(
    PreviewPromotionFamily,
    WorthQueryDeclarationRouteContract::bridge_only(),
    WorthQueryDeclarationBridgeContinuationContract::preview_promotion()
);
define_family!(
    SubscriptionPreparationFamily,
    WorthQueryDeclarationRouteContract::bridge_only(),
    WorthQueryDeclarationBridgeContinuationContract::subscription_preparation()
);
define_family!(
    WritebackPreparationFamily,
    WorthQueryDeclarationRouteContract::bridge_only(),
    WorthQueryDeclarationBridgeContinuationContract::writeback_preparation()
);
define_family!(
    SignalOnlyFamily,
    WorthQueryDeclarationRouteContract::signal_only(),
    WorthQueryDeclarationBridgeContinuationContract::runtime_route_current()
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MixedAuthorityFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for MixedAuthorityFamily {
    type PrimaryAuthority = WorthQueryMixedAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "MixedAuthorityFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_and_bridge()
    }

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(WorthQueryDeclarationBridgeContinuationContract::preview_session())
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(
            &["selection.active_face"],
            &[],
            &[],
            &[],
            &[],
        )
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        bridge_aspect_coverage()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RelationalOnlyFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for RelationalOnlyFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "RelationalOnlyFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_only()
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        bridge_aspect_contract()
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        bridge_aspect_coverage()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MissingAspectFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for MissingAspectFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "MissingAspectFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(
            WorthQueryDeclarationBridgeContinuationContract::preview_session()
                .with_required_aspects(WorthQueryDeclarationAspectContract::from_slices(
                    &["continuation.bridge.authority_slice"],
                    &[],
                    &[],
                    &[],
                    &[],
                )),
        )
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(
            &["selection.active_face"],
            &[],
            &[],
            &[],
            &[],
        )
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        WorthQueryDeclarationAspectCoverage::from_slices(&["selection.active_face"], &[], &[])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ConflictingAspectFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for ConflictingAspectFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "ConflictingAspectFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(
            WorthQueryDeclarationBridgeContinuationContract::preview_session()
                .with_required_aspects(WorthQueryDeclarationAspectContract::from_slices(
                    &["continuation.conflicting_preview"],
                    &[],
                    &[],
                    &[],
                    &[],
                )),
        )
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(
            &["selection.active_face"],
            &[],
            &[],
            &[],
            &[],
        )
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        WorthQueryDeclarationAspectCoverage::from_slices(
            &["selection.active_face", "continuation.conflicting_preview"],
            &[],
            &["continuation.conflicting_preview"],
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ExpandedAspectFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for ExpandedAspectFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "ExpandedAspectFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(WorthQueryDeclarationBridgeContinuationContract::preview_session())
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(
            &["selection.active_face", "continuation.preview_ready"],
            &[
                "selection.neighborhood.local_topology",
                "continuation.preview_material",
            ],
            &["continuation.preview.surface"],
            &["continuation.private_branch"],
            &["continuation.conflicting_preview"],
        )
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        WorthQueryDeclarationAspectCoverage::from_slices(
            &[
                "selection.active_face",
                "continuation.preview_ready",
                "selection.neighborhood.local_topology",
                "continuation.bridge.audit_lane",
                "continuation.preview_material",
                "continuation.preview.surface",
            ],
            &[],
            &[],
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct RoutingInput<F> {
    edge_ref: &'static str,
    _marker: PhantomData<F>,
}

impl<F> RoutingInput<F> {
    pub(crate) fn new(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            _marker: PhantomData,
        }
    }
}

macro_rules! impl_declaration_input {
    ($($family:ty),+ $(,)?) => {
        $(
            impl WorthQueryDeclarationInput<GeometryDomain> for RoutingInput<$family> {
                type Family = $family;

                fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
                    vec![WorthQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
                }
            }
        )+
    };
}

impl_declaration_input!(
    RuntimeRouteFamily,
    TruthViewCurrentFamily,
    TruthViewHistoricalFamily,
    PreviewSessionFamily,
    PreviewPromotionFamily,
    SubscriptionPreparationFamily,
    WritebackPreparationFamily,
    MixedAuthorityFamily,
    SignalOnlyFamily,
    RelationalOnlyFamily,
    MissingAspectFamily,
    ConflictingAspectFamily,
    ExpandedAspectFamily,
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TemporalRuntimeRouteFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for TemporalRuntimeRouteFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "TemporalRuntimeRouteFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(WorthQueryDeclarationBridgeContinuationContract::runtime_route_current())
    }

    fn temporal_declaration_support() -> WorthQueryTemporalDeclarationSupport {
        WorthQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        bridge_aspect_contract()
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        bridge_aspect_coverage()
    }
}

impl<F> Clone for RoutingInput<F> {
    fn clone(&self) -> Self {
        Self {
            edge_ref: self.edge_ref,
            _marker: PhantomData,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AsyncRuntimeRouteFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AsyncRuntimeRouteFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AsyncRuntimeRouteFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(WorthQueryDeclarationBridgeContinuationContract::runtime_route_current())
    }

    fn async_declaration_support() -> WorthQueryAsyncDeclarationSupport {
        WorthQueryAsyncDeclarationSupport::CanonicalIdentityOnly
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        bridge_aspect_contract()
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        bridge_aspect_coverage()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TemporalSignalOnlyFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for TemporalSignalOnlyFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "TemporalSignalOnlyFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::signal_only()
    }

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(WorthQueryDeclarationBridgeContinuationContract::runtime_route_current())
    }

    fn temporal_declaration_support() -> WorthQueryTemporalDeclarationSupport {
        WorthQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        bridge_aspect_contract()
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        bridge_aspect_coverage()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AsyncSignalOnlyFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AsyncSignalOnlyFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AsyncSignalOnlyFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::signal_only()
    }

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(WorthQueryDeclarationBridgeContinuationContract::runtime_route_current())
    }

    fn async_declaration_support() -> WorthQueryAsyncDeclarationSupport {
        WorthQueryAsyncDeclarationSupport::CanonicalIdentityOnly
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        bridge_aspect_contract()
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        bridge_aspect_coverage()
    }
}

impl WorthQueryDeclarationInput<GeometryDomain> for RoutingInput<TemporalRuntimeRouteFamily> {
    type Family = TemporalRuntimeRouteFamily;

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

impl WorthQueryDeclarationInput<GeometryDomain> for RoutingInput<AsyncRuntimeRouteFamily> {
    type Family = AsyncRuntimeRouteFamily;

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

impl WorthQueryDeclarationInput<GeometryDomain> for RoutingInput<TemporalSignalOnlyFamily> {
    type Family = TemporalSignalOnlyFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text(
            "edge_ref",
            self.edge_ref,
        )]
    }

    fn temporal_declaration_clauses(&self) -> Vec<WorthQueryTemporalDeclarationClause> {
        vec![WorthQueryTemporalDeclarationClause::interval(
            WorthQueryTemporalDuration::seconds(15),
        )]
    }
}

impl WorthQueryDeclarationInput<GeometryDomain> for RoutingInput<AsyncSignalOnlyFamily> {
    type Family = AsyncSignalOnlyFamily;

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
