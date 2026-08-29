#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiRuntimeServiceResourceCensus {
    portal_records: usize,
    active_portals: usize,
    focus_participants: usize,
    pending_focus_proposals: usize,
    focus_restoration_records: usize,
    staged_motion_proposals: u16,
    active_motion_tracks: u16,
    motion_exit_retentions: u16,
    scroll_owners: usize,
    selection_owners: usize,
    command_routes: usize,
    command_prefixes: usize,
    service_proposals: u16,
    proposal_occupancy_leases: u16,
    proposal_cancellation_records: u16,
    proposal_stage_receipts: u16,
    portal_exit_retentions: usize,
    pending_portal_exit_terminals: usize,
}

impl UiRuntimeServiceResourceCensus {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        portal_records: usize,
        active_portals: usize,
        focus_participants: usize,
        pending_focus_proposals: usize,
        focus_restoration_records: usize,
        staged_motion_proposals: u16,
        active_motion_tracks: u16,
        motion_exit_retentions: u16,
        scroll_owners: usize,
        selection_owners: usize,
        command_routes: usize,
        command_prefixes: usize,
        service_proposals: u16,
        proposal_occupancy_leases: u16,
        proposal_cancellation_records: u16,
        proposal_stage_receipts: u16,
        portal_exit_retentions: usize,
        pending_portal_exit_terminals: usize,
    ) -> Self {
        Self {
            portal_records,
            active_portals,
            focus_participants,
            pending_focus_proposals,
            focus_restoration_records,
            staged_motion_proposals,
            active_motion_tracks,
            motion_exit_retentions,
            scroll_owners,
            selection_owners,
            command_routes,
            command_prefixes,
            service_proposals,
            proposal_occupancy_leases,
            proposal_cancellation_records,
            proposal_stage_receipts,
            portal_exit_retentions,
            pending_portal_exit_terminals,
        }
    }

    pub const EMPTY: Self = Self::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);

    pub fn is_empty(self) -> bool {
        self == Self::EMPTY
    }

    pub fn non_proposal_resources_are_empty(self) -> bool {
        self.portal_records == 0
            && self.active_portals == 0
            && self.focus_participants == 0
            && self.pending_focus_proposals == 0
            && self.focus_restoration_records == 0
            && self.staged_motion_proposals == 0
            && self.active_motion_tracks == 0
            && self.motion_exit_retentions == 0
            && self.scroll_owners == 0
            && self.selection_owners == 0
            && self.command_routes == 0
            && self.command_prefixes == 0
            && self.portal_exit_retentions == 0
            && self.pending_portal_exit_terminals == 0
    }
    pub const fn portal_records(self) -> usize {
        self.portal_records
    }
    pub const fn active_portals(self) -> usize {
        self.active_portals
    }
    pub const fn focus_participants(self) -> usize {
        self.focus_participants
    }
    pub const fn pending_focus_proposals(self) -> usize {
        self.pending_focus_proposals
    }
    pub const fn focus_restoration_records(self) -> usize {
        self.focus_restoration_records
    }
    pub const fn staged_motion_proposals(self) -> u16 {
        self.staged_motion_proposals
    }
    pub const fn active_motion_tracks(self) -> u16 {
        self.active_motion_tracks
    }
    pub const fn motion_exit_retentions(self) -> u16 {
        self.motion_exit_retentions
    }
    pub const fn scroll_owners(self) -> usize {
        self.scroll_owners
    }
    pub const fn selection_owners(self) -> usize {
        self.selection_owners
    }
    pub const fn command_routes(self) -> usize {
        self.command_routes
    }
    pub const fn command_prefixes(self) -> usize {
        self.command_prefixes
    }
    pub const fn service_proposals(self) -> u16 {
        self.service_proposals
    }
    pub const fn proposal_occupancy_leases(self) -> u16 {
        self.proposal_occupancy_leases
    }
    pub const fn proposal_cancellation_records(self) -> u16 {
        self.proposal_cancellation_records
    }
    pub const fn proposal_stage_receipts(self) -> u16 {
        self.proposal_stage_receipts
    }
    pub const fn portal_exit_retentions(self) -> usize {
        self.portal_exit_retentions
    }
    pub const fn pending_portal_exit_terminals(self) -> usize {
        self.pending_portal_exit_terminals
    }
}
