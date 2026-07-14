use std::marker::PhantomData;

use crate::application::{
    WorthQueryBridgeContinuationAuthority, WorthQueryDeclarationBridgeContinuationContract,
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationEnvelope,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput,
    WorthQueryDeclarationLegalityContract, WorthQueryDeclarationRouteContract,
    WorthQueryDeclarationSignalCompatibilityContract, WorthQueryDomainOperatingContext,
    WorthQueryInstalledDomainDeclarationContext, WorthQueryNeighborhoodCapableGrouping,
    WorthQuerySignalCompatiblePosture, WorthQueryTemporalDeclarationClause,
    WorthQueryTemporalDeclarationSupport, WorthQueryTemporalDuration,
};

use super::domain::GeometryDomain;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TemporalCurrentFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for TemporalCurrentFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "TemporalCurrentFamily"
    }

    fn temporal_declaration_support() -> WorthQueryTemporalDeclarationSupport {
        WorthQueryTemporalDeclarationSupport::CanonicalIdentityOnly
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
pub struct TemporalPreviewFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for TemporalPreviewFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "TemporalPreviewFamily"
    }

    fn temporal_declaration_support() -> WorthQueryTemporalDeclarationSupport {
        WorthQueryTemporalDeclarationSupport::CanonicalIdentityOnly
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TemporalHistoricalFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for TemporalHistoricalFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "TemporalHistoricalFamily"
    }

    fn temporal_declaration_support() -> WorthQueryTemporalDeclarationSupport {
        WorthQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(WorthQueryDeclarationBridgeContinuationContract::truth_view_historical())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TemporalInput<F> {
    edge_ref: &'static str,
    temporal_clauses: Vec<WorthQueryTemporalDeclarationClause>,
    _family: PhantomData<F>,
}

impl<F> TemporalInput<F> {
    pub fn stale(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            temporal_clauses: vec![WorthQueryTemporalDeclarationClause::stale_after(
                WorthQueryTemporalDuration::seconds(30),
            )],
            _family: PhantomData,
        }
    }

    pub fn plain(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            temporal_clauses: Vec::new(),
            _family: PhantomData,
        }
    }
}

macro_rules! impl_temporal_input {
    ($($family:ty),+ $(,)?) => {$(
        impl WorthQueryDeclarationInput<GeometryDomain> for TemporalInput<$family> {
            type Family = $family;

            fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
                vec![WorthQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
            }

            fn temporal_declaration_clauses(&self) -> Vec<WorthQueryTemporalDeclarationClause> {
                self.temporal_clauses.clone()
            }
        }
    )+};
}

impl_temporal_input!(
    TemporalCurrentFamily,
    TemporalPreviewFamily,
    TemporalHistoricalFamily
);

pub fn temporal_current_envelope<C: WorthQueryDomainOperatingContext<GeometryDomain>>(
    handle: &WorthQueryInstalledDomainDeclarationContext<GeometryDomain, C>,
    input: TemporalInput<TemporalCurrentFamily>,
) -> WorthQueryDeclarationEnvelope<GeometryDomain, TemporalInput<TemporalCurrentFamily>> {
    let progressed = match handle.declare_review_and_progress(input) {
        Ok(progressed) => progressed,
        Err(_) => panic!("progression should succeed"),
    };
    match handle.envelope_routes_from_progressed(progressed) {
        Ok(envelope) => envelope,
        Err(_) => panic!("envelope should succeed"),
    }
}
