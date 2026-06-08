use std::marker::PhantomData;

use crate::application::{
    ForgeQueryAsyncDeclarationClause, ForgeQueryAsyncDeclarationSupport,
    ForgeQueryAsyncFailurePosture, ForgeQueryAsyncLoadingPosture,
    ForgeQueryAsyncRequestIdentityPart, ForgeQueryAsyncSourceFamily,
    ForgeQueryBridgeContinuationAuthority, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDeclarationBridgeContinuationContract,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryMixedAuthority,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture, ForgeQueryTemporalDeclarationClause,
    ForgeQueryTemporalDeclarationSupport, ForgeQueryTemporalDuration,
};

use super::runtime::{bridge_aspect_contract, bridge_aspect_coverage, GeometryDomain};

macro_rules! define_family {
    ($name:ident, $route_contract:expr, $bridge_contract:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(crate) struct $name;

        impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for $name {
            type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
            type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
            type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

            fn semantic_family_key() -> &'static str {
                stringify!($name)
            }

            fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
                ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
            }

            fn route_contract() -> ForgeQueryDeclarationRouteContract {
                $route_contract
            }

            fn bridge_continuation_contract(
            ) -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
                Some($bridge_contract)
            }

            fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
                bridge_aspect_contract()
            }

            fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
                bridge_aspect_coverage()
            }
        }
    };
}

define_family!(
    RuntimeRouteFamily,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current()
);
define_family!(
    TruthViewCurrentFamily,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    ForgeQueryDeclarationBridgeContinuationContract::truth_view_current()
);
define_family!(
    TruthViewHistoricalFamily,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    ForgeQueryDeclarationBridgeContinuationContract::truth_view_historical()
);
define_family!(
    PreviewSessionFamily,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    ForgeQueryDeclarationBridgeContinuationContract::preview_session()
);
define_family!(
    PreviewPromotionFamily,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    ForgeQueryDeclarationBridgeContinuationContract::preview_promotion()
);
define_family!(
    SubscriptionPreparationFamily,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    ForgeQueryDeclarationBridgeContinuationContract::subscription_preparation()
);
define_family!(
    WritebackPreparationFamily,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    ForgeQueryDeclarationBridgeContinuationContract::writeback_preparation()
);
define_family!(
    SignalOnlyFamily,
    ForgeQueryDeclarationRouteContract::signal_only(),
    ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current()
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MixedAuthorityFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for MixedAuthorityFamily {
    type PrimaryAuthority = ForgeQueryMixedAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "MixedAuthorityFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_and_bridge()
    }

    fn bridge_continuation_contract() -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
        Some(ForgeQueryDeclarationBridgeContinuationContract::preview_session())
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.active_face"],
            &[],
            &[],
            &[],
            &[],
        )
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        bridge_aspect_coverage()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RelationalOnlyFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for RelationalOnlyFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "RelationalOnlyFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        bridge_aspect_contract()
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        bridge_aspect_coverage()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MissingAspectFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for MissingAspectFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "MissingAspectFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn bridge_continuation_contract() -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
        Some(
            ForgeQueryDeclarationBridgeContinuationContract::preview_session()
                .with_required_aspects(ForgeQueryDeclarationAspectContract::from_slices(
                    &["continuation.bridge.authority_slice"],
                    &[],
                    &[],
                    &[],
                    &[],
                )),
        )
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.active_face"],
            &[],
            &[],
            &[],
            &[],
        )
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_slices(&["selection.active_face"], &[], &[])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ConflictingAspectFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for ConflictingAspectFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "ConflictingAspectFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn bridge_continuation_contract() -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
        Some(
            ForgeQueryDeclarationBridgeContinuationContract::preview_session()
                .with_required_aspects(ForgeQueryDeclarationAspectContract::from_slices(
                    &["continuation.conflicting_preview"],
                    &[],
                    &[],
                    &[],
                    &[],
                )),
        )
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.active_face"],
            &[],
            &[],
            &[],
            &[],
        )
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_slices(
            &["selection.active_face", "continuation.conflicting_preview"],
            &[],
            &["continuation.conflicting_preview"],
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ExpandedAspectFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for ExpandedAspectFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "ExpandedAspectFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn bridge_continuation_contract() -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
        Some(ForgeQueryDeclarationBridgeContinuationContract::preview_session())
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
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

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_slices(
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
            impl ForgeQueryDeclarationInput<GeometryDomain> for RoutingInput<$family> {
                type Family = $family;

                fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
                    vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
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

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for TemporalRuntimeRouteFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "TemporalRuntimeRouteFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn bridge_continuation_contract() -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
        Some(ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current())
    }

    fn temporal_declaration_support() -> ForgeQueryTemporalDeclarationSupport {
        ForgeQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        bridge_aspect_contract()
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
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

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for AsyncRuntimeRouteFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AsyncRuntimeRouteFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn bridge_continuation_contract() -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
        Some(ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current())
    }

    fn async_declaration_support() -> ForgeQueryAsyncDeclarationSupport {
        ForgeQueryAsyncDeclarationSupport::CanonicalIdentityOnly
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        bridge_aspect_contract()
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        bridge_aspect_coverage()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TemporalSignalOnlyFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for TemporalSignalOnlyFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "TemporalSignalOnlyFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::signal_only()
    }

    fn bridge_continuation_contract() -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
        Some(ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current())
    }

    fn temporal_declaration_support() -> ForgeQueryTemporalDeclarationSupport {
        ForgeQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        bridge_aspect_contract()
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        bridge_aspect_coverage()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AsyncSignalOnlyFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for AsyncSignalOnlyFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AsyncSignalOnlyFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::signal_only()
    }

    fn bridge_continuation_contract() -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
        Some(ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current())
    }

    fn async_declaration_support() -> ForgeQueryAsyncDeclarationSupport {
        ForgeQueryAsyncDeclarationSupport::CanonicalIdentityOnly
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        bridge_aspect_contract()
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        bridge_aspect_coverage()
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for RoutingInput<TemporalRuntimeRouteFamily> {
    type Family = TemporalRuntimeRouteFamily;

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

impl ForgeQueryDeclarationInput<GeometryDomain> for RoutingInput<AsyncRuntimeRouteFamily> {
    type Family = AsyncRuntimeRouteFamily;

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

impl ForgeQueryDeclarationInput<GeometryDomain> for RoutingInput<TemporalSignalOnlyFamily> {
    type Family = TemporalSignalOnlyFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text(
            "edge_ref",
            self.edge_ref,
        )]
    }

    fn temporal_declaration_clauses(&self) -> Vec<ForgeQueryTemporalDeclarationClause> {
        vec![ForgeQueryTemporalDeclarationClause::interval(
            ForgeQueryTemporalDuration::seconds(15),
        )]
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for RoutingInput<AsyncSignalOnlyFamily> {
    type Family = AsyncSignalOnlyFamily;

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
