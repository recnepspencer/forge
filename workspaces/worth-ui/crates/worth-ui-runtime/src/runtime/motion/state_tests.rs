use super::*;

#[test]
fn accepted_motion_publishes_once_and_terminalizes_once() {
    let mut state = runtime_state();
    let frame = frame();
    let proposal = proposal(1);
    let request = request(11, 1, 2, frame);

    let commit = commit(&mut state, proposal, request, frame);
    assert_eq!(state.census().active_tracks(), 1);
    assert_eq!(state.census().staged_tracks(), 0);
    assert_eq!(state.publication_count(), 1);
    assert_eq!(commit.fact().kind(), UiMotionProducedFactKind::Started);

    let terminal = state
        .terminalize(commit.track().identity(), UiMotionTerminalCause::Completed)
        .expect("committed track remains terminalizable");
    assert_eq!(terminal.cause(), UiMotionTerminalCause::Completed);
    assert_eq!(terminal.fact().publication_sequence(), 2);
    assert!(state.census().is_zero());
}

#[test]
fn retarget_replaces_one_track_and_emits_one_retarget_fact() {
    let mut state = runtime_state();
    let frame = frame();
    let target = target(19);
    let first = commit(
        &mut state,
        proposal(10),
        request_for_target(target, 1, 2, frame),
        frame,
    );
    let second = commit(
        &mut state,
        proposal(11),
        request_for_target(target, 2, 3, frame),
        frame,
    );

    assert_ne!(first.track().identity(), second.track().identity());
    assert_eq!(state.census().active_tracks(), 1);
    assert_eq!(state.publication_count(), 2);
    assert_eq!(
        second.fact().kind(),
        UiMotionProducedFactKind::Retargeted(UiMotionRetargetDisposition::Install {
            predecessor: UiMotionRetargetPredecessor::CurrentPresentationSample,
        })
    );
    let interrupted = state
        .last_interruption()
        .expect("the owner retains its latest interruption cause");
    assert_eq!(interrupted, second.fact());

    state
        .terminalize(second.track().identity(), UiMotionTerminalCause::Completed)
        .expect("the retargeted track remains terminalizable");
    assert_eq!(state.last_interruption(), Some(interrupted));
}

#[test]
fn terminal_exit_retention_survives_track_terminalization_until_exact_release() {
    let mut state = runtime_state();
    let frame = frame();
    let target = target(24);
    let commit = commit(
        &mut state,
        proposal(24),
        request_for_target_with_declaration(
            target,
            1,
            2,
            frame,
            UiMotionDeclaration::portal_exit(),
        ),
        frame,
    );
    let retention = commit
        .exit_retention()
        .expect("portal exit reserves exact Motion retention");
    assert_eq!(state.census().exit_retentions(), 1);

    let terminal = state
        .terminalize(commit.track().identity(), UiMotionTerminalCause::Completed)
        .expect("retained exit remains terminalizable");
    assert_eq!(terminal.exit_retention(), Some(retention));
    assert_eq!(state.census().active_tracks(), 0);
    assert_eq!(state.census().exit_retentions(), 1);

    assert!(state.release_exit_retention(retention));
    assert!(!state.release_exit_retention(retention));
    assert!(state.census().is_zero());
}

#[test]
fn interruption_displaces_a_terminal_exit_retention_without_leaking_census() {
    let mut state = runtime_state();
    let frame = frame();
    let target = target(25);
    let exit = commit(
        &mut state,
        proposal(25),
        request_for_target_with_declaration(
            target,
            1,
            2,
            frame,
            UiMotionDeclaration::portal_exit(),
        ),
        frame,
    );
    let displaced = exit.exit_retention().unwrap();
    state
        .terminalize(exit.track().identity(), UiMotionTerminalCause::Completed)
        .unwrap();

    let entrance = commit(
        &mut state,
        proposal(26),
        request_for_target_with_declaration(
            target,
            2,
            3,
            frame,
            UiMotionDeclaration::portal_entrance(),
        ),
        frame,
    );
    assert_eq!(entrance.displaced_exit_retention(), Some(displaced));
    assert_eq!(entrance.exit_retention(), None);
    assert_eq!(state.census().exit_retentions(), 0);
    assert!(!state.release_exit_retention(displaced));
}

