/// Structural counters frozen for the publication handoff. They are a
/// projection, not a substitute for owner evidence or retention authority.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CompositePublicationCostCounters {
    relational_owner_contacts: u64,
    signal_owner_contacts: u64,
    expected_head_rechecks: u64,
    unique_pin_hits: u64,
    unique_pin_acquisitions: u64,
    unique_pin_releases: u64,
    history_slots_reserved: u64,
    history_slots_installed: u64,
    product_cell_touches: u64,
    cas_attempts: u64,
    cas_wins: u64,
    cas_losses: u64,
    cancellation_observations: u64,
    retained_partial_creations: u64,
    retained_partial_cleanups: u64,
}

impl CompositePublicationCostCounters {
    pub(crate) const fn zero() -> Self {
        Self {
            relational_owner_contacts: 0,
            signal_owner_contacts: 0,
            expected_head_rechecks: 0,
            unique_pin_hits: 0,
            unique_pin_acquisitions: 0,
            unique_pin_releases: 0,
            history_slots_reserved: 0,
            history_slots_installed: 0,
            product_cell_touches: 0,
            cas_attempts: 0,
            cas_wins: 0,
            cas_losses: 0,
            cancellation_observations: 0,
            retained_partial_creations: 0,
            retained_partial_cleanups: 0,
        }
    }

    pub const fn relational_owner_contacts(self) -> u64 {
        self.relational_owner_contacts
    }

    pub const fn signal_owner_contacts(self) -> u64 {
        self.signal_owner_contacts
    }

    pub const fn expected_head_rechecks(self) -> u64 {
        self.expected_head_rechecks
    }

    pub const fn unique_pin_hits(self) -> u64 {
        self.unique_pin_hits
    }

    pub const fn unique_pin_acquisitions(self) -> u64 {
        self.unique_pin_acquisitions
    }

    pub const fn unique_pin_releases(self) -> u64 {
        self.unique_pin_releases
    }

    pub const fn history_slots_reserved(self) -> u64 {
        self.history_slots_reserved
    }

    pub const fn history_slots_installed(self) -> u64 {
        self.history_slots_installed
    }

    pub const fn product_cell_touches(self) -> u64 {
        self.product_cell_touches
    }

    pub const fn cas_attempts(self) -> u64 {
        self.cas_attempts
    }

    pub const fn cas_wins(self) -> u64 {
        self.cas_wins
    }

    pub const fn cas_losses(self) -> u64 {
        self.cas_losses
    }

    pub const fn cancellation_observations(self) -> u64 {
        self.cancellation_observations
    }

    pub const fn retained_partial_creations(self) -> u64 {
        self.retained_partial_creations
    }

    pub const fn retained_partial_cleanups(self) -> u64 {
        self.retained_partial_cleanups
    }

    pub(crate) fn record_expected_head_recheck(&mut self) {
        self.expected_head_rechecks = self
            .expected_head_rechecks
            .checked_add(1)
            .expect("one bounded publication cannot overflow head recheck accounting");
    }

    pub(crate) fn record_history_slot_installed(&mut self) {
        self.history_slots_installed = self
            .history_slots_installed
            .checked_add(1)
            .expect("one bounded publication cannot overflow history accounting");
    }

    pub(crate) fn record_product_cell_touch(&mut self) {
        self.product_cell_touches = self
            .product_cell_touches
            .checked_add(1)
            .expect("one bounded publication cannot overflow cell accounting");
    }

    pub(crate) fn record_cas_attempt(&mut self) {
        self.cas_attempts = self
            .cas_attempts
            .checked_add(1)
            .expect("one bounded publication cannot overflow CAS accounting");
    }

    pub(crate) fn record_cas_win(&mut self) {
        self.cas_wins = self
            .cas_wins
            .checked_add(1)
            .expect("one bounded publication cannot overflow CAS-win accounting");
    }

    pub(crate) fn record_cas_loss(&mut self) {
        self.cas_losses = self
            .cas_losses
            .checked_add(1)
            .expect("one bounded publication cannot overflow CAS-loss accounting");
    }

    pub(crate) fn record_cancellation_observation(&mut self) {
        self.cancellation_observations = self
            .cancellation_observations
            .checked_add(1)
            .expect("one bounded publication cannot overflow cancellation accounting");
    }
}
