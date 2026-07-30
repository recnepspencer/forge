use crate::runtime::observation::UiChangeClassificationOutcome;

fn evidence_plan(
    session: &mut crate::facade::WorthUiActiveApplicationSession,
) -> crate::runtime::rebind::UiRebindPlan {
    let candidate = crate::runtime::tests::active_application_session_test_support::
        component_candidate_submission(
            session,
            "phase-313-effecting-plan",
            "workspace.component.active_session_current",
        );
    let mut turn = session.begin_observation_turn().unwrap();
    turn.admit_source(candidate).unwrap();
    let admitted = turn.seal().unwrap();
    let evidence = match session.classify_observations(admitted).unwrap() {
        UiChangeClassificationOutcome::EvidenceOnly(evidence) => evidence,
        _ => panic!("equal semantics with new evidence stays evidence-only"),
    };
    session
        .compile_preservation_rebind(
            evidence,
            crate::runtime::rebind::UiRebindExecutionPolicy::ordinary(),
        )
        .unwrap()
}

fn admitted_source_sets(
    session: &mut crate::facade::WorthUiActiveApplicationSession,
    count: usize,
) -> Vec<crate::runtime::observation::UiAdmittedObservationSet> {
    (0..count)
        .map(|index| {
            let candidate = crate::runtime::tests::active_application_session_test_support::
                component_candidate_submission(
                    session,
                    &format!("phase-313-effecting-queued-{index}"),
                    "workspace.component.active_session_current",
                );
            let mut turn = session.begin_observation_turn().unwrap();
            turn.admit_source(candidate).unwrap();
            turn.seal().unwrap()
        })
        .collect()
}

#[test]
fn effecting_queue_accepts_sixteen_observations_and_returns_the_seventeenth() {
    let mut session = crate::runtime::tests::active_application_session_test_support::
        source_backed_component_session();
    let plan = evidence_plan(&mut session);
    let mut observations = admitted_source_sets(&mut session, 17).into_iter();
    let prepared = session
        .prepare_rebind(
            plan,
            crate::runtime::rebind::UiRebindExecutionRequest::new(1),
        )
        .unwrap();
    let mut effecting = match prepared.begin_effecting() {
        Ok(effecting) => effecting,
        Err(_) => panic!("prepared rebind owns the only effecting reservation"),
    };
    assert_eq!(effecting.queued_observation_count(), 0);

    for expected in 1..=15 {
        let receipt = match effecting.admit_observations(observations.next().unwrap()) {
            Ok(receipt) => receipt,
            Err(_) => panic!("the first fifteen observations fit the configured queue"),
        };
        assert_eq!(receipt.admitted_observations(), 1);
        assert_eq!(receipt.total_queued_observations(), expected);
        assert_eq!(receipt.remaining_capacity(), 16 - expected);
    }
    let sixteenth = match effecting.admit_observations(observations.next().unwrap()) {
        Ok(receipt) => receipt,
        Err(_) => panic!("the sixteenth observation exactly fills the configured queue"),
    };
    assert_eq!(sixteenth.total_queued_observations(), 16);
    assert_eq!(sixteenth.remaining_capacity(), 0);

    let rejected = observations.next().unwrap();
    let rejected_turn = rejected.turn();
    let stop = effecting.admit_observations(rejected).unwrap_err();
    assert_eq!(stop.configured(), 16);
    assert_eq!(stop.observed(), 16);
    assert_eq!(stop.attempted(), 17);
    let returned = stop.into_observation_set();
    assert_eq!(returned.turn(), rejected_turn);

    let completion = effecting.complete(1);
    assert_eq!(completion.queued_observations().len(), 16);
    let (outcome, queued) = completion.into_parts();
    assert!(matches!(
        outcome,
        crate::runtime::rebind::UiRebindOutcome::Published(_)
    ));
    drop(outcome);
    drop(queued);
    drop(returned);
    assert!(session.shutdown().rebind().is_empty());
}

#[test]
fn dropping_effecting_rebind_releases_its_reservation() {
    let mut session = crate::runtime::tests::active_application_session_test_support::
        source_backed_component_session();
    let plan = evidence_plan(&mut session);
    let prepared = session
        .prepare_rebind(
            plan,
            crate::runtime::rebind::UiRebindExecutionRequest::new(1),
        )
        .unwrap();
    let effecting = match prepared.begin_effecting() {
        Ok(effecting) => effecting,
        Err(_) => panic!("prepared rebind owns the only effecting reservation"),
    };
    assert_eq!(effecting.queued_observation_count(), 0);
    drop(effecting);
    assert!(session.shutdown().rebind().is_empty());
}
