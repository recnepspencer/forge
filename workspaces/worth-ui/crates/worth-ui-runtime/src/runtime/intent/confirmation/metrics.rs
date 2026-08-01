use super::state::{UiIntentConfirmationSlotState, UiIntentConfirmationState};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiIntentConfirmationMetrics {
    pending_challenges: usize,
    retained_candidates: usize,
    retained_payloads: usize,
    issued: u64,
    continued: u64,
    stopped: u64,
    cancelled: u64,
    expired: u64,
    replays: u64,
}

impl UiIntentConfirmationState {
    pub(crate) fn metrics(&self) -> UiIntentConfirmationMetrics {
        let mut pending = 0;
        let mut retained_payloads = 0;
        for slot in &self.slots {
            let UiIntentConfirmationSlotState::Pending(challenge) = &slot.state else {
                continue;
            };
            pending += 1;
            retained_payloads += challenge.candidate.retained_payload_count();
        }
        UiIntentConfirmationMetrics {
            pending_challenges: pending,
            retained_candidates: pending,
            retained_payloads,
            issued: self.counters.issued,
            continued: self.counters.continued,
            stopped: self.counters.stopped,
            cancelled: self.counters.cancelled,
            expired: self.counters.expired,
            replays: self.counters.replays,
        }
    }
}

impl UiIntentConfirmationMetrics {
    pub const fn pending_challenges(self) -> usize {
        self.pending_challenges
    }

    pub const fn retained_candidates(self) -> usize {
        self.retained_candidates
    }

    pub const fn retained_payloads(self) -> usize {
        self.retained_payloads
    }

    pub const fn issued(self) -> u64 {
        self.issued
    }

    pub const fn continued(self) -> u64 {
        self.continued
    }

    pub const fn stopped(self) -> u64 {
        self.stopped
    }

    pub const fn cancelled(self) -> u64 {
        self.cancelled
    }

    pub const fn expired(self) -> u64 {
        self.expired
    }

    pub const fn replays(self) -> u64 {
        self.replays
    }
}
