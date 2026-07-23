use crate::facade::{WorthUiActiveApplicationSession, WorthUiApplicationCutoverDenial};
use crate::runtime::tests::active_application_session_test_support::{
    admit_candidate_catalog, source_backed_component_session,
};
use crate::runtime::tests::source_ingress_boundary_test_support::lower_file_submission;
use crate::runtime::{
    WorthUiFrameBoundary, WorthUiSourceProvider, WorthUiWatchedCandidateSubmission,
    WorthUiWatcherEvent,
};

#[test]
fn public_multi_removal_is_canonical_idempotent_and_atomic() {
    let forward = publish_multi_removal(false, false);
    let reverse = publish_multi_removal(true, true);

    assert_eq!(forward.removed_roots, reverse.removed_roots);
    assert_eq!(
        forward.transaction_neighborhoods,
        reverse.transaction_neighborhoods
    );
    assert_eq!(forward.idempotency_key, reverse.idempotency_key);
    assert_eq!(forward.removed_roots.len(), 2);
    assert_eq!(forward.transaction_neighborhoods.len(), 2);

    let mut session = primed_pair_session();
    let predecessor = session.inspect_runtime();
    let (pending, delta) = prepare_removal(&session, false);
    let boundary = WorthUiFrameBoundary::safe_to_activate(
        predecessor.frame_epoch(),
        session.host_session_identity(),
    );
    let (denied, observed) = crate::certification_support::with_activation_precommit_interruption(
        crate::certification_support::WorthUiActivationPrecommitStage::InvalidationWrite,
        || session.activate_prepared_replacement(pending, delta, boundary, None),
    );
    assert_eq!(
        observed,
        Some(crate::certification_support::WorthUiActivationPrecommitStage::InvalidationWrite)
    );
    assert!(matches!(
        denied,
        Err(WorthUiApplicationCutoverDenial::Activation(_))
    ));
    assert_eq!(session.inspect_runtime(), predecessor);

    let retry = activate_removal(&mut session, false);
    assert_eq!(retry.allocation_catalog_successor().successor_rows(), 0);
    assert_eq!(retry.allocation_catalog_successor().transitions().len(), 2);
}

struct RemovalObservation {
    removed_roots: Vec<crate::graph::UiGraphNodeIdentity>,
    transaction_neighborhoods: Vec<crate::evidence::UiAllocationNeighborhoodIdentity>,
    idempotency_key: u64,
}

fn publish_multi_removal(reverse_roots: bool, exercise_boundary_retry: bool) -> RemovalObservation {
    let mut session = primed_pair_session();
    let (pending, delta) = prepare_removal(&session, reverse_roots);
    let frame_epoch = session.inspect_runtime().frame_epoch();
    let host_session = session.host_session_identity();
    let outcome = if exercise_boundary_retry {
        let blocked =
            WorthUiFrameBoundary::traversal_in_progress_for_test(frame_epoch, host_session);
        let denial = match session.activate_prepared_replacement(pending, delta, blocked, None) {
            Ok(_) => panic!("an in-progress frame cannot publish"),
            Err(denial) => denial,
        };
        let WorthUiApplicationCutoverDenial::FrameBoundaryUnavailable { retry, .. } = denial else {
            panic!("frame posture is the only expected denial");
        };
        retry
            .retry(
                &mut session,
                WorthUiFrameBoundary::safe_to_activate(frame_epoch, host_session),
            )
            .expect("the same multi-removal candidate commits exactly once")
    } else {
        session
            .activate_prepared_replacement(
                pending,
                delta,
                WorthUiFrameBoundary::safe_to_activate(frame_epoch, host_session),
                None,
            )
            .expect("multi-removal commits")
    };
    let activation = outcome
        .into_activation()
        .expect("removing two allocation rows changes active truth");
    let successor = activation.allocation_catalog_successor();
    assert_eq!(successor.predecessor_rows(), 2);
    assert_eq!(successor.successor_rows(), 0);
    assert_eq!(successor.carried_rows(), 0);
    assert!(successor.transitions().iter().all(|row| {
        row.disposition() == crate::runtime::UiAllocationCatalogRowDisposition::Removed
    }));
    let transaction = activation.plan_swap().committed_allocation().transaction();
    assert_eq!(
        transaction.ordered_neighborhoods().len(),
        successor.transitions().len(),
        "the ledger transaction and public successor receipt cover the same canonical removals"
    );
    RemovalObservation {
        removed_roots: successor
            .transitions()
            .iter()
            .map(|row| row.root())
            .collect(),
        transaction_neighborhoods: transaction.ordered_neighborhoods().to_vec(),
        idempotency_key: transaction.idempotency_key(),
    }
}

