use crate::application::{
    WorthQueryAsyncDeclarationClause, WorthQueryAsyncDeclarationSupport,
    WorthQueryAsyncFailurePosture, WorthQueryAsyncLoadingPosture,
    WorthQueryAsyncRequestIdentityPart, WorthQueryAsyncSourceFamily,
    WorthQueryBridgeContinuationAuthority, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationCanonicalEntry,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput,
    WorthQueryDeclarationLegalityContract, WorthQueryDeclarationRouteContract,
    WorthQueryDeclarationSignalCompatibilityContract, WorthQueryNeighborhoodCapableGrouping,
    WorthQuerySignalCompatiblePosture, WorthQueryTemporalDeclarationClause,
    WorthQueryTemporalDeclarationSupport, WorthQueryTemporalDuration,
};

use super::families::Input;
use super::runtime::{
    signal_aspect_contract, signal_aspect_coverage, signal_dependency_aspects,
    signal_produced_aspects, GeometryDomain,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TemporalRuntimeFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for TemporalRuntimeFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "TemporalRuntimeFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn signal_compatibility_contract() -> Option<WorthQueryDeclarationSignalCompatibilityContract> {
        Some(
            WorthQueryDeclarationSignalCompatibilityContract::runtime_derived_execution()
                .with_aspects(signal_dependency_aspects(), signal_produced_aspects()),
        )
    }

    fn temporal_declaration_support() -> WorthQueryTemporalDeclarationSupport {
        WorthQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        signal_aspect_contract()
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        signal_aspect_coverage()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AsyncRuntimeFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AsyncRuntimeFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AsyncRuntimeFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn signal_compatibility_contract() -> Option<WorthQueryDeclarationSignalCompatibilityContract> {
        Some(
            WorthQueryDeclarationSignalCompatibilityContract::runtime_derived_execution()
                .with_aspects(signal_dependency_aspects(), signal_produced_aspects()),
        )
    }

    fn async_declaration_support() -> WorthQueryAsyncDeclarationSupport {
        WorthQueryAsyncDeclarationSupport::CanonicalIdentityOnly
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        signal_aspect_contract()
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        signal_aspect_coverage()
    }
}

impl WorthQueryDeclarationInput<GeometryDomain> for Input<TemporalRuntimeFamily> {
    type Family = TemporalRuntimeFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text(
            "edge_ref", self.0,
        )]
    }

    fn temporal_declaration_clauses(&self) -> Vec<WorthQueryTemporalDeclarationClause> {
        vec![WorthQueryTemporalDeclarationClause::stale_after(
            WorthQueryTemporalDuration::seconds(30),
        )]
    }
}

impl WorthQueryDeclarationInput<GeometryDomain> for Input<AsyncRuntimeFamily> {
    type Family = AsyncRuntimeFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![WorthQueryDeclarationCanonicalEntry::text(
            "edge_ref", self.0,
        )]
    }

    fn async_resource_declaration_clauses(&self) -> Vec<WorthQueryAsyncDeclarationClause> {
        vec![WorthQueryAsyncDeclarationClause::resource_request(
            WorthQueryAsyncSourceFamily::BridgeResource,
            WorthQueryAsyncLoadingPosture::Blocking,
            WorthQueryAsyncFailurePosture::FailClosed,
            vec![WorthQueryAsyncRequestIdentityPart::text("edge_ref", self.0)],
        )]
    }
}
