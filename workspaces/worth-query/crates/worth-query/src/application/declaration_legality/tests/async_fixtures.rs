use std::marker::PhantomData;

use crate::application::{
    WorthQueryAsyncDeclarationClause, WorthQueryAsyncDeclarationSupport,
    WorthQueryAsyncFailurePosture, WorthQueryAsyncLoadingPosture,
    WorthQueryAsyncRequestIdentityPart, WorthQueryAsyncSourceFamily,
    WorthQueryBridgeContinuationAuthority, WorthQueryDeclarationBridgeContinuationContract,
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract,
    WorthQueryNeighborhoodCapableGrouping, WorthQuerySignalCompatiblePosture,
};

use super::fixtures::GeometryDomain;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AsyncCurrentFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AsyncCurrentFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "async-current"
    }

    fn async_declaration_support() -> WorthQueryAsyncDeclarationSupport {
        WorthQueryAsyncDeclarationSupport::CanonicalIdentityOnly
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(WorthQueryDeclarationBridgeContinuationContract::runtime_route_current())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AsyncPreviewFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AsyncPreviewFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "async-preview"
    }

    fn async_declaration_support() -> WorthQueryAsyncDeclarationSupport {
        WorthQueryAsyncDeclarationSupport::CanonicalIdentityOnly
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(WorthQueryDeclarationBridgeContinuationContract::preview_session())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AsyncHistoricalFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AsyncHistoricalFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "async-historical"
    }

    fn async_declaration_support() -> WorthQueryAsyncDeclarationSupport {
        WorthQueryAsyncDeclarationSupport::CanonicalIdentityOnly
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(WorthQueryDeclarationBridgeContinuationContract::truth_view_historical())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct AsyncDeclaration<F> {
    edge_ref: &'static str,
    async_clauses: Vec<WorthQueryAsyncDeclarationClause>,
    _family: PhantomData<F>,
}

impl<F> AsyncDeclaration<F> {
    pub(super) fn bridge_blocking(edge_ref: &'static str) -> Self {
        Self::resource_request(
            edge_ref,
            WorthQueryAsyncSourceFamily::BridgeResource,
            WorthQueryAsyncLoadingPosture::Blocking,
            WorthQueryAsyncFailurePosture::FailClosed,
        )
    }

    pub(super) fn external_blocking(edge_ref: &'static str) -> Self {
        Self::resource_request(
            edge_ref,
            WorthQueryAsyncSourceFamily::ExternalResource,
            WorthQueryAsyncLoadingPosture::Blocking,
            WorthQueryAsyncFailurePosture::FailClosed,
        )
    }

    pub(super) fn bridge_refresh(edge_ref: &'static str) -> Self {
        Self::resource_request(
            edge_ref,
            WorthQueryAsyncSourceFamily::BridgeResource,
            WorthQueryAsyncLoadingPosture::BackgroundRefresh,
            WorthQueryAsyncFailurePosture::FailClosed,
        )
    }

    pub(super) fn bridge_retain_stale(edge_ref: &'static str) -> Self {
        Self::resource_request(
            edge_ref,
            WorthQueryAsyncSourceFamily::BridgeResource,
            WorthQueryAsyncLoadingPosture::Blocking,
            WorthQueryAsyncFailurePosture::RetainStaleValue,
        )
    }

    pub(super) fn bridge_completion(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            async_clauses: vec![WorthQueryAsyncDeclarationClause::completion_request(
                WorthQueryAsyncSourceFamily::BridgeResource,
                WorthQueryAsyncFailurePosture::FailClosed,
                vec![WorthQueryAsyncRequestIdentityPart::text(
                    "edge_ref", edge_ref,
                )],
            )],
            _family: PhantomData,
        }
    }

    fn resource_request(
        edge_ref: &'static str,
        source_family: WorthQueryAsyncSourceFamily,
        loading_posture: WorthQueryAsyncLoadingPosture,
        failure_posture: WorthQueryAsyncFailurePosture,
    ) -> Self {
        Self {
            edge_ref,
            async_clauses: vec![WorthQueryAsyncDeclarationClause::resource_request(
                source_family,
                loading_posture,
                failure_posture,
                vec![WorthQueryAsyncRequestIdentityPart::text(
                    "edge_ref", edge_ref,
                )],
            )],
            _family: PhantomData,
        }
    }
}

macro_rules! impl_async_input {
    ($($family:ty),+ $(,)?) => {
        $(
            impl WorthQueryDeclarationInput<GeometryDomain> for AsyncDeclaration<$family> {
                type Family = $family;

                fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
                    vec![WorthQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
                }

                fn async_resource_declaration_clauses(&self) -> Vec<WorthQueryAsyncDeclarationClause> {
                    self.async_clauses.clone()
                }
            }
        )+
    };
}

impl_async_input!(
    AsyncCurrentFamily,
    AsyncPreviewFamily,
    AsyncHistoricalFamily
);
