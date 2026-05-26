use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass,
    ForgeQueryDeclarationEntryOrchestrationOutcome,
    ForgeQueryDeclarationEntryOrchestrationRefusalClass,
    ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEntryOrchestrationStepDisposition,
    ForgeQueryDeclarationEntryOrchestrationTerminalError,
};

use super::domain::{
    admitted_handle, DeferredFamily, DeferredRouteFamily, DeniedFamily, ExpensiveAutomationFamily,
    ExplicitIntentFamily, FailedFamily, Input, StaleFamily, UnsupportedReceiptFamily,
    WorldSensitiveFamily,
};

#[test]
fn progression_non_success_posture_stays_typed() {
    let handle = admitted_handle("collaborative");

    assert!(matches!(
        handle.orchestrate_declaration_entry_checked(Input::<DeferredFamily>::new("edge:42")),
        ForgeQueryDeclarationEntryOrchestrationOutcome::Deferred(_)
    ));
    assert!(matches!(
        handle.orchestrate_declaration_entry_checked(Input::<DeniedFamily>::new("edge:42")),
        ForgeQueryDeclarationEntryOrchestrationOutcome::Denied(_)
    ));
    assert!(matches!(
        handle.orchestrate_declaration_entry_checked(Input::<FailedFamily>::new("edge:42")),
        ForgeQueryDeclarationEntryOrchestrationOutcome::Failed(_)
    ));
}

#[test]
fn stale_and_rebind_required_remain_distinct() {
    let collaborative = admitted_handle("collaborative");
    let restricted = admitted_handle("restricted");

    assert!(matches!(
        collaborative.orchestrate_declaration_entry_checked(Input::<StaleFamily>::new("edge:42")),
        ForgeQueryDeclarationEntryOrchestrationOutcome::Stale(_)
    ));
    assert!(matches!(
        restricted
            .orchestrate_declaration_entry_checked(Input::<WorldSensitiveFamily>::new("edge:42")),
        ForgeQueryDeclarationEntryOrchestrationOutcome::RebindRequired(_)
    ));
}

#[test]
fn intent_required_route_is_refused_without_hidden_route_lowering() {
    let handle = admitted_handle("collaborative");
    let checked =
        handle.orchestrate_declaration_entry_checked(Input::<ExplicitIntentFamily>::new("edge:42"));
    let proof =
        handle.orchestrate_declaration_entry_proof(Input::<ExplicitIntentFamily>::new("edge:42"));

    match checked {
        ForgeQueryDeclarationEntryOrchestrationOutcome::Refused(refusal) => {
            assert_eq!(
                refusal.refusal_class(),
                ForgeQueryDeclarationEntryOrchestrationRefusalClass::ExplicitIntentRequired
            );
            assert_eq!(
                refusal.automation_refusal_class(),
                ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass::ExplicitIntentRequired
            );
            assert_eq!(
                refusal.stop_stage(),
                ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned
            );
        }
        _ => panic!("expected explicit-intent refusal"),
    }

    let last = proof
        .stage_records()
        .last()
        .expect("stop stage should exist");
    assert_eq!(
        last.stage(),
        ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned
    );
    assert!(last.is_stop());
}

#[test]
fn unsupported_receipt_kind_is_refused_without_hidden_continuation() {
    let handle = admitted_handle("collaborative");
    let checked = handle
        .orchestrate_declaration_entry_checked(Input::<UnsupportedReceiptFamily>::new("edge:42"));

    match checked {
        ForgeQueryDeclarationEntryOrchestrationOutcome::Refused(refusal) => {
            assert_eq!(
                refusal.refusal_class(),
                ForgeQueryDeclarationEntryOrchestrationRefusalClass::UnsupportedAutomation
            );
            assert_eq!(
                refusal.automation_refusal_class(),
                ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass::UnsupportedAutomation
            );
            assert_eq!(
                refusal.stop_stage(),
                ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued
            );
        }
        _ => panic!("expected unsupported-automation refusal"),
    }
}

#[test]
fn expensive_but_legal_declaration_refuses_automation_at_route() {
    let handle = admitted_handle("collaborative");
    let checked = handle
        .orchestrate_declaration_entry_checked(Input::<ExpensiveAutomationFamily>::new("edge:42"));
    let proof = handle
        .orchestrate_declaration_entry_proof(Input::<ExpensiveAutomationFamily>::new("edge:42"));

    match checked {
        ForgeQueryDeclarationEntryOrchestrationOutcome::Refused(refusal) => {
            assert_eq!(
                refusal.refusal_class(),
                ForgeQueryDeclarationEntryOrchestrationRefusalClass::ExpensiveWorkNotAdmittedByDefault
            );
            assert_eq!(
                refusal.automation_refusal_class(),
                ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass::ExpensiveAutomationForbidden
            );
            assert_eq!(
                refusal.stop_stage(),
                ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned
            );
        }
        _ => panic!("expected expensive automation refusal"),
    }

    let last = proof
        .stage_records()
        .last()
        .expect("stop stage should exist");
    assert_eq!(
        last.stage(),
        ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned
    );
    assert_eq!(
        last.disposition(),
        ForgeQueryDeclarationEntryOrchestrationStepDisposition::ExplicitForCaller
    );
}

