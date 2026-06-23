use crate::capability::{CommandId, CommandProjectionId, ThemeTokenId};
use crate::runtime::{
    WorthUiCapabilityReloadEvidence, WorthUiCapabilityReloadStage, WorthUiCapabilityReloadStatus,
    WorthUiDropdownSelectionInteractionReceipt, WorthUiDropdownSelectionInteractionStatus,
    WorthUiDropdownSelectionState, WorthUiRuntimeFactId, WorthUiRuntimeFactSet,
    WorthUiValidationReloadEvidence, WorthUiValidationReloadStatus,
};

use super::{
    WorthUiAdmittedRuntimeChangeEvidence, WorthUiClassifiedRuntimeChange,
    WorthUiRuntimeChangeActivationPosture, WorthUiRuntimeChangeAdmissionDenial,
    WorthUiRuntimeChangeFamily, WorthUiRuntimeChangeFamilyRow, WorthUiRuntimeChangeFamilyStatus,
    WorthUiRuntimeInstanceWitness,
};

#[test]
fn activated_common_evidence_rejects_empty_changed_facts() {
    let evidence = WorthUiCapabilityReloadEvidence::prepared(
        7,
        WorthUiCapabilityReloadStatus::ReadyForFrameBoundary,
        10,
        11,
        29,
        0,
        0,
        0,
        WorthUiRuntimeFactSet::empty(),
    )
    .mark_activated(11);
    let classified = WorthUiClassifiedRuntimeChange::from_capability_reload(&evidence);

    let denial = WorthUiAdmittedRuntimeChangeEvidence::admit(
        classified,
        WorthUiRuntimeInstanceWitness::from_raw(7),
    )
    .expect_err("activated rows without proof-bearing facts must not enter rebind evidence");

    assert_eq!(
        denial,
        WorthUiRuntimeChangeAdmissionDenial::ActivatedFamilyWithoutChangedFacts
    );
}

#[test]
fn denied_common_evidence_accepts_empty_changed_facts() {
    let evidence = capability_denied(7, "malformed theme");
    let classified = WorthUiClassifiedRuntimeChange::from_capability_reload(&evidence);

    let admitted = WorthUiAdmittedRuntimeChangeEvidence::admit(
        classified,
        WorthUiRuntimeInstanceWitness::from_raw(7),
    )
    .expect("denials may carry no changed facts");

    assert_eq!(
        admitted.posture(),
        WorthUiRuntimeChangeActivationPosture::Denied
    );
    assert_eq!(admitted.counters().changed_fact_count(), 0);
    assert_eq!(admitted.counters().denied_family_count(), 1);
}

#[test]
fn equivalent_common_evidence_accepts_empty_changed_facts() {
    let evidence = WorthUiCapabilityReloadEvidence::prepared(
        7,
        WorthUiCapabilityReloadStatus::EquivalentNoOp,
        10,
        10,
        29,
        0,
        0,
        0,
        WorthUiRuntimeFactSet::empty(),
    );
    let classified = WorthUiClassifiedRuntimeChange::from_capability_reload(&evidence);

    let admitted = WorthUiAdmittedRuntimeChangeEvidence::admit(
        classified,
        WorthUiRuntimeInstanceWitness::from_raw(7),
    )
    .expect("equivalent rows may carry no changed facts");

    assert_eq!(
        admitted.posture(),
        WorthUiRuntimeChangeActivationPosture::EquivalentNoOp
    );
    assert_eq!(admitted.counters().changed_fact_count(), 0);
}

#[test]
fn mixed_source_and_capability_rows_preserve_family_status() {
    let source = validation_ready(7).mark_activated(21, 22);
    let capability = capability_denied(7, "bad token");
    let classified = WorthUiClassifiedRuntimeChange::from_rows(vec![
        WorthUiRuntimeChangeFamilyRow::from_validation_evidence(&source),
        WorthUiRuntimeChangeFamilyRow::from_capability_evidence(&capability),
    ])
    .expect("same runtime instance can classify mixed rows");

    let admitted = WorthUiAdmittedRuntimeChangeEvidence::admit(
        classified,
        WorthUiRuntimeInstanceWitness::from_raw(7),
    )
    .expect("activated source row has changed facts");

    let WorthUiRuntimeChangeActivationPosture::Mixed(mixed) = admitted.posture() else {
        panic!("mixed source/capability rows must stay visibly mixed");
    };
    assert_eq!(mixed.activated_family_count(), 1);
    assert_eq!(mixed.denied_family_count(), 1);
    assert_eq!(
        admitted.family_rows()[0].family(),
        WorthUiRuntimeChangeFamily::ValidationSource
    );
    assert_eq!(
        admitted.family_rows()[0].status(),
        WorthUiRuntimeChangeFamilyStatus::Activated
    );
    assert_eq!(
        admitted.family_rows()[1].family(),
        WorthUiRuntimeChangeFamily::Capability
    );
    assert_eq!(
        admitted.family_rows()[1].status(),
        WorthUiRuntimeChangeFamilyStatus::Denied
    );
}

#[test]
fn stale_runtime_witness_cannot_admit_common_evidence() {
    let evidence = capability_ready(7, theme_fact("validation.theme.header.panel"));
    let classified = WorthUiClassifiedRuntimeChange::from_capability_reload(&evidence);

    let denial = WorthUiAdmittedRuntimeChangeEvidence::admit(
        classified,
        WorthUiRuntimeInstanceWitness::from_raw(8),
    )
    .expect_err("foreign witnesses must not admit classified rows");

    assert_eq!(
        denial,
        WorthUiRuntimeChangeAdmissionDenial::RuntimeInstanceMismatch
    );
}

