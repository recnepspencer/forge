pub(crate) struct UiRuntimeServiceResourceOwnerView<'a> {
    pub(crate) portal: Option<&'a crate::runtime::portal::UiPortalRuntimeState>,
    pub(crate) focus: Option<&'a crate::runtime::focus::UiFocusRuntimeState>,
    pub(crate) motion: Option<&'a crate::runtime::motion::UiMotionRuntimeState>,
    pub(crate) scroll: Option<&'a crate::runtime::scroll::UiScrollRuntimeState>,
    pub(crate) selection: Option<&'a crate::runtime::selection::UiSelectionRuntimeState>,
    pub(crate) command: Option<&'a crate::runtime::command_routing::UiCommandRoutingRuntimeState>,
    pub(crate) proposal_counts: [u16; 4],
    pub(crate) portal_exit_counts: (usize, usize),
}

pub(crate) fn resource_census(
    owners: UiRuntimeServiceResourceOwnerView<'_>,
) -> worth_ui_inspection::UiRuntimeServiceResourceCensus {
    let (portal_records, active_portals) = owners
        .portal
        .map_or((0, 0), |owner| (owner.record_count(), owner.active_count()));
    let (focus_participants, pending_focus_proposals, focus_restoration_records) =
        owners.focus.map_or(
            (0, 0, 0),
            crate::runtime::focus::UiFocusRuntimeState::resource_counts,
        );
    let motion = owners
        .motion
        .map(crate::runtime::motion::UiMotionRuntimeState::census);
    let (command_routes, command_prefixes) = owners.command.map_or(
        (0, 0),
        crate::runtime::command_routing::UiCommandRoutingRuntimeState::resource_counts,
    );
    worth_ui_inspection::UiRuntimeServiceResourceCensus::new(
        portal_records,
        active_portals,
        focus_participants,
        pending_focus_proposals,
        focus_restoration_records,
        motion.map_or(
            0,
            crate::runtime::motion::UiMotionResourceCensus::staged_tracks,
        ),
        motion.map_or(
            0,
            crate::runtime::motion::UiMotionResourceCensus::active_tracks,
        ),
        motion.map_or(
            0,
            crate::runtime::motion::UiMotionResourceCensus::exit_retentions,
        ),
        owners
            .scroll
            .map_or(0, crate::runtime::scroll::UiScrollRuntimeState::owner_count),
        owners.selection.map_or(
            0,
            crate::runtime::selection::UiSelectionRuntimeState::owner_count,
        ),
        command_routes,
        command_prefixes,
        owners.proposal_counts[0],
        owners.proposal_counts[1],
        owners.proposal_counts[2],
        owners.proposal_counts[3],
        owners.portal_exit_counts.0,
        owners.portal_exit_counts.1,
    )
}
