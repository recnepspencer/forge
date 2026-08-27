#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiFocusRuntimeCertificationSnapshot {
    current_participant: Option<u64>,
    active_descendant: Option<u64>,
    participant_count: usize,
    pending_portal_transitions: usize,
    revision: u64,
}

pub trait WorthUiFocusRuntimeCertificationExt {
    fn inspect_focus_runtime_for_certification(&self) -> UiFocusRuntimeCertificationSnapshot;
}

impl WorthUiFocusRuntimeCertificationExt for crate::facade::WorthUiActiveApplicationSession {
    fn inspect_focus_runtime_for_certification(&self) -> UiFocusRuntimeCertificationSnapshot {
        crate::facade::WorthUiActiveApplicationSession::inspect_focus_runtime_for_certification(
            self,
        )
    }
}

impl UiFocusRuntimeCertificationSnapshot {
    pub(crate) const fn new(
        current_participant: Option<u64>,
        active_descendant: Option<u64>,
        participant_count: usize,
        pending_portal_transitions: usize,
        revision: u64,
    ) -> Self {
        Self {
            current_participant,
            active_descendant,
            participant_count,
            pending_portal_transitions,
            revision,
        }
    }

    pub const fn current_participant(self) -> Option<u64> {
        self.current_participant
    }

    pub const fn active_descendant(self) -> Option<u64> {
        self.active_descendant
    }

    pub const fn participant_count(self) -> usize {
        self.participant_count
    }

    pub const fn pending_portal_transitions(self) -> usize {
        self.pending_portal_transitions
    }

    pub const fn revision(self) -> u64 {
        self.revision
    }
}
