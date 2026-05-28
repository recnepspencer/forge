use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationRefusalClass,
    ForgeQueryDeclarationEntryOrchestrationStage,
};
use crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts;
use crate::grouped_authoring::{
    forge_query_grouped_declaration_checked_on_handle,
    forge_query_grouped_orchestration_checked_on_handle, ForgeQueryGroupedDeclarationChecked,
    ForgeQueryGroupedDeclarationInput, ForgeQueryGroupedOrchestrationChecked,
};
use crate::ordinary_outcome::{
    ForgeQueryOrdinaryCheckedTopology, ForgeQueryOrdinaryContinuationCheckedTopologyKind,
    ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture,
    ForgeQueryOrdinaryPostureKind,
    ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind,
};

use super::support::{
    recovery_admitted_handle, RecoveryInput, RequiredIntentRouteFamily, SignalReceiptFamily,
};
use crate::recovery_boundary::{
    forge_query_recovery_brief_from_ordinary_outcome, ForgeQueryRecoveryAction,
    ForgeQueryRecoveryAspectPosture, ForgeQueryRecoveryAuthoritySurface,
    ForgeQueryRecoveryEvidenceStrength, ForgeQueryRecoverySourceFamily,
    ForgeQueryRecoveryStopFamily, ForgeQueryRecoveryStopKind,
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

#[test]
fn grouped_wrong_world_checked_uses_grouped_recovery_family() {
    let left = recovery_admitted_handle("left");
    let right = recovery_admitted_handle("right");
    let declaration = match forge_query_grouped_declaration_checked_on_handle(
        &left,
        ForgeQueryGroupedDeclarationInput::local_neighborhood(
            RecoveryInput::<SignalReceiptFamily>::new("edge-a"),
        ),
    ) {
        ForgeQueryGroupedDeclarationChecked::Bound(value) => value,
        ForgeQueryGroupedDeclarationChecked::MemberStopped(_) => {
            panic!("grouped declaration should admit")
        }
    };

    let checked = forge_query_grouped_orchestration_checked_on_handle(&right, declaration);
    assert!(matches!(
        checked,
        ForgeQueryGroupedOrchestrationChecked::WrongWorld(_)
    ));

    let brief = right
        .recover_from_grouped_orchestration_checked(checked)
        .expect("grouped wrong-world stop should yield a recovery brief");
    assert_eq!(
        brief.stop_family(),
        ForgeQueryRecoveryStopFamily::GroupedNeighborhoodOrchestration
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
fn grouped_wrong_world_recovery_brief_matches_between_checked_and_proof_lanes() {
    let left = recovery_admitted_handle("left");
    let right = recovery_admitted_handle("right");
    let declaration = |edge_ref| match forge_query_grouped_declaration_checked_on_handle(
        &left,
        ForgeQueryGroupedDeclarationInput::local_neighborhood(
            RecoveryInput::<SignalReceiptFamily>::new(edge_ref),
        ),
    ) {
        ForgeQueryGroupedDeclarationChecked::Bound(value) => value,
        ForgeQueryGroupedDeclarationChecked::MemberStopped(_) => {
            panic!("grouped declaration should admit")
        }
    };

    let checked_brief = right
        .recover_from_grouped_orchestration_checked(
            forge_query_grouped_orchestration_checked_on_handle(&right, declaration("edge-b")),
        )
        .expect("checked grouped stop should yield a recovery brief");
    let proof_brief = right
        .recover_from_grouped_orchestration_proof(
            crate::grouped_authoring::forge_query_grouped_orchestration_proof_on_handle(
                &right,
                declaration("edge-b"),
            ),
        )
        .expect("proof grouped stop should yield a recovery brief");

    assert_eq!(checked_brief.stop_family(), proof_brief.stop_family());
    assert_eq!(checked_brief.stop_kind(), proof_brief.stop_kind());
    assert_eq!(
        checked_brief.authority_surface(),
        proof_brief.authority_surface()
    );
    assert_eq!(
        checked_brief.recommended_action(),
        proof_brief.recommended_action()
    );
}

#[test]
fn grouped_member_stop_recovery_preserves_member_aspect_context() {
    let handle = recovery_admitted_handle("main");
    let declaration = match forge_query_grouped_declaration_checked_on_handle(
        &handle,
        ForgeQueryGroupedDeclarationInput::local_neighborhood(RecoveryInput::<
            RequiredIntentRouteFamily,
        >::new("edge-c"))
        .with_member(RecoveryInput::<RequiredIntentRouteFamily>::new("edge-d")),
    ) {
        ForgeQueryGroupedDeclarationChecked::Bound(value) => value,
        ForgeQueryGroupedDeclarationChecked::MemberStopped(_) => {
            panic!("grouped declaration should admit")
        }
    };
    let expected_member_aspect = declaration.members()[0].aspect_record().clone();

    let checked = forge_query_grouped_orchestration_checked_on_handle(&handle, declaration);
    let brief = handle
        .recover_from_grouped_orchestration_checked(checked)
        .expect("grouped member stop should yield a recovery brief");

    assert_eq!(
        brief.stop_family(),
        ForgeQueryRecoveryStopFamily::GroupedNeighborhoodOrchestration
    );
    assert_eq!(
        brief.source_family(),
        ForgeQueryRecoverySourceFamily::GroupedNeighborhood
    );
    assert_eq!(
        brief.evidence_strength(),
        ForgeQueryRecoveryEvidenceStrength::CheckedRetained
    );
    assert_eq!(
        brief.aspect_posture(),
        ForgeQueryRecoveryAspectPosture::RetainedContractAndCoverage
    );
    let context = brief
        .explanation()
        .grouped_member_context()
        .expect("grouped member stop should carry member-local aspect context");
    assert_eq!(context.member_index(), 0);
    assert_eq!(context.member_role().as_str(), "seed");
    assert_eq!(context.aspect_record(), &expected_member_aspect);
    assert!(brief
        .explanation()
        .has_retained_grouped_member_aspect_context());
}

#[test]
fn grouped_member_stop_recovery_brief_matches_between_checked_and_proof_lanes() {
    let handle = recovery_admitted_handle("main");
    let declaration = || match forge_query_grouped_declaration_checked_on_handle(
        &handle,
        ForgeQueryGroupedDeclarationInput::local_neighborhood(RecoveryInput::<
            RequiredIntentRouteFamily,
        >::new("edge-c"))
        .with_member(RecoveryInput::<RequiredIntentRouteFamily>::new("edge-d")),
    ) {
        ForgeQueryGroupedDeclarationChecked::Bound(value) => value,
        ForgeQueryGroupedDeclarationChecked::MemberStopped(_) => {
            panic!("grouped declaration should admit")
        }
    };

    let checked_brief = handle
        .recover_from_grouped_orchestration_checked(
            forge_query_grouped_orchestration_checked_on_handle(&handle, declaration()),
        )
        .expect("checked grouped member stop should yield a recovery brief");
    let proof_brief = handle
        .recover_from_grouped_orchestration_proof(
            crate::grouped_authoring::forge_query_grouped_orchestration_proof_on_handle(
                &handle,
                declaration(),
            ),
        )
        .expect("proof grouped member stop should yield a recovery brief");

    assert_eq!(checked_brief.stop_family(), proof_brief.stop_family());
    assert_eq!(checked_brief.stop_kind(), proof_brief.stop_kind());
    assert_eq!(checked_brief.source_family(), proof_brief.source_family());
    assert_eq!(checked_brief.aspect_posture(), proof_brief.aspect_posture());
    assert_eq!(
        checked_brief.authority_surface(),
        proof_brief.authority_surface()
    );
    assert_eq!(
        checked_brief.recommended_action(),
        proof_brief.recommended_action()
    );
    assert_eq!(
        checked_brief
            .explanation()
            .grouped_member_context()
            .expect("checked grouped member stop should carry member context"),
        proof_brief
            .explanation()
            .grouped_member_context()
            .expect("proof grouped member stop should carry member context")
    );
    assert_eq!(
        checked_brief.evidence_strength(),
        ForgeQueryRecoveryEvidenceStrength::CheckedRetained
    );
    assert_eq!(
        proof_brief.evidence_strength(),
        ForgeQueryRecoveryEvidenceStrength::ProofRetained
    );
}
