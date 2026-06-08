use std::marker::PhantomData;

use crate::application::{
    ForgeQueryAsyncDeclarationClause, ForgeQueryAsyncDeclarationSupport,
    ForgeQueryAsyncFailurePosture, ForgeQueryAsyncLoadingPosture,
    ForgeQueryAsyncRequestIdentityPart, ForgeQueryAsyncSourceFamily,
    ForgeQueryBridgeContinuationAuthority, ForgeQueryDeclarationBridgeContinuationContract,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQuerySignalCompatiblePosture,
};

use super::fixtures::GeometryDomain;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AsyncCurrentFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for AsyncCurrentFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "async-current"
    }

    fn async_declaration_support() -> ForgeQueryAsyncDeclarationSupport {
        ForgeQueryAsyncDeclarationSupport::CanonicalIdentityOnly
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn bridge_continuation_contract() -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
        Some(ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AsyncPreviewFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for AsyncPreviewFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "async-preview"
    }

    fn async_declaration_support() -> ForgeQueryAsyncDeclarationSupport {
        ForgeQueryAsyncDeclarationSupport::CanonicalIdentityOnly
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn bridge_continuation_contract() -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
        Some(ForgeQueryDeclarationBridgeContinuationContract::preview_session())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AsyncHistoricalFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for AsyncHistoricalFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "async-historical"
    }

    fn async_declaration_support() -> ForgeQueryAsyncDeclarationSupport {
        ForgeQueryAsyncDeclarationSupport::CanonicalIdentityOnly
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn bridge_continuation_contract() -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
        Some(ForgeQueryDeclarationBridgeContinuationContract::truth_view_historical())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct AsyncDeclaration<F> {
    edge_ref: &'static str,
    async_clauses: Vec<ForgeQueryAsyncDeclarationClause>,
    _family: PhantomData<F>,
}

impl<F> AsyncDeclaration<F> {
    pub(super) fn bridge_blocking(edge_ref: &'static str) -> Self {
        Self::resource_request(
            edge_ref,
            ForgeQueryAsyncSourceFamily::BridgeResource,
            ForgeQueryAsyncLoadingPosture::Blocking,
            ForgeQueryAsyncFailurePosture::FailClosed,
        )
    }

    pub(super) fn external_blocking(edge_ref: &'static str) -> Self {
        Self::resource_request(
            edge_ref,
            ForgeQueryAsyncSourceFamily::ExternalResource,
            ForgeQueryAsyncLoadingPosture::Blocking,
            ForgeQueryAsyncFailurePosture::FailClosed,
        )
    }

    pub(super) fn bridge_refresh(edge_ref: &'static str) -> Self {
        Self::resource_request(
            edge_ref,
            ForgeQueryAsyncSourceFamily::BridgeResource,
            ForgeQueryAsyncLoadingPosture::BackgroundRefresh,
            ForgeQueryAsyncFailurePosture::FailClosed,
        )
    }

    pub(super) fn bridge_retain_stale(edge_ref: &'static str) -> Self {
        Self::resource_request(
            edge_ref,
            ForgeQueryAsyncSourceFamily::BridgeResource,
            ForgeQueryAsyncLoadingPosture::Blocking,
            ForgeQueryAsyncFailurePosture::RetainStaleValue,
        )
    }

    pub(super) fn bridge_completion(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            async_clauses: vec![ForgeQueryAsyncDeclarationClause::completion_request(
                ForgeQueryAsyncSourceFamily::BridgeResource,
                ForgeQueryAsyncFailurePosture::FailClosed,
                vec![ForgeQueryAsyncRequestIdentityPart::text(
                    "edge_ref", edge_ref,
                )],
            )],
            _family: PhantomData,
        }
    }

    fn resource_request(
        edge_ref: &'static str,
        source_family: ForgeQueryAsyncSourceFamily,
        loading_posture: ForgeQueryAsyncLoadingPosture,
        failure_posture: ForgeQueryAsyncFailurePosture,
    ) -> Self {
        Self {
            edge_ref,
            async_clauses: vec![ForgeQueryAsyncDeclarationClause::resource_request(
                source_family,
                loading_posture,
                failure_posture,
                vec![ForgeQueryAsyncRequestIdentityPart::text(
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
            impl ForgeQueryDeclarationInput<GeometryDomain> for AsyncDeclaration<$family> {
                type Family = $family;

                fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
                    vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
                }

                fn async_resource_declaration_clauses(&self) -> Vec<ForgeQueryAsyncDeclarationClause> {
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
