use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationRefusalClass,
    ForgeQueryDeclarationEntryOrchestrationStage,
};
use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;
use crate::ordinary_outcome::{
    ForgeQueryOrdinaryCheckedTopology, ForgeQueryOrdinaryContinuationCheckedTopologyKind,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
    ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind,
};

use crate::recovery_boundary::{
    forge_query_recovery_brief_from_ordinary_outcome, ForgeQueryRecoveryAction,
    ForgeQueryRecoveryAuthoritySurface, ForgeQueryRecoveryStopFamily, ForgeQueryRecoveryStopKind,
};

#[test]
fn ordinary_contribution_denial_uses_contribution_recovery_surface() {
    let posture = ForgeQueryOrdinaryPosture::new(
        "contribution denied",
        ForgeQueryOrdinaryPostureKind::Denied,
        ForgeQueryOrdinaryNextStep::InspectCheckedLane,
        ForgeQueryOrdinaryCheckedTopology::contribution_composed(
            crate::ordinary_outcome::ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::ContributionDenied,
            ForgeQueryBindingLinkedArtifacts::new().with_envelope_digest("env-1"),
            Some("contrib-1".to_string()),
        ),
    );
    let brief = forge_query_recovery_brief_from_ordinary_outcome(
        &ForgeQueryOrdinaryOutcome::<()>::Denied(posture),
    )
    .expect("denial should yield a recovery brief");

    assert_eq!(
        brief.stop_family(),
        ForgeQueryRecoveryStopFamily::ContributionComposedOrchestration
    );
    assert_eq!(
        brief.stop_kind(),
        ForgeQueryRecoveryStopKind::ContributionDenied
    );
    assert_eq!(
        brief.authority_surface(),
        ForgeQueryRecoveryAuthoritySurface::ContributionComposition
    );
    assert_eq!(
        brief.recommended_action(),
        ForgeQueryRecoveryAction::ReviewContributionIntent
    );
    assert_eq!(
        brief.route_sensitive_explanation().contribution_digest(),
        Some("contrib-1")
    );
    assert_eq!(
        brief.recovery_request().explanation().contribution_digest(),
        Some("contrib-1")
    );
}

#[test]
fn ordinary_declaration_refusal_preserves_route_sensitive_refusal_context() {
    let brief = forge_query_recovery_brief_from_ordinary_outcome(
        &ForgeQueryOrdinaryOutcome::<()>::Refused(ForgeQueryOrdinaryPosture::new(
            "prepared but not executed",
            ForgeQueryOrdinaryPostureKind::Refused,
            ForgeQueryOrdinaryNextStep::UseExplicitHandoff,
            ForgeQueryOrdinaryCheckedTopology::orchestration(
                ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned,
                Some("retained-1".to_string()),
                Some(
                    ForgeQueryDeclarationEntryOrchestrationRefusalClass::PreparedButNotExecutedContinuation,
                ),
            ),
        )),
    )
    .expect("refusal should yield a recovery brief");

    assert_eq!(
        brief.stop_family(),
        ForgeQueryRecoveryStopFamily::DeclarationEntry
    );
    assert_eq!(brief.stop_kind(), ForgeQueryRecoveryStopKind::Refused);
    assert_eq!(
        brief.authority_surface(),
        ForgeQueryRecoveryAuthoritySurface::AutomationBoundary
    );
    assert_eq!(
        brief.recommended_action(),
        ForgeQueryRecoveryAction::UseExplicitHandoff
    );
    assert_eq!(
        brief.route_sensitive_explanation().refusal_class(),
        Some(
            crate::application::ForgeQueryDeclarationEntryOrchestrationRefusalClass::PreparedButNotExecutedContinuation,
        )
    );
}

#[test]
fn ordinary_continuation_wrong_world_maps_to_world_repair() {
    let brief = forge_query_recovery_brief_from_ordinary_outcome(
        &ForgeQueryOrdinaryOutcome::<()>::WrongWorld(ForgeQueryOrdinaryPosture::new(
            "wrong world",
            ForgeQueryOrdinaryPostureKind::WrongWorld,
            ForgeQueryOrdinaryNextStep::CorrectWorld,
            ForgeQueryOrdinaryCheckedTopology::continuation(
                ForgeQueryOrdinaryContinuationCheckedTopologyKind::WrongWorld,
                ForgeQueryBindingLinkedArtifacts::new().with_envelope_digest("env-1"),
            ),
        )),
    )
    .expect("wrong-world continuation should yield a recovery brief");

    assert_eq!(
        brief.stop_family(),
        ForgeQueryRecoveryStopFamily::Continuation
    );
    assert_eq!(brief.stop_kind(), ForgeQueryRecoveryStopKind::WrongWorld);
    assert_eq!(
        brief.authority_surface(),
        ForgeQueryRecoveryAuthoritySurface::AdmittedOperatingWorld
    );
    assert_eq!(
        brief.recommended_action(),
        ForgeQueryRecoveryAction::CorrectWorld
    );
}

#[test]
fn ordinary_signal_basis_mismatch_maps_to_signal_repair() {
    let brief = forge_query_recovery_brief_from_ordinary_outcome(
        &ForgeQueryOrdinaryOutcome::<()>::BasisMismatch(ForgeQueryOrdinaryPosture::new(
            "basis mismatch",
            ForgeQueryOrdinaryPostureKind::BasisMismatch,
            ForgeQueryOrdinaryNextStep::RefreshBasis,
            ForgeQueryOrdinaryCheckedTopology::signal_compatibility_orchestration(
                ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind::BasisMismatch,
                ForgeQueryBindingLinkedArtifacts::new().with_envelope_digest("env-1"),
            ),
        )),
    )
    .expect("basis mismatch should yield a recovery brief");

    assert_eq!(
        brief.stop_family(),
        ForgeQueryRecoveryStopFamily::SignalCompatibilityOrchestration
    );
    assert_eq!(brief.stop_kind(), ForgeQueryRecoveryStopKind::BasisMismatch);
    assert_eq!(
        brief.authority_surface(),
        ForgeQueryRecoveryAuthoritySurface::SignalCompatibility
    );
    assert_eq!(
        brief.recommended_action(),
        ForgeQueryRecoveryAction::RefreshBasis
    );
}