#[test]
fn common_evidence_digest_changes_with_denial_payload_or_changed_facts() {
    let denied_left = admitted_capability_denial("malformed theme");
    let denied_right = admitted_capability_denial("unknown theme");
    let theme_left = admitted_capability_activation("validation.theme.header.panel");
    let theme_right = admitted_capability_activation("validation.theme.header.menu");

    assert_ne!(denied_left.digest(), denied_right.digest());
    assert_ne!(theme_left.digest(), theme_right.digest());
    assert_ne!(denied_left.digest(), theme_left.digest());
}

#[test]
fn dropdown_selection_interaction_activates_only_the_touched_projection_fact() {
    let projection_id = CommandProjectionId::new("workspace.header.file").unwrap();
    let command_id = CommandId::new("workspace.command.save").unwrap();
    let receipt = WorthUiDropdownSelectionInteractionReceipt::new(
        &projection_id,
        &command_id,
        WorthUiDropdownSelectionState::None,
        WorthUiDropdownSelectionState::Single(command_id.as_str().to_owned()),
        WorthUiDropdownSelectionInteractionStatus::SelectedSingle,
    );
    let classified = WorthUiClassifiedRuntimeChange::from_dropdown_selection_interaction(
        WorthUiRuntimeInstanceWitness::from_raw(7),
        &receipt,
    );

    let admitted = WorthUiAdmittedRuntimeChangeEvidence::admit(
        classified,
        WorthUiRuntimeInstanceWitness::from_raw(7),
    )
    .expect("changed selection fact should admit interaction change");

    assert_eq!(
        receipt.status(),
        &WorthUiDropdownSelectionInteractionStatus::SelectedSingle
    );
    assert_eq!(
        admitted.posture(),
        WorthUiRuntimeChangeActivationPosture::Activated
    );
    assert_eq!(admitted.counters().changed_fact_count(), 1);
    assert_eq!(
        admitted.family_rows()[0].changed_facts().facts(),
        &WorthUiRuntimeFactSet::single(WorthUiRuntimeFactId::dropdown_selection_state(
            &projection_id,
        ))
    );
}

#[test]
fn already_selected_dropdown_interaction_stays_equivalent_without_changed_facts() {
    let projection_id = CommandProjectionId::new("workspace.header.file").unwrap();
    let command_id = CommandId::new("workspace.command.save").unwrap();
    let selected_state = WorthUiDropdownSelectionState::Single(command_id.as_str().to_owned());
    let receipt = WorthUiDropdownSelectionInteractionReceipt::new(
        &projection_id,
        &command_id,
        selected_state.clone(),
        selected_state,
        WorthUiDropdownSelectionInteractionStatus::AlreadySelected,
    );
    let classified = WorthUiClassifiedRuntimeChange::from_dropdown_selection_interaction(
        WorthUiRuntimeInstanceWitness::from_raw(7),
        &receipt,
    );

    let admitted = WorthUiAdmittedRuntimeChangeEvidence::admit(
        classified,
        WorthUiRuntimeInstanceWitness::from_raw(7),
    )
    .expect("equivalent interaction may admit without changed facts");

    assert_eq!(
        receipt.status(),
        &WorthUiDropdownSelectionInteractionStatus::AlreadySelected
    );
    assert_eq!(
        admitted.posture(),
        WorthUiRuntimeChangeActivationPosture::EquivalentNoOp
    );
    assert_eq!(admitted.counters().changed_fact_count(), 0);
}

fn admitted_capability_denial(detail: &str) -> WorthUiAdmittedRuntimeChangeEvidence {
    let classified =
        WorthUiClassifiedRuntimeChange::from_capability_reload(&capability_denied(7, detail));
    WorthUiAdmittedRuntimeChangeEvidence::admit(
        classified,
        WorthUiRuntimeInstanceWitness::from_raw(7),
    )
    .expect("denied evidence should admit for diagnostics")
}

fn admitted_capability_activation(raw_theme_id: &str) -> WorthUiAdmittedRuntimeChangeEvidence {
    let evidence = capability_ready(7, theme_fact(raw_theme_id)).mark_activated(11);
    let classified = WorthUiClassifiedRuntimeChange::from_capability_reload(&evidence);
    WorthUiAdmittedRuntimeChangeEvidence::admit(
        classified,
        WorthUiRuntimeInstanceWitness::from_raw(7),
    )
    .expect("activated evidence should admit when changed facts are present")
}

fn validation_ready(runtime_instance: u64) -> WorthUiValidationReloadEvidence {
    WorthUiValidationReloadEvidence::builder(runtime_instance, 10, 11)
        .record_candidate_artifact(20)
        .record_candidate_plan(21)
        .finish(WorthUiValidationReloadStatus::ReadyForFrameBoundary, 10, 11)
}

fn capability_ready(
    runtime_instance: u64,
    changed_fact: WorthUiRuntimeFactId,
) -> WorthUiCapabilityReloadEvidence {
    WorthUiCapabilityReloadEvidence::prepared(
        runtime_instance,
        WorthUiCapabilityReloadStatus::ReadyForFrameBoundary,
        10,
        11,
        29,
        1,
        3,
        1,
        WorthUiRuntimeFactSet::single(changed_fact),
    )
}

fn capability_denied(runtime_instance: u64, detail: &str) -> WorthUiCapabilityReloadEvidence {
    WorthUiCapabilityReloadEvidence::denied(
        runtime_instance,
        10,
        29,
        WorthUiCapabilityReloadStage::ThemeTokenAdmission,
        detail,
    )
}

fn theme_fact(raw_id: &str) -> WorthUiRuntimeFactId {
    let token = ThemeTokenId::new(raw_id).expect("valid theme token id");
    WorthUiRuntimeFactId::theme_token(&token)
}