fn primed_pair_session() -> WorthUiActiveApplicationSession {
    let mut session = source_backed_component_session();
    let submission = paired_component_submission(&session, "prime-two-removal-rows");
    let prepared = session
        .prepare_replacement(submission)
        .expect("paired component candidate prepares");
    let mut prepared = prepared;
    let delta = admit_candidate_catalog(&mut prepared);
    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("paired component candidate lowers");
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("paired component candidate stages");
    let boundary = WorthUiFrameBoundary::safe_to_activate(
        session.inspect_runtime().frame_epoch(),
        session.host_session_identity(),
    );
    let activation = session
        .activate_prepared_replacement(pending, delta, boundary, None)
        .expect("paired allocation truth publishes")
        .into_activation()
        .expect("paired allocation truth changes active authority");
    assert_eq!(
        activation.allocation_catalog_successor().successor_rows(),
        2
    );
    session
}

fn activate_removal(
    session: &mut WorthUiActiveApplicationSession,
    reverse_roots: bool,
) -> crate::facade::WorthUiApplicationCutoverReceipt {
    let (pending, delta) = prepare_removal(session, reverse_roots);
    let boundary = WorthUiFrameBoundary::safe_to_activate(
        session.inspect_runtime().frame_epoch(),
        session.host_session_identity(),
    );
    session
        .activate_prepared_replacement(pending, delta, boundary, None)
        .expect("retry after precommit denial publishes complete removal truth")
        .into_activation()
        .expect("multi-removal changes active truth")
}

fn prepare_removal(
    session: &WorthUiActiveApplicationSession,
    reverse_roots: bool,
) -> (
    crate::facade::WorthUiPendingApplicationCutover,
    crate::graph::UiAdmittedAllocationCatalogDelta,
) {
    let submission = empty_candidate_submission(session, "remove-two-allocation-rows");
    let prepared = session
        .prepare_replacement(submission)
        .expect("empty candidate prepares");
    let mut removed = primed_roots_from_fresh_candidate(session);
    if reverse_roots {
        removed.reverse();
    }
    let delta = prepared
        .admit_candidate_allocation_catalog_delta(Vec::new(), removed)
        .expect("candidate graph admits both active removals");
    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("empty candidate lowers");
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("empty candidate stages");
    (pending, delta)
}

fn primed_roots_from_fresh_candidate(
    session: &WorthUiActiveApplicationSession,
) -> Vec<crate::graph::UiGraphNodeIdentity> {
    let mut prepared = session
        .prepare_replacement(paired_component_submission(
            session,
            "observe-two-active-roots",
        ))
        .expect("equivalent pair exposes canonical graph roots");
    let catalog = admit_candidate_catalog(&mut prepared);
    catalog
        .changed
        .iter()
        .map(|(basis, _)| basis.graph_node_identity())
        .collect()
}

fn paired_component_submission(
    session: &WorthUiActiveApplicationSession,
    source_name: &str,
) -> WorthUiWatchedCandidateSubmission {
    lower_file_submission(
        WorthUiSourceProvider::in_memory(source_name).with_file(
            "app/main.wui",
            "component workspace.component.active_session_current { region workspace.region.primary { sizing workspace.sizing.mosaic_support; } }\ncomponent workspace.component.active_session_candidate { region workspace.region.primary { sizing workspace.sizing.mosaic_support; } }",
        ),
        [WorthUiWatcherEvent::provider_revision(source_name)],
        session.capabilities(),
    )
}

fn empty_candidate_submission(
    session: &WorthUiActiveApplicationSession,
    source_name: &str,
) -> WorthUiWatchedCandidateSubmission {
    lower_file_submission(
        WorthUiSourceProvider::in_memory(source_name)
            .with_file("app/main.wui", "token theme.removal_only = \"empty\";"),
        [WorthUiWatcherEvent::provider_revision(source_name)],
        session.capabilities(),
    )
}
