use worth_ui_inspection::{UiInspectionQuery, UiInspectionScope, UiInspectionTarget};

use crate::runtime::tests::active_application_session_test_support::{
    admit_candidate_catalog, component_candidate_submission, source_backed_component_session,
};

#[test]
fn equivalent_replacement_returns_typed_noop() {
    let session = source_backed_component_session();
    let generation = session.generation_identity().clone();
    let submission = component_candidate_submission(
        &session,
        "active-session-equivalent",
        "workspace.component.active_session_current",
    );

    let outcome = session
        .prepare_replacement(submission)
        .expect("equivalent candidate should admit");
    let crate::facade::WorthUiApplicationReplacementPreparation::NoOp(noop) = outcome else {
        panic!("equivalent candidate must not publish");
    };

    assert_eq!(noop.active_generation(), &generation);
    assert_eq!(session.generation_identity(), &generation);
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
    let outcome = first
        .prepare_replacement(component_candidate_submission(
            &first,
            "active-session-cross-launch",
            "workspace.component.active_session_candidate",
        ))
        .expect("candidate should prepare against the first session");
    let crate::facade::WorthUiApplicationReplacementPreparation::Prepared(prepared) = outcome
    else {
        panic!("structurally different candidate should require replacement");
    };

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
    let outcome = first
        .prepare_replacement(component_candidate_submission(
            &first,
            "active-session-cross-stage",
            "workspace.component.active_session_candidate",
        ))
        .expect("candidate should prepare against the first session");
    let crate::facade::WorthUiApplicationReplacementPreparation::Prepared(prepared) = outcome
    else {
        panic!("structurally different candidate should require replacement");
    };
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
    let outcome = first
        .prepare_replacement(component_candidate_submission(
            &first,
            "active-session-cross-cutover",
            "workspace.component.active_session_candidate",
        ))
        .expect("candidate should prepare against the first session");
    let crate::facade::WorthUiApplicationReplacementPreparation::Prepared(mut prepared) = outcome
    else {
        panic!("structurally different candidate should require replacement");
    };
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
fn mounted_transition_from_equivalent_candidate_graph_cannot_advance_another_candidate() {
    let first = source_backed_component_session();
    let second = source_backed_component_session();
    let first_outcome = first
        .prepare_replacement(component_candidate_submission(
            &first,
            "active-session-first-graph-authority",
            "workspace.component.active_session_candidate",
        ))
        .expect("first candidate should prepare");
    let second_outcome = second
        .prepare_replacement(component_candidate_submission(
            &second,
            "active-session-second-graph-authority",
            "workspace.component.active_session_candidate",
        ))
        .expect("second candidate should prepare");
    let crate::facade::WorthUiApplicationReplacementPreparation::Prepared(mut first_prepared) =
        first_outcome
    else {
        panic!("first candidate should be structural");
    };
    let crate::facade::WorthUiApplicationReplacementPreparation::Prepared(second_prepared) =
        second_outcome
    else {
        panic!("second candidate should be structural");
    };
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
    let first_outcome = session
        .prepare_replacement(component_candidate_submission(
            &session,
            "active-session-equal-digest-catalog",
            "workspace.component.active_session_candidate",
        ))
        .expect("first equivalent candidate should prepare");
    let second_outcome = session
        .prepare_replacement(component_candidate_submission(
            &session,
            "active-session-equal-digest-catalog",
            "workspace.component.active_session_candidate",
        ))
        .expect("second equivalent candidate should prepare independently");
    let crate::facade::WorthUiApplicationReplacementPreparation::Prepared(mut first_prepared) =
        first_outcome
    else {
        panic!("first structural candidate should require replacement");
    };
    let crate::facade::WorthUiApplicationReplacementPreparation::Prepared(mut second_prepared) =
        second_outcome
    else {
        panic!("second structural candidate should require replacement");
    };

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

#[test]
fn successful_cutover_publishes_runtime_app_and_inspection_as_one_generation() {
    let mut session = source_backed_component_session();
    let prior_generation = session.generation_identity().clone();
    let prior_planning_authority = session.planning_inspection_authority_identity_for_test();
    assert!(session.planning_inspection_authority_is_runtime_coherent_for_test());
    let outcome = session
        .prepare_replacement(component_candidate_submission(
            &session,
            "active-session-successor",
            "workspace.component.active_session_candidate",
        ))
        .expect("successor candidate should prepare");
    let crate::facade::WorthUiApplicationReplacementPreparation::Prepared(mut prepared) = outcome
    else {
        panic!("successor candidate should not converge as a no-op");
    };
    let catalog = admit_candidate_catalog(&mut prepared);
    let successor_generation = prepared.inspect_candidate(UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::graph(),
    ));
    let successor_generation = successor_generation.generation_identity().clone();
    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("prepared successor should lower");
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("lowered successor should stage");
    let boundary = session
        .execute_framework_turn(|_| {})
        .into_completion()
        .into_execution()
        .expect("empty framework turn should yield an activation boundary")
        .into_activation_boundary();

    let receipt = session
        .activate_prepared_replacement(pending, catalog, boundary, None)
        .expect("candidate-owned catalog should cut over atomically");
    let active_inspection = session.inspect(UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::graph(),
    ));

    assert_eq!(receipt.prior_generation(), &prior_generation);
    assert_eq!(receipt.active_generation(), &successor_generation);
    assert_eq!(session.generation_identity(), &successor_generation);
    assert_eq!(
        session.inspect_runtime().generation_identity(),
        &successor_generation
    );
    assert_eq!(
        active_inspection.generation_identity(),
        &successor_generation
    );
    assert_ne!(
        session.planning_inspection_authority_identity_for_test(),
        prior_planning_authority
    );
    assert!(session.planning_inspection_authority_is_runtime_coherent_for_test());
}

#[test]
fn foreign_catalog_denial_preserves_active_and_candidate_inspection_scopes() {
    let mut session = source_backed_component_session();
    let active_generation = session.generation_identity().clone();
    let outcome = session
        .prepare_replacement(component_candidate_submission(
            &session,
            "active-session-foreign-catalog",
            "workspace.component.active_session_candidate",
        ))
        .expect("structurally different candidate should prepare");
    let crate::facade::WorthUiApplicationReplacementPreparation::Prepared(prepared) = outcome
    else {
        panic!("structurally different candidate must not converge as a no-op");
    };
    let candidate = prepared.inspect_candidate(UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::graph(),
    ));
    assert_ne!(candidate.generation_identity(), &active_generation);
    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("prepared application basis should lower");
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("lowered application replacement should stage");
    let (foreign_snapshot, first, second) =
        crate::runtime::tests::allocation_catalog_test_support::admitted_disjoint_planning_admissions(
            "active-session.foreign-catalog",
        );
    let foreign_catalog = foreign_snapshot
        .admit_allocation_catalog_basis_set(vec![first, second])
        .expect("foreign graph should admit its own complete catalog");
    let boundary = session
        .execute_framework_turn(|_| {})
        .into_completion()
        .into_execution()
        .expect("empty framework turn should yield an activation boundary")
        .into_activation_boundary();

    let denial =
        match session.activate_prepared_replacement(pending, foreign_catalog, boundary, None) {
            Ok(_) => panic!("catalog from a different graph authority must deny"),
            Err(denial) => denial,
        };

    assert!(matches!(
        denial,
        crate::facade::WorthUiApplicationCutoverDenial::PreparedApplicationGraphMismatch
    ));
    assert_eq!(session.generation_identity(), &active_generation);
    assert_eq!(
        session.inspect_runtime().generation_identity(),
        &active_generation
    );
}
