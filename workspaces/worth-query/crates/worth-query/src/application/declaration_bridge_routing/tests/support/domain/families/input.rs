use std::marker::PhantomData;

use crate::application::{
    WorthQueryAsyncDeclarationClause, WorthQueryAsyncDeclarationSupport,
    WorthQueryAsyncFailurePosture, WorthQueryAsyncLoadingPosture,
    WorthQueryAsyncRequestIdentityPart, WorthQueryAsyncSourceFamily,
    WorthQueryBridgeContinuationAuthority, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationBridgeContinuationContract,
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract,
    WorthQueryDeclarationRouteContract, WorthQueryNeighborhoodCapableGrouping,
    WorthQuerySignalCompatiblePosture, WorthQueryTemporalDeclarationClause,
    WorthQueryTemporalDeclarationSupport, WorthQueryTemporalDuration,
};

use super::super::runtime::{bridge_aspect_contract, bridge_aspect_coverage, GeometryDomain};

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
    super::bridge_markers::RuntimeRouteFamily,
    super::bridge_markers::TruthViewCurrentFamily,
    super::bridge_markers::TruthViewHistoricalFamily,
    super::bridge_markers::PreviewSessionFamily,
    super::bridge_markers::PreviewPromotionFamily,
    super::bridge_markers::SubscriptionPreparationFamily,
    super::bridge_markers::WritebackPreparationFamily,
    super::bridge_markers::MixedAuthorityFamily,
    super::bridge_markers::SignalOnlyFamily,
    super::bridge_markers::RelationalOnlyFamily,
    super::bridge_markers::MissingAspectFamily,
    super::bridge_markers::ConflictingAspectFamily,
    super::bridge_markers::ExpandedAspectFamily,
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
