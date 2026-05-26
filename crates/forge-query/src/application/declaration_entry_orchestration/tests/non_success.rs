use crate::application::{
    ForgeQueryDeclarationEntryOrchestrationChecked,
    ForgeQueryDeclarationEntryOrchestrationRefusalClass,
    ForgeQueryDeclarationEntryOrchestrationStage,
    ForgeQueryDeclarationEntryOrchestrationTerminalError,
};

use super::domain::{
    admitted_handle, DeferredFamily, DeferredRouteFamily, DeniedFamily, ExplicitIntentFamily,
    FailedFamily, Input, StaleFamily, UnsupportedReceiptFamily, WorldSensitiveFamily,
};

#[test]
fn progression_non_success_posture_stays_typed() {
    let handle = admitted_handle("collaborative");

    assert!(matches!(
        handle.orchestrate_declaration_entry_checked(Input::<DeferredFamily>::new("edge:42")),
        ForgeQueryDeclarationEntryOrchestrationChecked::Deferred(_)
    ));
    assert!(matches!(
        handle.orchestrate_declaration_entry_checked(Input::<DeniedFamily>::new("edge:42")),
        ForgeQueryDeclarationEntryOrchestrationChecked::Denied(_)
    ));
    assert!(matches!(
        handle.orchestrate_declaration_entry_checked(Input::<FailedFamily>::new("edge:42")),
        ForgeQueryDeclarationEntryOrchestrationChecked::Failed(_)
    ));
}

#[test]
fn stale_and_rebind_required_remain_distinct() {
    let collaborative = admitted_handle("collaborative");
    let restricted = admitted_handle("restricted");

    assert!(matches!(
        collaborative.orchestrate_declaration_entry_checked(Input::<StaleFamily>::new("edge:42")),
        ForgeQueryDeclarationEntryOrchestrationChecked::Stale(_)
    ));
    assert!(matches!(
        restricted
            .orchestrate_declaration_entry_checked(Input::<WorldSensitiveFamily>::new("edge:42")),
        ForgeQueryDeclarationEntryOrchestrationChecked::RebindRequired(_)
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
        ForgeQueryDeclarationEntryOrchestrationChecked::Refused(refusal) => {
            assert_eq!(
                refusal.refusal_class(),
                ForgeQueryDeclarationEntryOrchestrationRefusalClass::ExplicitIntentRequired
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
        ForgeQueryDeclarationEntryOrchestrationChecked::Refused(refusal) => {
            assert_eq!(
                refusal.refusal_class(),
                ForgeQueryDeclarationEntryOrchestrationRefusalClass::UnsupportedAutomation
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
fn deferred_route_lowering_records_receipt_stop_honestly() {
    let handle = admitted_handle("collaborative");
    let checked =
        handle.orchestrate_declaration_entry_checked(Input::<DeferredRouteFamily>::new("edge:42"));
    let proof =
        handle.orchestrate_declaration_entry_proof(Input::<DeferredRouteFamily>::new("edge:42"));

    match checked {
        ForgeQueryDeclarationEntryOrchestrationChecked::Deferred(outcome) => {
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
fn refusal_taxonomy_remains_publicly_nameable_even_before_all_classes_are_reached() {
    let classes = [
        ForgeQueryDeclarationEntryOrchestrationRefusalClass::UnsupportedAutomation,
        ForgeQueryDeclarationEntryOrchestrationRefusalClass::ExplicitIntentRequired,
        ForgeQueryDeclarationEntryOrchestrationRefusalClass::StrongerProofRequired,
        ForgeQueryDeclarationEntryOrchestrationRefusalClass::AuthorityTransitionRequired,
        ForgeQueryDeclarationEntryOrchestrationRefusalClass::ExpensiveWorkNotAdmittedByDefault,
        ForgeQueryDeclarationEntryOrchestrationRefusalClass::PreparedButNotExecutedContinuation,
    ];

    let labels = classes.map(|class| class.as_str());
    assert_eq!(
        labels,
        [
            "unsupported_automation",
            "explicit_intent_required",
            "stronger_proof_required",
            "authority_transition_required",
            "expensive_work_not_admitted_by_default",
            "prepared_but_not_executed_continuation",
        ]
    );
}
