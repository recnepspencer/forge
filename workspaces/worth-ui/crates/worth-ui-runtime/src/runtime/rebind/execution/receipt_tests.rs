use super::receipt::UiRebindReceipt;
use crate::runtime::observation::UiChangeClassificationOutcome;

#[test]
fn planned_realized_mismatch_is_a_recoverable_internal_defect() {
    let mut session = crate::runtime::tests::active_application_session_test_support::
        source_backed_component_session();
    let foreign = crate::runtime::tests::active_application_session_test_support::
        source_backed_component_session();
    let candidate = crate::runtime::tests::active_application_session_test_support::
        component_candidate_submission(
            &session,
            "phase-312-receipt-mismatch",
            "workspace.component.active_session_current",
        );
    let mut turn = session.begin_observation_turn().unwrap();
    turn.admit_source(candidate).unwrap();
    let admitted = turn.seal().unwrap();
    let evidence = match session.classify_observations(admitted).unwrap() {
        UiChangeClassificationOutcome::EvidenceOnly(evidence) => evidence,
        _ => panic!("equal semantics with fresh evidence stays evidence-only"),
    };
    let plan = session
        .compile_preservation_rebind(
            evidence,
            crate::runtime::rebind::UiRebindExecutionPolicy::ordinary(),
        )
        .unwrap();
    let prior = plan
        .basis()
        .classification()
        .predecessor_generation()
        .clone();
    let state =
        super::UiRebindRuntimeState::new(crate::runtime::rebind::UiRebindProfile::platform_pulse());
    let basis = super::UiRebindFinalAdmissionBasis::new(
        plan.basis().classification().session(),
        plan.basis().classification().source_basis(),
        plan.basis().classification().predecessor_generation(),
    );
    let reservation = super::admit_plan(
        &state,
        basis,
        &plan,
        crate::runtime::rebind::UiRebindExecutionRequest::new(1),
    )
    .expect("current plan reserves terminal capacity");

    let defect = match UiRebindReceipt::evidence_only(
        plan,
        reservation,
        prior,
        foreign.generation_identity().clone(),
    ) {
        Err(defect) => defect,
        Ok(receipt) => {
            drop(receipt);
            panic!("crossed realized generation must not become a receipt");
        }
    };
    assert_eq!(
        defect.kind(),
        super::UiRebindInternalDefectKind::PlannedRealizedMismatch
    );
    assert!(defect.publication_occurred());
    assert!(defect.retains_recovery_authority());
    assert_eq!(
        defect.valid_next_action(),
        super::UiRebindValidNextAction::ReportDefect
    );
    drop(defect);
    assert!(state.shutdown().is_empty());
    assert!(session.shutdown().rebind().is_empty());
    assert!(foreign.shutdown().rebind().is_empty());
}