#[test]
fn exit_retarget_replaces_retention_one_for_one_at_capacity_boundary() {
    let mut state = runtime_state();
    let frame = frame();
    let target = target(27);
    let first = commit(
        &mut state,
        proposal(27),
        request_for_target_with_declaration(
            target,
            1,
            2,
            frame,
            UiMotionDeclaration::portal_exit(),
        ),
        frame,
    );
    let second = commit(
        &mut state,
        proposal(28),
        request_for_target_with_declaration(
            target,
            2,
            3,
            frame,
            UiMotionDeclaration::portal_exit(),
        ),
        frame,
    );

    assert_eq!(second.displaced_exit_retention(), first.exit_retention());
    assert_ne!(second.exit_retention(), first.exit_retention());
    assert_eq!(state.census().active_tracks(), 1);
    assert_eq!(state.census().exit_retentions(), 1);
}

#[test]
fn rejected_publication_retains_a_discardable_linear_token() {
    let mut state = runtime_state();
    let frame = frame();
    let proposal = proposal(20);
    let request = request(20, 1, 2, frame);
    let presentation = request.successor().presentation();
    let staged = state
        .stage(proposal, request)
        .expect("one Motion proposal fits the census");
    let derived = state.derive(staged, frame);
    let publication = publication(
        proposal,
        crate::runtime::session::service_proposal::UiServiceProposalPublicationDisposition::Rejected,
    );
    let derived = match state.commit_published(derived, publication, frame, presentation) {
        Err((derived, UiMotionCommitDenial::PublicationRejected)) => derived,
        _ => panic!("rejected mounted publication must not commit Motion"),
    };

    assert_eq!(state.publication_count(), 0);
    assert_eq!(state.census().staged_tracks(), 1);
    state.discard_derived(derived);
    assert!(state.census().is_zero());
}

#[test]
fn track_capacity_is_exactly_sixty_four() {
    let mut state = runtime_state();
    let frame = frame();
    let mut staged = Vec::new();
    for value in 0..64_u64 {
        staged.push(
            state
                .stage(proposal(100 + value), request(100 + value, 1, 2, frame))
                .expect("declared Motion capacity admits sixty-four tracks"),
        );
    }
    assert!(matches!(
        state.stage(proposal(999), request(999, 1, 2, frame)),
        Err(UiMotionStagingDenial::CapacityExceeded)
    ));
    assert_eq!(state.census().staged_tracks(), 64);

    for token in staged {
        state.discard_staged(token);
    }
    assert!(state.census().is_zero());
}

#[test]
fn shutdown_reports_and_zeroes_every_motion_resource() {
    let mut state = runtime_state();
    let frame = frame();
    let _active = commit(&mut state, proposal(300), request(300, 1, 2, frame), frame);
    let _staged = state
        .stage(proposal(301), request(301, 1, 2, frame))
        .expect("shutdown fixture stages one retained proposal");

    let report = state.shutdown();
    assert_eq!(report.abandoned_staged_tracks(), 1);
    assert_eq!(report.terminated_active_tracks(), 1);
    assert_eq!(report.cancelled_exit_retentions(), 0);
    assert!(report.final_census().is_zero());
    assert_eq!(state.publication_count(), 2);
    assert_eq!(
        state
            .last_fact()
            .expect("shutdown publishes a terminal fact")
            .kind(),
        UiMotionProducedFactKind::Terminal(UiMotionTerminalCause::ApplicationShutdown)
    );
}

#[test]
fn shutdown_counts_and_cancels_terminal_exit_retention() {
    let mut state = runtime_state();
    let frame = frame();
    let exit = commit(
        &mut state,
        proposal(302),
        request_for_target_with_declaration(
            target(302),
            1,
            2,
            frame,
            UiMotionDeclaration::portal_exit(),
        ),
        frame,
    );
    state
        .terminalize(exit.track().identity(), UiMotionTerminalCause::Completed)
        .unwrap();

    let report = state.shutdown();
    assert_eq!(report.terminated_active_tracks(), 0);
    assert_eq!(report.cancelled_exit_retentions(), 1);
    assert!(report.final_census().is_zero());
}

