use worth_ui_inspection::{UiInspectionQuery, UiInspectionScope, UiInspectionTarget};

use crate::runtime::tests::active_application_session_test_support::{
    admit_candidate_catalog, component_candidate_submission, source_backed_component_session,
};

#[test]
fn equivalent_replacement_reaches_lowering_and_can_be_discarded() {
    let session = source_backed_component_session();
    let generation = session.generation_identity().clone();
    let submission = component_candidate_submission(
        &session,
        "active-session-equivalent",
        "workspace.component.active_session_current",
    );
    let prepared = session
        .prepare_replacement(submission)
        .expect("equivalent candidate should admit");
    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("equivalent candidate must continue through lowering");
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("equivalent candidate must reach staged authority");
    drop(pending);
    assert_eq!(session.generation_identity(), &generation);
    assert_eq!(session.inspect_runtime().generation_identity(), &generation);
}

#[test]
fn prepared_replacement_cannot_cross_identical_active_sessions() {
    let first = source_backed_component_session();
    let second = source_backed_component_session();
    assert_eq!(first.generation_identity(), second.generation_identity());
    assert_eq!(
        first.inspect_runtime().artifact_digest(),
        second.inspect_runtime().artifact_digest()
    );
    let prepared = first
        .prepare_replacement(component_candidate_submission(
            &first,
            "active-session-cross-launch",
            "workspace.component.active_session_candidate",
        ))
        .expect("candidate should prepare against the first session");
    let denial = match second.lower_prepared_replacement(*prepared) {
        Ok(_) => panic!("replacement prepared by session A must deny in session B"),
        Err(denial) => denial,
    };
    assert!(matches!(
        denial,
        crate::facade::WorthUiApplicationReplacementLoweringDenial::ForeignActiveApplicationSession
    ));
}

#[test]
fn lowered_replacement_cannot_cross_identical_active_sessions() {
    let first = source_backed_component_session();
    let second = source_backed_component_session();
    let prepared = first
        .prepare_replacement(component_candidate_submission(
            &first,
            "active-session-cross-stage",
            "workspace.component.active_session_candidate",
        ))
        .expect("candidate should prepare against the first session");
    let lowered = first
        .lower_prepared_replacement(*prepared)
        .expect("origin session should lower its replacement");
    let denial = match second.stage_prepared_replacement(lowered) {
        Ok(_) => panic!("lowering from session A must not stage in session B"),
        Err(denial) => denial,
    };
    assert!(matches!(
        denial,
        crate::facade::WorthUiApplicationReplacementStagingDenial::ForeignActiveApplicationSession
    ));
}

#[test]
fn pending_cutover_cannot_cross_identical_active_sessions() {
    let first = source_backed_component_session();
    let mut second = source_backed_component_session();
    let first_generation = first.generation_identity().clone();
    let second_generation = second.generation_identity().clone();
    let first_host = first.host_session_identity();
    let second_host = second.host_session_identity();
    let mut prepared = first
        .prepare_replacement(component_candidate_submission(
            &first,
            "active-session-cross-cutover",
            "workspace.component.active_session_candidate",
        ))
        .expect("candidate should prepare against the first session");
    let catalog = admit_candidate_catalog(&mut prepared);
    let lowered = first
        .lower_prepared_replacement(*prepared)
        .expect("origin session should lower its replacement");
    let pending = first
        .stage_prepared_replacement(lowered)
        .expect("origin session should stage its replacement");
    let boundary = second
        .execute_framework_turn(|_| {})
        .into_completion()
        .into_execution()
        .expect("foreign session can produce only its own boundary")
        .into_activation_boundary();
    let denial = match second.activate_prepared_replacement(pending, catalog, boundary, None) {
        Ok(_) => panic!("pending cutover from session A must not activate in session B"),
        Err(denial) => denial,
    };
    assert!(matches!(
        denial,
        crate::facade::WorthUiApplicationCutoverDenial::ForeignActiveApplicationSession
    ));
    assert_eq!(first.generation_identity(), &first_generation);
    assert_eq!(second.generation_identity(), &second_generation);
    assert_eq!(first.host_session_identity(), first_host);
    assert_eq!(second.host_session_identity(), second_host);
}

