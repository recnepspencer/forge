use super::evidence::*;
use crate::lower_runtime_routing::ForgeQueryLowerRuntimeSeamKey;

#[test]
fn representative_surface_covers_every_crossing_row_once() {
    let surface = forge_query_lower_runtime_representative_surface();
    let crossing_count =
        crate::lower_runtime_routing::forge_query_lower_runtime_crossing_inventory()
            .rows()
            .len();

    assert_eq!(surface.requests().len(), crossing_count);
    assert_eq!(surface.eligibilities().len(), crossing_count);
    assert_eq!(surface.boundary_receipts().len(), crossing_count);
    assert_eq!(surface.envelopes().len(), crossing_count);
    assert!(!surface.route_parity_digest().is_empty());
}

#[test]
fn representative_surface_uses_runtime_backed_fixtures_for_named_phase_six_seams() {
    let surface = forge_query_lower_runtime_representative_surface();

    for seam_key in [
        ForgeQueryLowerRuntimeSeamKey::ComposeRead,
        ForgeQueryLowerRuntimeSeamKey::ComposeReadWithInvariantPack,
        ForgeQueryLowerRuntimeSeamKey::ExecuteReadFamily,
        ForgeQueryLowerRuntimeSeamKey::ExecuteReadFamilyInBasisContext,
        ForgeQueryLowerRuntimeSeamKey::ExecuteRuntimeCurrentReadGraph,
        ForgeQueryLowerRuntimeSeamKey::ExecuteRuntimeBasisContextReadGraph,
        ForgeQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission,
        ForgeQueryLowerRuntimeSeamKey::LiveViewSourceDeclaration,
        ForgeQueryLowerRuntimeSeamKey::PublicLiveViewDeclaration,
        ForgeQueryLowerRuntimeSeamKey::RuntimeLiveInstallationOrchestration,
        ForgeQueryLowerRuntimeSeamKey::PreviewBasisAdmission,
        ForgeQueryLowerRuntimeSeamKey::SubscriptionActivation,
        ForgeQueryLowerRuntimeSeamKey::SubscriptionContinuity,
        ForgeQueryLowerRuntimeSeamKey::BasisReadmissionFromTruthViewEvidence,
        ForgeQueryLowerRuntimeSeamKey::BasisReadmissionFromSubscriptionEvidence,
        ForgeQueryLowerRuntimeSeamKey::HistoricalBridgeLowering,
        ForgeQueryLowerRuntimeSeamKey::EffectBackedRelationalMutation,
        ForgeQueryLowerRuntimeSeamKey::EffectBackedRelationalMerge,
        ForgeQueryLowerRuntimeSeamKey::EffectBackedBridgeWriteback,
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        ForgeQueryLowerRuntimeSeamKey::RuntimeIntentAuthorityAdapter,
        ForgeQueryLowerRuntimeSeamKey::IntentRuntimeExecution,
        ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromQueryReceipts,
        ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromRelationalArtifacts,
        ForgeQueryLowerRuntimeSeamKey::ProjectionSourceIntakeFromBridgeArtifacts,
        ForgeQueryLowerRuntimeSeamKey::CausalBridgeMaterialization,
        ForgeQueryLowerRuntimeSeamKey::FrontierEvidenceIntake,
    ] {
        assert_eq!(
            surface.evidence_source_for(seam_key),
            Some(ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture)
        );
    }
}
