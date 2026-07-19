use std::marker::PhantomData;

use crate::application::{
    WorthQueryAsyncDeclarationClause, WorthQueryAsyncDeclarationSupport,
    WorthQueryAsyncFailurePosture, WorthQueryAsyncLoadingPosture,
    WorthQueryAsyncRequestIdentityPart, WorthQueryAsyncSourceFamily,
    WorthQueryBridgeContinuationAuthority, WorthQueryDeclarationBridgeContinuationContract,
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationEnvelope,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput,
    WorthQueryDeclarationLegalityContract, WorthQueryDeclarationRouteContract,
    WorthQueryDeclarationSignalCompatibilityContract, WorthQueryDomainOperatingContext,
    WorthQueryInstalledDomainDeclarationContext, WorthQueryNeighborhoodCapableGrouping,
    WorthQuerySignalCompatiblePosture,
};

use super::domain::GeometryDomain;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AsyncCurrentFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AsyncCurrentFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AsyncCurrentFamily"
    }

    fn async_declaration_support() -> WorthQueryAsyncDeclarationSupport {
        WorthQueryAsyncDeclarationSupport::CanonicalIdentityOnly
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
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AsyncPreviewFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AsyncPreviewFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AsyncPreviewFamily"
    }

    fn async_declaration_support() -> WorthQueryAsyncDeclarationSupport {
        WorthQueryAsyncDeclarationSupport::CanonicalIdentityOnly
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

    fn signal_compatibility_contract() -> Option<WorthQueryDeclarationSignalCompatibilityContract> {
        Some(WorthQueryDeclarationSignalCompatibilityContract::preview_derived_execution())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeferredAsyncFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for DeferredAsyncFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "DeferredAsyncFamily"
    }

    fn async_declaration_support() -> WorthQueryAsyncDeclarationSupport {
        WorthQueryAsyncDeclarationSupport::DeferredDebt
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsyncInput<F> {
    edge_ref: &'static str,
    async_clauses: Vec<WorthQueryAsyncDeclarationClause>,
    _family: PhantomData<F>,
}

impl<F> AsyncInput<F> {
    pub fn bridge_blocking(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            async_clauses: vec![WorthQueryAsyncDeclarationClause::resource_request(
                WorthQueryAsyncSourceFamily::BridgeResource,
                WorthQueryAsyncLoadingPosture::Blocking,
                WorthQueryAsyncFailurePosture::FailClosed,
                vec![WorthQueryAsyncRequestIdentityPart::text(
                    "edge_ref", edge_ref,
                )],
            )],
            _family: PhantomData,
        }
    }

    pub fn external_refresh(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            async_clauses: vec![WorthQueryAsyncDeclarationClause::resource_request(
                WorthQueryAsyncSourceFamily::ExternalResource,
                WorthQueryAsyncLoadingPosture::BackgroundRefresh,
                WorthQueryAsyncFailurePosture::RetainStaleValue,
                vec![WorthQueryAsyncRequestIdentityPart::text(
                    "edge_ref", edge_ref,
                )],
            )],
            _family: PhantomData,
        }
    }

    pub fn plain(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            async_clauses: Vec::new(),
            _family: PhantomData,
        }
    }
}

macro_rules! impl_async_input {
    ($($family:ty),+ $(,)?) => {$(
        impl WorthQueryDeclarationInput<GeometryDomain> for AsyncInput<$family> {
            type Family = $family;

            fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
                vec![WorthQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
            }

            fn async_resource_declaration_clauses(&self) -> Vec<WorthQueryAsyncDeclarationClause> {
                self.async_clauses.clone()
            }
        }
    )+};
}

impl_async_input!(AsyncCurrentFamily, AsyncPreviewFamily, DeferredAsyncFamily);

pub fn async_current_envelope<C: WorthQueryDomainOperatingContext<GeometryDomain>>(
    handle: &WorthQueryInstalledDomainDeclarationContext<GeometryDomain, C>,
    input: AsyncInput<AsyncCurrentFamily>,
) -> WorthQueryDeclarationEnvelope<GeometryDomain, AsyncInput<AsyncCurrentFamily>> {
    let progressed = match handle.declare_review_and_progress(input) {
        Ok(progressed) => progressed,
        Err(_) => panic!("progression should succeed"),
    };
    match handle.envelope_routes_from_progressed(progressed) {
        Ok(envelope) => envelope,
        Err(_) => panic!("envelope should succeed"),
    }
}
