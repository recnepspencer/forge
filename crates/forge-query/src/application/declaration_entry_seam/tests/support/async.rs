use std::marker::PhantomData;

use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryAsyncDeclarationClause,
    ForgeQueryAsyncDeclarationSupport, ForgeQueryAsyncFailurePosture,
    ForgeQueryAsyncLoadingPosture, ForgeQueryAsyncRequestIdentityPart, ForgeQueryAsyncSourceFamily,
    ForgeQueryBridgeContinuationAuthority, ForgeQueryDeclarationBridgeContinuationContract,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryDeclarationSignalCompatibilityContract, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQuerySignalCompatiblePosture,
};

use super::domain::GeometryDomain;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AsyncCurrentFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for AsyncCurrentFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AsyncCurrentFamily"
    }

    fn async_declaration_support() -> ForgeQueryAsyncDeclarationSupport {
        ForgeQueryAsyncDeclarationSupport::CanonicalIdentityOnly
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
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AsyncPreviewFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for AsyncPreviewFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AsyncPreviewFamily"
    }

    fn async_declaration_support() -> ForgeQueryAsyncDeclarationSupport {
        ForgeQueryAsyncDeclarationSupport::CanonicalIdentityOnly
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

    fn signal_compatibility_contract() -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
        Some(ForgeQueryDeclarationSignalCompatibilityContract::preview_derived_execution())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeferredAsyncFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for DeferredAsyncFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "DeferredAsyncFamily"
    }

    fn async_declaration_support() -> ForgeQueryAsyncDeclarationSupport {
        ForgeQueryAsyncDeclarationSupport::DeferredDebt
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsyncInput<F> {
    edge_ref: &'static str,
    async_clauses: Vec<ForgeQueryAsyncDeclarationClause>,
    _family: PhantomData<F>,
}

impl<F> AsyncInput<F> {
    pub fn bridge_blocking(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            async_clauses: vec![ForgeQueryAsyncDeclarationClause::resource_request(
                ForgeQueryAsyncSourceFamily::BridgeResource,
                ForgeQueryAsyncLoadingPosture::Blocking,
                ForgeQueryAsyncFailurePosture::FailClosed,
                vec![ForgeQueryAsyncRequestIdentityPart::text(
                    "edge_ref", edge_ref,
                )],
            )],
            _family: PhantomData,
        }
    }

    pub fn external_refresh(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            async_clauses: vec![ForgeQueryAsyncDeclarationClause::resource_request(
                ForgeQueryAsyncSourceFamily::ExternalResource,
                ForgeQueryAsyncLoadingPosture::BackgroundRefresh,
                ForgeQueryAsyncFailurePosture::RetainStaleValue,
                vec![ForgeQueryAsyncRequestIdentityPart::text(
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
        impl ForgeQueryDeclarationInput<GeometryDomain> for AsyncInput<$family> {
            type Family = $family;

            fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
                vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
            }

            fn async_resource_declaration_clauses(&self) -> Vec<ForgeQueryAsyncDeclarationClause> {
                self.async_clauses.clone()
            }
        }
    )+};
}

impl_async_input!(AsyncCurrentFamily, AsyncPreviewFamily, DeferredAsyncFamily);

pub fn async_current_envelope<C: ForgeQueryDomainOperatingContext<GeometryDomain>>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, C>,
    input: AsyncInput<AsyncCurrentFamily>,
) -> ForgeQueryDeclarationEnvelope<GeometryDomain, AsyncInput<AsyncCurrentFamily>> {
    let progressed = match handle.declare_review_and_progress(input) {
        Ok(progressed) => progressed,
        Err(_) => panic!("progression should succeed"),
    };
    match handle.envelope_routes_from_progressed(progressed) {
        Ok(envelope) => envelope,
        Err(_) => panic!("envelope should succeed"),
    }
}
