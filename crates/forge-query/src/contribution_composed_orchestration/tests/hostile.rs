use crate::contribution_composed_orchestration::{
    ForgeQueryContributionComposedClassification, ForgeQueryContributionComposedOrchestrationInput,
    ForgeQueryContributionIntent,
};
use crate::domain_capabilities::{
    ForgeQueryContinuityContributionAuthoring, ForgeQuerySupportContributionAuthoring,
};
use crate::ordinary_outcome::ForgeQueryOrdinaryOutcome;

use super::support::{
    admitted_handle, ContributionInput, DeferredAdmissionContributionInput,
    DeferredContributionInput,
};

#[test]
fn partial_composition_preserves_admitted_truth_when_later_intent_denies() {
    let handle = admitted_handle();
    let checked = handle.orchestrate_declaration_with_contributions_checked(
        ForgeQueryContributionComposedOrchestrationInput::new(ContributionInput::new("face-b"))
            .with_contribution(ForgeQueryContributionIntent::support(
                ForgeQuerySupportContributionAuthoring::declaration_traceability(
                    "domain.traceability.face",
                    "face selection remains traceable through declaration entry",
                ),
            ))
            .with_contribution(ForgeQueryContributionIntent::support(
                ForgeQuerySupportContributionAuthoring::declaration_traceability(
                    "domain.traceability.face",
                    "",
                ),
            )),
    );

    match checked {
        crate::contribution_composed_orchestration::ForgeQueryContributionComposedOrchestrationOutcome::Bound(
            composed,
        ) => {
            assert_eq!(
                composed.classification(),
                ForgeQueryContributionComposedClassification::PartiallyAdmitted
            );
            assert_eq!(composed.contributions().len(), 1);
            assert_eq!(composed.rejected_intents().len(), 1);
            assert_eq!(
                composed
                    .composition()
                    .rejected_category_families()
                    .len(),
                1
            );
            assert_eq!(
                composed.contributions()[0].evidence().evidence_digest(),
                composed.contribution_composition().evidence()[0].evidence_digest()
            );
        }
        _ => panic!("expected partial composition to remain inspectable as a bound artifact"),
    }
}

#[test]
fn no_admitted_contributions_stay_contribution_owned_denial() {
    let handle = admitted_handle();
    let checked = handle.orchestrate_declaration_with_contributions_checked(
        ForgeQueryContributionComposedOrchestrationInput::new(ContributionInput::new("face-c"))
            .with_contribution(ForgeQueryContributionIntent::support(
                ForgeQuerySupportContributionAuthoring::declaration_traceability(
                    "domain.traceability.face",
                    "",
                ),
            )),
    );
    let ordinary = handle.orchestrate_declaration_with_contributions_outcome(
        ForgeQueryContributionComposedOrchestrationInput::new(ContributionInput::new("face-c"))
            .with_contribution(ForgeQueryContributionIntent::support(
                ForgeQuerySupportContributionAuthoring::declaration_traceability(
                    "domain.traceability.face",
                    "",
                ),
            )),
    );

    assert!(matches!(
        checked,
        crate::contribution_composed_orchestration::ForgeQueryContributionComposedOrchestrationOutcome::ContributionDenied(_)
    ));
    match ordinary {
        ForgeQueryOrdinaryOutcome::Denied(posture) => {
            assert_eq!(
                posture.checked_topology().contribution_composed_kind(),
                Some(
                    crate::ordinary_outcome::ForgeQueryOrdinaryContributionComposedCheckedTopologyKind::ContributionDenied
                )
            );
        }
        _ => panic!("expected ordinary denial"),
    }
}

