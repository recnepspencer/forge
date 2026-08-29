pub(crate) fn scroll_scale_evidence() -> (u64, u64, bool) {
    let surface = worth_ui_host_contract::UiSemanticSurfaceIdentity::mint_unbound().unwrap();
    let mut state = super::UiScrollRuntimeState::new_session_restore_candidate();
    let mut chain = Vec::new();
    for index in 1..=8_u64 {
        let owner = super::UiScrollOwnerIdentity::region(
            surface,
            crate::graph::UiGraphNodeIdentity::new(index),
            index,
        );
        let incarnation = super::UiScrollOwnerIncarnation::new(index).unwrap();
        state
            .register(super::UiScrollOwnerRegistration::new(
                owner,
                incarnation,
                super::UiScrollAxes::Block,
                super::UiScrollBounds::new(0, 1).unwrap(),
                super::UiScrollOffset::origin(),
            ))
            .expect("scroll owner registers");
        chain.push(super::UiScrollChainEntry::new(owner, incarnation));
    }
    let request = super::UiScrollDeltaRequest::new(
        chain,
        super::UiScrollDelta::new(0, 8),
        super::UiScrollDeltaCause::Host {
            source: worth_ui_host_contract::UiHostScrollDeltaSource::PointerWheel,
            phase: worth_ui_host_contract::UiHostScrollDeltaPhase::Updated,
            precision: worth_ui_host_contract::UiHostScrollDeltaPrecision::Pixel,
        },
    )
    .expect("the eight-owner chain is legal");
    let receipt = state.route(request).expect("the nested route commits");
    let owners_visited = receipt.owners_visited() as u64;
    let released = state.shutdown();
    (8, owners_visited, released == 8)
}