#[test]
fn equal_looking_frame_boundary_cannot_cross_active_sessions() {
    let mut first = source_backed_component_session();
    let mut second = source_backed_component_session();
    assert_eq!(first.inspect_runtime(), second.inspect_runtime());
    let active_generation = first.generation_identity().clone();
    let mut prepared = first
        .prepare_replacement(component_candidate_submission(
            &first,
            "active-session-cross-boundary",
            "workspace.component.active_session_candidate",
        ))
        .expect("origin candidate prepares");
    let catalog = admit_candidate_catalog(&mut prepared);
    let successor_generation = prepared
        .inspect_candidate(UiInspectionQuery::new(
            UiInspectionTarget::product_root(),
            UiInspectionScope::graph(),
        ))
        .generation_identity()
        .clone();
    let lowered = first
        .lower_prepared_replacement(*prepared)
        .expect("origin candidate lowers");
    let pending = first
        .stage_prepared_replacement(lowered)
        .expect("origin candidate stages");
    let foreign_boundary = second
        .execute_framework_turn(|_| {})
        .into_completion()
        .into_execution()
        .expect("equal-looking foreign session produces its own boundary")
        .into_activation_boundary();
    let denial = match first.activate_prepared_replacement(pending, catalog, foreign_boundary, None)
    {
        Ok(_) => panic!("foreign boundary authority must deny before publication"),
        Err(denial) => denial,
    };
    let crate::facade::WorthUiApplicationCutoverDenial::FrameBoundaryUnavailable { reason, retry } =
        denial
    else {
        panic!("foreign boundary denial must return the intact candidate")
    };
    assert_eq!(
        reason,
        crate::runtime::WorthUiActivationGateDenialReason::ForeignFrameBoundarySession
    );
    assert_eq!(first.generation_identity(), &active_generation);
    assert_eq!(
        first.inspect_runtime().generation_identity(),
        &active_generation
    );
    let boundary = first
        .execute_framework_turn(|_| {})
        .into_completion()
        .into_execution()
        .expect("origin session produces a retry boundary")
        .into_activation_boundary();
    let outcome = retry
        .retry(&mut first, boundary)
        .expect("returned candidate retries without reconstruction");
    assert!(outcome.activation().is_some());
    assert_eq!(first.generation_identity(), &successor_generation);
}

#[test]
fn mounted_transition_from_equivalent_candidate_graph_cannot_advance_another_candidate() {
    let first = source_backed_component_session();
    let second = source_backed_component_session();
    let mut first_prepared = first
        .prepare_replacement(component_candidate_submission(
            &first,
            "active-session-first-graph-authority",
            "workspace.component.active_session_candidate",
        ))
        .expect("first candidate should prepare");
    let second_prepared = second
        .prepare_replacement(component_candidate_submission(
            &second,
            "active-session-second-graph-authority",
            "workspace.component.active_session_candidate",
        ))
        .expect("second candidate should prepare");
    let node = second_prepared
        .candidate_graph()
        .node_identities()
        .next()
        .expect("candidate graph should contain a node");
    let prior = second_prepared
        .candidate_graph()
        .lookup()
        .graph_node(node)
        .expect("candidate node should remain addressable")
        .value()
        .participation_posture()
        .axis(crate::graph::UiGraphParticipationAxis::Mounted);
    let foreign = second_prepared
        .candidate_graph()
        .mounted_receipt_transition_for_node(
            node,
            prior,
            crate::graph::UiGraphAxisParticipation::runtime_mutation(
                crate::graph::UiGraphParticipationStatus::Admitted,
            ),
        )
        .expect("second graph should mint its own transition");
    let denial = first_prepared
        .commit_candidate_mounted_layout_admissions(vec![foreign])
        .expect_err("opaque graph authority must deny cross-candidate transition reuse");
    assert!(matches!(
        denial,
        crate::graph::UiGraphMountedLayoutAdmissionDenial::ForeignMountedReceipt(_)
    ));
}

#[test]
fn equal_digest_foreign_catalog_cannot_cross_candidate_graph_authority() {
    let mut session = source_backed_component_session();
    let active_generation = session.generation_identity().clone();
    let mut first_prepared = session
        .prepare_replacement(component_candidate_submission(
            &session,
            "active-session-equal-digest-catalog",
            "workspace.component.active_session_candidate",
        ))
        .expect("first equivalent candidate should prepare");
    let mut second_prepared = session
        .prepare_replacement(component_candidate_submission(
            &session,
            "active-session-equal-digest-catalog",
            "workspace.component.active_session_candidate",
        ))
        .expect("second equivalent candidate should prepare independently");
    let _first_catalog = admit_candidate_catalog(&mut first_prepared);
    let foreign_catalog = admit_candidate_catalog(&mut second_prepared);
    assert_eq!(
        first_prepared
            .candidate_graph()
            .snapshot()
            .authority_digest(),
        second_prepared
            .candidate_graph()
            .snapshot()
            .authority_digest(),
        "the hostile pair must differ only in opaque graph authority"
    );
    assert_ne!(
        first_prepared
            .candidate_graph()
            .snapshot()
            .authority_identity(),
        second_prepared
            .candidate_graph()
            .snapshot()
            .authority_identity()
    );
    let lowered = session
        .lower_prepared_replacement(*first_prepared)
        .expect("origin candidate should lower");
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("origin candidate should stage");
    let boundary = session
        .execute_framework_turn(|_| {})
        .into_completion()
        .into_execution()
        .expect("empty framework turn should yield an activation boundary")
        .into_activation_boundary();
    let denial =
        match session.activate_prepared_replacement(pending, foreign_catalog, boundary, None) {
            Ok(_) => panic!("equal digest cannot replace exact candidate graph authority"),
            Err(denial) => denial,
        };
    assert!(matches!(
        denial,
        crate::facade::WorthUiApplicationCutoverDenial::PreparedApplicationGraphMismatch
    ));
    assert_eq!(session.generation_identity(), &active_generation);
}