fn runtime_state() -> UiMotionRuntimeState {
    UiMotionRuntimeState::new(crate::runtime::UiServiceStatePersistencePosture::Ephemeral)
}

fn commit(
    state: &mut UiMotionRuntimeState,
    proposal: crate::runtime::session::service_proposal::UiServiceProposalIdentity,
    request: UiMotionTransitionRequest,
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
) -> UiMotionCommitReceipt {
    let presentation = request.successor().presentation();
    let staged = state
        .stage(proposal, request)
        .expect("Motion fixture fits the census");
    let derived = state.derive(staged, frame);
    match state.commit_published(
        derived,
        publication(
            proposal,
            crate::runtime::session::service_proposal::UiServiceProposalPublicationDisposition::Accepted,
        ),
        frame,
        presentation,
    ) {
        Ok(receipt) => receipt,
        Err((_, denial)) => panic!("valid Motion fixture must commit: {denial:?}"),
    }
}

fn request(
    owner_key: u64,
    predecessor_revision: u64,
    successor_revision: u64,
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
) -> UiMotionTransitionRequest {
    request_for_target(
        target(owner_key),
        predecessor_revision,
        successor_revision,
        frame,
    )
}

fn target(owner_key: u64) -> UiMotionTargetIdentity {
    UiMotionTargetIdentity::from_family_owner(
        worth_ui_host_contract::UiSemanticSurfaceIdentity::mint_unbound().unwrap(),
        worth_ui_host_contract::UiMountedInstanceIdentity::mint_unbound().unwrap(),
        owner_key,
    )
}

fn request_for_target(
    target: UiMotionTargetIdentity,
    predecessor_revision: u64,
    successor_revision: u64,
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
) -> UiMotionTransitionRequest {
    request_for_target_with_declaration(
        target,
        predecessor_revision,
        successor_revision,
        frame,
        UiMotionDeclaration::portal_entrance(),
    )
}

fn request_for_target_with_declaration(
    target: UiMotionTargetIdentity,
    predecessor_revision: u64,
    successor_revision: u64,
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    declaration: UiMotionDeclaration,
) -> UiMotionTransitionRequest {
    let presentation = presentation(frame);
    UiMotionTransitionRequest::from_family_transition(
        target,
        predecessor_revision,
        successor_revision,
        presentation,
        geometry([0.0, 0.0, 40.0, 20.0]),
        predecessor_revision > 0,
        presentation,
        geometry([4.0, 8.0, 44.0, 24.0]),
        true,
        declaration,
    )
    .expect("Motion fixture preserves one presentation binding")
}

fn geometry(components: [f32; 4]) -> Option<UiMotionSemanticGeometry> {
    Some(
        UiMotionSemanticGeometry::from_committed_components(
            components,
            worth_ui_host_contract::UiMountedCoordinateSpace::HostSurface,
        )
        .unwrap(),
    )
}

fn presentation(
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
) -> worth_ui_host_contract::UiHostObservationPresentationBasis {
    worth_ui_host_contract::UiHostObservationPresentationBasis::new(
        worth_ui_host_contract::UiHostSurfaceIdentity::mint_unbound().unwrap(),
        frame,
        worth_ui_host_contract::UiSurfaceBindingGeneration::mint_unbound().unwrap(),
        worth_ui_host_contract::UiHostPresentationEpoch::issued_by_host(1),
    )
}

fn frame() -> worth_ui_host_contract::UiMountedFrameIdentity {
    worth_ui_host_contract::UiMountedFrameIdentity::mint_unbound().unwrap()
}

fn proposal(value: u64) -> crate::runtime::session::service_proposal::UiServiceProposalIdentity {
    crate::runtime::session::service_proposal::UiServiceProposalIdentity::for_test(value)
}

fn publication(
    proposal: crate::runtime::session::service_proposal::UiServiceProposalIdentity,
    disposition: crate::runtime::session::service_proposal::UiServiceProposalPublicationDisposition,
) -> crate::runtime::session::service_proposal::UiServiceProposalPublicationReceipt {
    crate::runtime::session::service_proposal::UiServiceProposalPublicationReceipt::recorded_foreign_fixture(
        proposal,
        7,
        disposition,
    )
}
