use forge_foundational::FoundationalDiagnosticOutcomeKind;

use crate::contribution_composed_orchestration::{
    ForgeQueryContributionComposedOrchestrationInput, ForgeQueryContributionIntent,
};
use crate::domain_capabilities::ForgeQuerySupportContributionAuthoring;
use crate::ordinary_outcome::ForgeQueryOrdinaryOutcome;

use super::support::{
    admitted_handle, ContributionInput, DeferredAdmissionContributionInput,
    DeferredContributionInput,
};

#[test]
fn contribution_denial_survives_after_declaration_success() {
    let handle = admitted_handle();
    let outcome = handle.orchestrate_declaration_with_contributions_checked(
        ForgeQueryContributionComposedOrchestrationInput::new(ContributionInput::new("face-b"))
            .with_contribution(ForgeQueryContributionIntent::support(
                ForgeQuerySupportContributionAuthoring::declaration_traceability(
                    "domain.traceability.face",
                    "",
                ),
            )),
    );

    match outcome {
        crate::contribution_composed_orchestration::ForgeQueryContributionComposedOrchestrationOutcome::ContributionDenied(
            posture,
        ) => {
            assert!(posture.reason().contains("non-empty detail"));
            assert!(posture.linked_artifacts().envelope_digest().is_some());
        }
        _ => panic!("expected contribution denial after declaration success"),
    }
}

#[test]
fn ordinary_outcome_preserves_contribution_checked_topology() {
    let handle = admitted_handle();
    let ordinary = handle.orchestrate_declaration_with_contributions_outcome(
        ForgeQueryContributionComposedOrchestrationInput::new(ContributionInput::new("face-c"))
            .with_contribution(ForgeQueryContributionIntent::support(
                ForgeQuerySupportContributionAuthoring::declaration_traceability(
                    "domain.traceability.face",
                    "",
                ),
            )),
    );

    match ordinary {
        ForgeQueryOrdinaryOutcome::Denied(posture) => {
            assert_eq!(
                posture.checked_topology().contribution_composed_kind(),
                Some(
                    crate::ordinary_outcome::ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::ContributionDenied
                )
            );
            assert!(posture
                .checked_topology()
                .contribution_composed_linked_artifacts()
                .and_then(|value| value.envelope_digest())
                .is_some());
        }
        _ => panic!("expected ordinary denial"),
    }
}

#[test]
fn bound_contributions_preserve_typed_semantic_posture() {
    let handle = admitted_handle();
    let composed = handle
        .orchestrate_declaration_with_contributions(
            ForgeQueryContributionComposedOrchestrationInput::new(ContributionInput::new(
                "face-posture",
            ))
            .with_contribution(ForgeQueryContributionIntent::workflow(
                crate::domain_capabilities::ForgeQueryWorkflowContributionAuthoring::preview_only(
                    "domain.workflow.face",
                    "preview workflow remains read-only at declaration entry",
                ),
            )),
        )
        .unwrap_or_else(|_| panic!("expected composed workflow contribution"));

    let contribution = &composed.contributions()[0];
    assert_eq!(
        contribution.contribution_category(),
        crate::domain_capabilities::ForgeQueryDomainCapabilityCategory::WorkflowPreview
    );
    assert_eq!(
        contribution.semantic_posture(),
        crate::domain_capabilities::ForgeQueryDomainCapabilitySemanticPosture::WorkflowPreviewOnly
    );
    assert_eq!(
        contribution.support_outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Advisory
    );
}

#[test]
fn declaration_deferred_stays_distinct_on_composed_lane() {
    let handle = admitted_handle();
    let checked = handle.orchestrate_declaration_with_contributions_checked(
        ForgeQueryContributionComposedOrchestrationInput::new(DeferredContributionInput::new(
            "face-deferred",
        )),
    );
    let ordinary = handle.orchestrate_declaration_with_contributions_outcome(
        ForgeQueryContributionComposedOrchestrationInput::new(DeferredContributionInput::new(
            "face-deferred",
        )),
    );

    assert!(matches!(
        checked,
        crate::contribution_composed_orchestration::ForgeQueryContributionComposedOrchestrationOutcome::Deferred(_)
    ));
    match ordinary {
        ForgeQueryOrdinaryOutcome::Deferred(posture) => {
            assert_eq!(
                posture.checked_topology().contribution_composed_kind(),
                Some(
                    crate::ordinary_outcome::ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::Deferred
                )
            );
        }
        _ => panic!("expected ordinary deferred posture"),
    }
}

#[test]
fn declaration_admission_deferred_stays_distinct_on_composed_lane() {
    let handle = admitted_handle();
    let checked = handle.orchestrate_declaration_with_contributions_checked(
        ForgeQueryContributionComposedOrchestrationInput::new(
            DeferredAdmissionContributionInput::new("face-admission-deferred"),
        ),
    );
    let ordinary = handle.orchestrate_declaration_with_contributions_outcome(
        ForgeQueryContributionComposedOrchestrationInput::new(
            DeferredAdmissionContributionInput::new("face-admission-deferred"),
        ),
    );

    assert!(matches!(
        checked,
        crate::contribution_composed_orchestration::ForgeQueryContributionComposedOrchestrationOutcome::Deferred(_)
    ));
    match ordinary {
        ForgeQueryOrdinaryOutcome::Deferred(posture) => {
            assert_eq!(
                posture.checked_topology().contribution_composed_kind(),
                Some(
                    crate::ordinary_outcome::ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::Deferred
                )
            );
        }
        _ => panic!("expected ordinary deferred posture"),
    }
}
