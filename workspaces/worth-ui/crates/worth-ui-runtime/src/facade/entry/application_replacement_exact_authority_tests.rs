use super::WorthUiActiveApplicationSession;
use crate::runtime::tests::active_application_session_test_support::{
    admit_candidate_catalog, component_candidate_submission, source_backed_component_session,
};

#[test]
fn equal_candidate_values_do_not_reopen_prepared_artifact_authority() {
    let session: WorthUiActiveApplicationSession = source_backed_component_session();
    let first = session
        .prepare_replacement(component_candidate_submission(
            &session,
            "exact-candidate-authority",
            "workspace.component.active_session_candidate",
        ))
        .expect("first candidate prepares");
    let second = session
        .prepare_replacement(component_candidate_submission(
            &session,
            "exact-candidate-authority",
            "workspace.component.active_session_candidate",
        ))
        .expect("semantically equal foreign candidate prepares");
    assert_eq!(
        first.semantic_input.admitted().candidate().basis(),
        second.semantic_input.admitted().candidate().basis()
    );
    let authority = first.next_app.prepared_authority().lowering_authority();
    assert!(authority.admits_candidate(first.semantic_input.admitted()));
    assert!(authority.admits_launch_artifact(
        first.semantic_input.admitted().artifact_bundle().artifact(),
        first
            .semantic_input
            .admitted()
            .artifact_bundle()
            .artifact_digest(),
    ));
    assert!(!authority.admits_candidate(second.semantic_input.admitted()));
    assert!(!authority.admits_launch_artifact(
        second
            .semantic_input
            .admitted()
            .artifact_bundle()
            .artifact(),
        second
            .semantic_input
            .admitted()
            .artifact_bundle()
            .artifact_digest(),
    ));
}

#[test]
fn traversal_in_progress_returns_the_exact_candidate_for_retry() {
    let mut session = source_backed_component_session();
    let active_generation = session.generation_identity().clone();
    let mut prepared = session
        .prepare_replacement(component_candidate_submission(
            &session,
            "retryable-frame-boundary",
            "workspace.component.active_session_candidate",
        ))
        .expect("candidate prepares");
    let catalog = admit_candidate_catalog(&mut prepared);
    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("candidate lowers");
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("candidate stages");
    let traversal = session.application.traversal_frame_boundary_for_test();

    let denial = match session.activate_prepared_replacement(pending, catalog, traversal, None) {
        Ok(_) => panic!("frame-in-progress cannot publish"),
        Err(denial) => denial,
    };
    let super::WorthUiApplicationCutoverDenial::FrameBoundaryUnavailable { reason, retry } = denial
    else {
        panic!("transient boundary denial must return candidate ownership")
    };
    assert_eq!(
        reason,
        crate::runtime::WorthUiActivationGateDenialReason::UnsafeFrameBoundary
    );
    assert_eq!(session.generation_identity(), &active_generation);

    let safe = session.application.safe_frame_boundary_for_test();
    let outcome = retry
        .retry(&mut session, safe)
        .expect("the same candidate commits at the next safe boundary");
    assert!(outcome.activation().is_some());
    assert_ne!(session.generation_identity(), &active_generation);
}