#[test]
fn expensive_route_can_still_continue_on_the_explicit_path() {
    let handle = admitted_handle("collaborative");

    let envelope = handle
        .declare_review_progress_describe_plan_receipt_and_envelope(Input::<
            ExpensiveAutomationFamily,
        >::new("edge:42"))
        .unwrap_or_else(|_| panic!("explicit lowering should still envelope"));

    assert_eq!(
        envelope.declaration_family_key(),
        "ExpensiveAutomationFamily"
    );
}

#[test]
fn deferred_route_lowering_records_receipt_stop_honestly() {
    let handle = admitted_handle("collaborative");
    let checked =
        handle.orchestrate_declaration_entry_checked(Input::<DeferredRouteFamily>::new("edge:42"));
    let proof =
        handle.orchestrate_declaration_entry_proof(Input::<DeferredRouteFamily>::new("edge:42"));

    match checked {
        ForgeQueryDeclarationEntryOrchestrationOutcome::Deferred(outcome) => {
            assert_eq!(
                outcome.stop_stage(),
                ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued
            );
        }
        _ => panic!("expected deferred route outcome"),
    }

    let route = proof
        .stage_records()
        .iter()
        .find(|record| record.stage() == ForgeQueryDeclarationEntryOrchestrationStage::RoutePlanned)
        .expect("route stage should exist");
    assert!(route.is_reached());

    let last = proof
        .stage_records()
        .last()
        .expect("receipt stop should exist");
    assert_eq!(
        last.stage(),
        ForgeQueryDeclarationEntryOrchestrationStage::ReceiptIssued
    );
    assert!(last.is_stop());
}

#[test]
fn ordinary_surface_preserves_terminal_error_classes() {
    let handle = admitted_handle("restricted");

    assert!(matches!(
        handle.orchestrate_declaration_entry(Input::<WorldSensitiveFamily>::new("edge:42")),
        Err(ForgeQueryDeclarationEntryOrchestrationTerminalError::RebindRequired(_))
    ));
    assert!(matches!(
        handle.orchestrate_declaration_entry(Input::<ExplicitIntentFamily>::new("edge:42")),
        Err(ForgeQueryDeclarationEntryOrchestrationTerminalError::Refused(_))
    ));
}

#[test]
fn refusal_taxonomies_remain_publicly_nameable_even_before_all_classes_are_reached() {
    let broad_classes = [
        ForgeQueryDeclarationEntryOrchestrationRefusalClass::UnsupportedAutomation,
        ForgeQueryDeclarationEntryOrchestrationRefusalClass::ExplicitIntentRequired,
        ForgeQueryDeclarationEntryOrchestrationRefusalClass::StrongerProofRequired,
        ForgeQueryDeclarationEntryOrchestrationRefusalClass::AuthorityTransitionRequired,
        ForgeQueryDeclarationEntryOrchestrationRefusalClass::ExpensiveWorkNotAdmittedByDefault,
        ForgeQueryDeclarationEntryOrchestrationRefusalClass::PreparedButNotExecutedContinuation,
    ];
    let automation_classes = [
        ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass::ExplicitIntentRequired,
        ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass::ExpensiveAutomationForbidden,
        ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass::AuthorityTransitionRequired,
        ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass::PreparedButNotExecuted,
        ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass::UnsupportedAutomation,
        ForgeQueryDeclarationEntryOrchestrationAutomationRefusalClass::StrongerProofRequired,
    ];

    let broad_labels = broad_classes.map(|class| class.as_str());
    let automation_labels = automation_classes.map(|class| class.as_str());
    assert_eq!(
        broad_labels,
        [
            "unsupported_automation",
            "explicit_intent_required",
            "stronger_proof_required",
            "authority_transition_required",
            "expensive_work_not_admitted_by_default",
            "prepared_but_not_executed_continuation",
        ]
    );
    assert_eq!(
        automation_labels,
        [
            "explicit_intent_required",
            "expensive_automation_forbidden",
            "authority_transition_required",
            "prepared_but_not_executed",
            "unsupported_automation",
            "stronger_proof_required",
        ]
    );
}