#[test]
fn declaration_deferred_stays_distinct_on_composed_lane() {
    let handle = admitted_handle();
    let checked = handle.orchestrate_declaration_with_contributions_checked(
        ForgeQueryContributionComposedOrchestrationInput::new(DeferredContributionInput::new(
            "face-deferred",
        ))
        .with_contribution(ForgeQueryContributionIntent::support(
            ForgeQuerySupportContributionAuthoring::declaration_traceability(
                "domain.traceability.face",
                "deferred declaration still carries an attached contribution request",
            ),
        )),
    );
    let ordinary = handle.orchestrate_declaration_with_contributions_outcome(
        ForgeQueryContributionComposedOrchestrationInput::new(DeferredContributionInput::new(
            "face-deferred",
        ))
        .with_contribution(ForgeQueryContributionIntent::support(
            ForgeQuerySupportContributionAuthoring::declaration_traceability(
                "domain.traceability.face",
                "deferred declaration still carries an attached contribution request",
            ),
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
        )
        .with_contribution(ForgeQueryContributionIntent::support(
            ForgeQuerySupportContributionAuthoring::declaration_traceability(
                "domain.traceability.face",
                "admission-deferred declaration still carries an attached contribution request",
            ),
        )),
    );
    let ordinary = handle.orchestrate_declaration_with_contributions_outcome(
        ForgeQueryContributionComposedOrchestrationInput::new(
            DeferredAdmissionContributionInput::new("face-admission-deferred"),
        )
        .with_contribution(ForgeQueryContributionIntent::support(
            ForgeQuerySupportContributionAuthoring::declaration_traceability(
                "domain.traceability.face",
                "admission-deferred declaration still carries an attached contribution request",
            ),
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
fn proof_transcript_explains_partial_run() {
    let handle = admitted_handle();
    let proof = handle.orchestrate_declaration_with_contributions_proof(
        ForgeQueryContributionComposedOrchestrationInput::new(ContributionInput::new("face-proof"))
            .with_contribution(ForgeQueryContributionIntent::support(
                ForgeQuerySupportContributionAuthoring::declaration_traceability(
                    "domain.traceability.face",
                    "proof-visible contribution",
                ),
            ))
            .with_contribution(ForgeQueryContributionIntent::support(
                ForgeQuerySupportContributionAuthoring::declaration_traceability(
                    "domain.traceability.face",
                    "",
                ),
            )),
    );

    assert_eq!(proof.intent_results().len(), 2);
    assert_eq!(
        proof.composition_classification(),
        Some(ForgeQueryContributionComposedClassification::PartiallyAdmitted)
    );
    assert!(proof.declaration().envelope_digest().is_some());
    assert!(proof.declaration().aspect_contract().is_some());
    assert!(proof.declaration().aspect_publication().is_some());
    assert!(proof.contribution_digest().is_some());
    assert_eq!(
        proof.intent_results()[0].semantic_code(),
        "domain.traceability.face"
    );
    assert_eq!(
        proof.intent_results()[0].detail(),
        "proof-visible contribution"
    );
    assert_eq!(
        proof.intent_results()[0]
            .aspect_record()
            .declaration_contract()
            .required(),
        &[
            "selection.active_face".to_string(),
            "selection.face".to_string()
        ]
    );
    assert_eq!(
        proof.intent_results()[0]
            .aspect_record()
            .declaration_coverage()
            .present(),
        proof
            .declaration()
            .aspect_publication()
            .expect("declaration aspect publication should be retained")
            .present()
    );
}

#[test]
fn empty_contribution_request_is_rejected_as_unsupported() {
    let handle = admitted_handle();
    let checked = handle.orchestrate_declaration_with_contributions_checked(
        ForgeQueryContributionComposedOrchestrationInput::new(ContributionInput::new("face-empty")),
    );

    assert!(matches!(
        checked,
        crate::contribution_composed_orchestration::ForgeQueryContributionComposedOrchestrationOutcome::Unsupported(_)
    ));
}

#[test]
fn declaration_bound_continuity_is_admitted_on_contribution_composed_lane() {
    let handle = admitted_handle();
    let composed = match handle.orchestrate_declaration_with_contributions(
        ForgeQueryContributionComposedOrchestrationInput::new(ContributionInput::new(
            "face-continuity",
        ))
        .with_contribution(ForgeQueryContributionIntent::continuity(
            ForgeQueryContinuityContributionAuthoring::preserved(
                "domain.continuity.face",
                "declaration retains canonical face continuity",
            ),
        )),
    ) {
        Ok(value) => value,
        Err(_) => panic!("expected declaration-bound continuity contribution to admit"),
    };

    assert_eq!(
        composed.classification(),
        ForgeQueryContributionComposedClassification::FullyAdmitted
    );
    assert_eq!(composed.contributions().len(), 1);
    assert_eq!(
        composed.contribution_composition().composed_category_families(),
        &[crate::application::ForgeQueryDeclarationEntryContributionCategoryFamily::ContinuityLineage]
    );
    assert_eq!(
        composed.intent_results()[0].semantic_code(),
        "domain.continuity.face"
    );
}
