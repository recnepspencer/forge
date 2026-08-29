pub(crate) fn motion_scale_evidence() -> (u64, u64, u64, bool) {
    let frame = worth_ui_host_contract::UiMountedFrameIdentity::mint_unbound().unwrap();
    let semantic_surface =
        worth_ui_host_contract::UiSemanticSurfaceIdentity::mint_unbound().unwrap();
    let presentation = presentation(frame);
    let mut state = super::UiMotionRuntimeState::new(
        crate::runtime::UiServiceStatePersistencePosture::Ephemeral,
    );
    let mut sampler =
        crate::mounting::presentation::motion_sampling::UiMountedMotionSampler::default();

    for index in 0..64_u64 {
        let target = super::UiMotionTargetIdentity::from_family_owner(
            semantic_surface,
            worth_ui_host_contract::UiMountedInstanceIdentity::mint_unbound().unwrap(),
            index + 1,
        );
        let request = super::UiMotionTransitionRequest::from_family_transition(
            target,
            1,
            2,
            presentation,
            geometry([0.0, 0.0, 40.0, 20.0]),
            true,
            presentation,
            geometry([4.0, 8.0, 44.0, 24.0]),
            true,
            super::UiMotionDeclaration::portal_entrance(),
        )
        .expect("motion scale request preserves one presentation binding");
        let proposal =
            crate::runtime::session::service_proposal::UiServiceProposalIdentity::for_test(
                index + 1,
            );
        let staged = state
            .stage(proposal, request)
            .expect("sixty-four tracks fit");
        let derived = state.derive(staged, frame);
        let committed = match state.commit_published(
            derived,
            crate::runtime::session::service_proposal::UiServiceProposalPublicationReceipt::recorded_foreign_fixture(
                proposal,
                7,
                crate::runtime::session::service_proposal::UiServiceProposalPublicationDisposition::Accepted,
            ),
            frame,
            presentation,
        ) {
            Ok(receipt) => receipt,
            Err((_, denial)) => panic!("motion scale track must commit: {denial:?}"),
        };
        sampler
            .install(committed)
            .expect("presentation sampler admits sixty-four active tracks");
    }

    let prepared = sampler
        .prepare_tick(1, presentation)
        .expect("active motion tick prepares");
    let sampled = sampler.commit_prepared(prepared);
    let tracks_considered = sampled.cost().tracks_considered();
    let mut inactive =
        crate::mounting::presentation::motion_sampling::UiMountedMotionSampler::default();
    let inactive_tick = inactive
        .prepare_tick(1, presentation)
        .expect("inactive tick is valid");
    let inactive_work = inactive
        .commit_prepared(inactive_tick)
        .cost()
        .tracks_considered();
    let active_tracks = state.census().active_tracks() as u64;
    let owner_shutdown = state.shutdown().final_census().is_zero();
    let sampler_released = sampler.shutdown() == 64;
    (
        active_tracks,
        tracks_considered,
        inactive_work,
        owner_shutdown && sampler_released,
    )
}

fn geometry(components: [f32; 4]) -> Option<super::UiMotionSemanticGeometry> {
    Some(
        super::UiMotionSemanticGeometry::from_committed_components(
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
