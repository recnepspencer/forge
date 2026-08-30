pub(crate) fn selection_scale_evidence() -> (u64, u64, bool) {
    let family =
        crate::runtime::UiApplicationItemKeyFamily::new(core::num::NonZeroU64::new(3).unwrap());
    let owner = super::UiSelectionOwnerIdentity::new(
        worth_ui_host_contract::UiSemanticSurfaceIdentity::mint_unbound().unwrap(),
        crate::graph::UiGraphNodeIdentity::new(71),
        family,
    );
    let incarnation = super::UiSelectionOwnerIncarnation::new(7).unwrap();
    let keys = (1..=1_024_u64)
        .map(|value| {
            super::UiSelectionStableKey::new(crate::runtime::UiApplicationItemKey::new(
                family,
                core::num::NonZeroU64::new(value).unwrap(),
            ))
        })
        .collect::<Vec<_>>();
    let mut state = super::UiSelectionRuntimeState::new_session_restore_candidate();
    state
        .synchronize(
            super::UiSelectionRegistration::new(
                owner,
                incarnation,
                super::UiSelectionPolicy::Multiple,
                keys.clone(),
                super::UiSelectionCatalogPosture::Complete,
            )
            .expect("selection scale registration is legal"),
        )
        .expect("selection scale catalog installs");
    let before = state.selection_keys_visited();
    let delta = state
        .apply(
            owner,
            incarnation,
            super::UiSelectionRequest::ToggleMultiple(keys[777]),
        )
        .expect("one stable key toggles");
    let visited = state.selection_keys_visited() - before;
    let bounded = delta.candidates_visited() == 1;
    let released = state.shutdown();
    (keys.len() as u64, visited, released == 1 && bounded)
}
