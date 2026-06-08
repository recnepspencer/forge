use std::marker::PhantomData;

use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryBridgeContinuationAuthority,
    ForgeQueryDeclarationBridgeContinuationContract, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryDeclarationSignalCompatibilityContract, ForgeQueryDomainOperatingContext,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQuerySignalCompatiblePosture,
    ForgeQueryTemporalDeclarationClause, ForgeQueryTemporalDeclarationSupport,
    ForgeQueryTemporalDuration,
};

use super::domain::GeometryDomain;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TemporalCurrentFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for TemporalCurrentFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "TemporalCurrentFamily"
    }

    fn temporal_declaration_support() -> ForgeQueryTemporalDeclarationSupport {
        ForgeQueryTemporalDeclarationSupport::CanonicalIdentityOnly
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
pub struct TemporalPreviewFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for TemporalPreviewFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "TemporalPreviewFamily"
    }

    fn temporal_declaration_support() -> ForgeQueryTemporalDeclarationSupport {
        ForgeQueryTemporalDeclarationSupport::CanonicalIdentityOnly
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TemporalHistoricalFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for TemporalHistoricalFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "TemporalHistoricalFamily"
    }

    fn temporal_declaration_support() -> ForgeQueryTemporalDeclarationSupport {
        ForgeQueryTemporalDeclarationSupport::CanonicalIdentityOnly
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn bridge_continuation_contract() -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
        Some(ForgeQueryDeclarationBridgeContinuationContract::truth_view_historical())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TemporalInput<F> {
    edge_ref: &'static str,
    temporal_clauses: Vec<ForgeQueryTemporalDeclarationClause>,
    _family: PhantomData<F>,
}

impl<F> TemporalInput<F> {
    pub fn stale(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            temporal_clauses: vec![ForgeQueryTemporalDeclarationClause::stale_after(
                ForgeQueryTemporalDuration::seconds(30),
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
        impl ForgeQueryDeclarationInput<GeometryDomain> for TemporalInput<$family> {
            type Family = $family;

            fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
                vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
            }

            fn temporal_declaration_clauses(&self) -> Vec<ForgeQueryTemporalDeclarationClause> {
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

pub fn temporal_current_envelope<C: ForgeQueryDomainOperatingContext<GeometryDomain>>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, C>,
    input: TemporalInput<TemporalCurrentFamily>,
) -> ForgeQueryDeclarationEnvelope<GeometryDomain, TemporalInput<TemporalCurrentFamily>> {
    let progressed = match handle.declare_review_and_progress(input) {
        Ok(progressed) => progressed,
        Err(_) => panic!("progression should succeed"),
    };
    match handle.envelope_routes_from_progressed(progressed) {
        Ok(envelope) => envelope,
        Err(_) => panic!("envelope should succeed"),
    }
}
