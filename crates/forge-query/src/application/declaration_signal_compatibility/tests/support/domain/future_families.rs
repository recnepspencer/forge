use crate::application::{
    ForgeQueryAsyncDeclarationClause, ForgeQueryAsyncDeclarationSupport,
    ForgeQueryAsyncFailurePosture, ForgeQueryAsyncLoadingPosture,
    ForgeQueryAsyncRequestIdentityPart, ForgeQueryAsyncSourceFamily,
    ForgeQueryBridgeContinuationAuthority, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryDeclarationSignalCompatibilityContract, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQuerySignalCompatiblePosture, ForgeQueryTemporalDeclarationClause,
    ForgeQueryTemporalDeclarationSupport, ForgeQueryTemporalDuration,
};

use super::families::Input;
use super::runtime::{
    signal_aspect_contract, signal_aspect_coverage, signal_dependency_aspects,
    signal_produced_aspects, GeometryDomain,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TemporalRuntimeFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for TemporalRuntimeFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "TemporalRuntimeFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn signal_compatibility_contract() -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
        Some(
            ForgeQueryDeclarationSignalCompatibilityContract::runtime_derived_execution()
                .with_aspects(signal_dependency_aspects(), signal_produced_aspects()),
        )
    }

    fn temporal_declaration_support() -> ForgeQueryTemporalDeclarationSupport {
        ForgeQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        signal_aspect_contract()
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        signal_aspect_coverage()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AsyncRuntimeFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for AsyncRuntimeFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AsyncRuntimeFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn signal_compatibility_contract() -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
        Some(
            ForgeQueryDeclarationSignalCompatibilityContract::runtime_derived_execution()
                .with_aspects(signal_dependency_aspects(), signal_produced_aspects()),
        )
    }

    fn async_declaration_support() -> ForgeQueryAsyncDeclarationSupport {
        ForgeQueryAsyncDeclarationSupport::CanonicalIdentityOnly
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        signal_aspect_contract()
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        signal_aspect_coverage()
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for Input<TemporalRuntimeFamily> {
    type Family = TemporalRuntimeFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text(
            "edge_ref", self.0,
        )]
    }

    fn temporal_declaration_clauses(&self) -> Vec<ForgeQueryTemporalDeclarationClause> {
        vec![ForgeQueryTemporalDeclarationClause::stale_after(
            ForgeQueryTemporalDuration::seconds(30),
        )]
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for Input<AsyncRuntimeFamily> {
    type Family = AsyncRuntimeFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text(
            "edge_ref", self.0,
        )]
    }

    fn async_resource_declaration_clauses(&self) -> Vec<ForgeQueryAsyncDeclarationClause> {
        vec![ForgeQueryAsyncDeclarationClause::resource_request(
            ForgeQueryAsyncSourceFamily::BridgeResource,
            ForgeQueryAsyncLoadingPosture::Blocking,
            ForgeQueryAsyncFailurePosture::FailClosed,
            vec![ForgeQueryAsyncRequestIdentityPart::text("edge_ref", self.0)],
        )]
    }
}
