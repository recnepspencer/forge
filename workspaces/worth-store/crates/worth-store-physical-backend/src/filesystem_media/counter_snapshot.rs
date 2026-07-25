use super::{MediaCounterOverflowPolicy, MediaOperationRole};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaCounterTerminal {
    Completed,
    DeniedBeforeEffect,
    PartialEffect,
    IndeterminateEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MediaCounterSnapshot {
    pub(super) attempted_operations: u64,
    pub(super) completed_operations: u64,
    pub(super) denied_before_effect: u64,
    pub(super) partial_effects: u64,
    pub(super) indeterminate_effects: u64,
    pub(super) requested_bytes: u64,
    pub(super) completed_bytes: u64,
    pub(super) eof_observations: u64,
    pub(super) retry_attempts: u64,
    pub(super) listing_batches: u64,
    pub(super) listing_entries: u64,
    pub(super) qualification_transactions: u64,
    pub(super) ownership_attempts: u64,
    pub(super) ownership_acquisitions: u64,
    pub(super) ownership_contentions: u64,
    pub(super) ownership_releases: u64,
    pub(super) file_syncs: u64,
    pub(super) directory_syncs: u64,
    pub(super) replacements: u64,
    pub(super) deletions: u64,
    pub(super) file_opens: u64,
    pub(super) file_creates: u64,
    pub(super) file_closes: u64,
    pub(super) live_file_handles: u64,
    pub(super) peak_file_handles: u64,
    pub(super) directory_opens: u64,
    pub(super) directory_closes: u64,
    pub(super) live_directory_handles: u64,
    pub(super) peak_directory_handles: u64,
    pub(super) confinement_denials: u64,
    pub(super) stale_handle_denials: u64,
    pub(super) unsupported_capabilities: u64,
    pub(super) cleanup_actions: u64,
    pub(super) preserved_residue: u64,
    pub(super) peak_request_width_bytes: u64,
    pub(super) explicit_heap_allocation_events: u64,
    pub(super) requested_heap_capacity_bytes: u64,
    pub(super) fault_matches: u64,
    pub(super) first_fault_match: Option<super::MediaOperationContext>,
    pub(super) first_fault_terminal: Option<MediaCounterTerminal>,
    pub(super) first_fault_completed_bytes: Option<u64>,
    pub(super) saturated: bool,
    pub(super) role_attempts: [u64; MediaOperationRole::ALL.len()],
    pub(super) role_identified_operation_attempts: [u64; MediaOperationRole::ALL.len()],
    pub(super) role_completed_operations: [u64; MediaOperationRole::ALL.len()],
    pub(super) role_denied_before_effect: [u64; MediaOperationRole::ALL.len()],
    pub(super) role_partial_effects: [u64; MediaOperationRole::ALL.len()],
    pub(super) role_indeterminate_effects: [u64; MediaOperationRole::ALL.len()],
    pub(super) role_requested_bytes: [u64; MediaOperationRole::ALL.len()],
    pub(super) role_completed_bytes: [u64; MediaOperationRole::ALL.len()],
}

macro_rules! accessors {
    ($($name:ident),+ $(,)?) => {$(
        pub const fn $name(self) -> u64 { self.$name }
    )+};
}

impl MediaCounterSnapshot {
    accessors!(
        attempted_operations,
        completed_operations,
        denied_before_effect,
        partial_effects,
        indeterminate_effects,
        requested_bytes,
        completed_bytes,
        eof_observations,
        retry_attempts,
        listing_batches,
        listing_entries,
        qualification_transactions,
        ownership_attempts,
        ownership_acquisitions,
        ownership_contentions,
        ownership_releases,
        file_syncs,
        directory_syncs,
        replacements,
        deletions,
        file_opens,
        file_creates,
        file_closes,
        live_file_handles,
        peak_file_handles,
        directory_opens,
        directory_closes,
        live_directory_handles,
        peak_directory_handles,
        confinement_denials,
        stale_handle_denials,
        unsupported_capabilities,
        cleanup_actions,
        preserved_residue,
        peak_request_width_bytes,
        explicit_heap_allocation_events,
        requested_heap_capacity_bytes,
        fault_matches,
    );

    /// The first production boundary actually selected by a certification
    /// fault schedule. This is observation only and grants no media authority.
    pub const fn first_fault_match(self) -> Option<super::MediaOperationContext> {
        self.first_fault_match
    }

    pub const fn first_fault_terminal(self) -> Option<MediaCounterTerminal> {
        self.first_fault_terminal
    }

    pub const fn first_fault_completed_bytes(self) -> Option<u64> {
        self.first_fault_completed_bytes
    }

    /// Compares exact counter and terminal values while excluding the
    /// identity-bearing coordinates of a certification fault match.
    pub fn same_counter_values(mut self, mut other: Self) -> bool {
        self.first_fault_match = None;
        other.first_fault_match = None;
        self == other
    }

    pub const fn overflow_policy(self) -> MediaCounterOverflowPolicy {
        MediaCounterOverflowPolicy::Saturate
    }

    pub const fn is_conserved(self) -> bool {
        if self.saturated
            || self.attempted_operations
                != self
                    .completed_operations
                    .saturating_add(self.denied_before_effect)
                    .saturating_add(self.partial_effects)
                    .saturating_add(self.indeterminate_effects)
            || self.completed_bytes > self.requested_bytes
        {
            return false;
        }
        let mut index = 0;
        while index < MediaOperationRole::ALL.len() {
            let terminal = self.role_completed_operations[index]
                .saturating_add(self.role_denied_before_effect[index])
                .saturating_add(self.role_partial_effects[index])
                .saturating_add(self.role_indeterminate_effects[index]);
            if self.role_attempts[index] != terminal
                || self.role_completed_bytes[index] > self.role_requested_bytes[index]
            {
                return false;
            }
            index += 1;
        }
        true
    }

    pub const fn attempts_for(self, role: MediaOperationRole) -> u64 {
        self.role_attempts[role.index()]
    }

    /// Per-role attempts carrying a backend-issued operation identity.
    /// Unbound qualification and recovery helpers do not advance it.
    pub const fn identified_operation_attempts_for(self, role: MediaOperationRole) -> u64 {
        self.role_identified_operation_attempts[role.index()]
    }

    pub const fn requested_bytes_for(self, role: MediaOperationRole) -> u64 {
        self.role_requested_bytes[role.index()]
    }

    pub const fn completed_operations_for(self, role: MediaOperationRole) -> u64 {
        self.role_completed_operations[role.index()]
    }

    pub const fn denied_before_effect_for(self, role: MediaOperationRole) -> u64 {
        self.role_denied_before_effect[role.index()]
    }

    pub const fn partial_effects_for(self, role: MediaOperationRole) -> u64 {
        self.role_partial_effects[role.index()]
    }

    pub const fn indeterminate_effects_for(self, role: MediaOperationRole) -> u64 {
        self.role_indeterminate_effects[role.index()]
    }

    pub const fn completed_bytes_for(self, role: MediaOperationRole) -> u64 {
        self.role_completed_bytes[role.index()]
    }

    pub const fn saturated(self) -> bool {
        self.saturated
    }

    pub const fn short_transfers(self) -> u64 {
        self.partial_effects
    }

    pub const fn positioned_read_attempts(self) -> u64 {
        self.attempts_for(MediaOperationRole::PositionedRead)
    }

    pub const fn positioned_write_attempts(self) -> u64 {
        self.attempts_for(MediaOperationRole::PositionedWrite)
    }

    pub const fn append_attempts(self) -> u64 {
        self.attempts_for(MediaOperationRole::Append)
    }

    pub(super) fn mark_saturation(&mut self) {
        self.saturated = self.any_counter_saturated();
    }

    fn any_counter_saturated(self) -> bool {
        let scalar = [
            self.attempted_operations,
            self.completed_operations,
            self.denied_before_effect,
            self.partial_effects,
            self.indeterminate_effects,
            self.requested_bytes,
            self.completed_bytes,
            self.eof_observations,
            self.retry_attempts,
            self.listing_batches,
            self.listing_entries,
            self.qualification_transactions,
            self.ownership_attempts,
            self.ownership_acquisitions,
            self.ownership_contentions,
            self.ownership_releases,
            self.file_syncs,
            self.directory_syncs,
            self.replacements,
            self.deletions,
            self.file_opens,
            self.file_creates,
            self.file_closes,
            self.live_file_handles,
            self.peak_file_handles,
            self.directory_opens,
            self.directory_closes,
            self.live_directory_handles,
            self.peak_directory_handles,
            self.confinement_denials,
            self.stale_handle_denials,
            self.unsupported_capabilities,
            self.cleanup_actions,
            self.preserved_residue,
            self.peak_request_width_bytes,
            self.explicit_heap_allocation_events,
            self.requested_heap_capacity_bytes,
            self.fault_matches,
        ];
        scalar.contains(&u64::MAX)
            || self.role_attempts.contains(&u64::MAX)
            || self.role_identified_operation_attempts.contains(&u64::MAX)
            || self.role_completed_operations.contains(&u64::MAX)
            || self.role_denied_before_effect.contains(&u64::MAX)
            || self.role_partial_effects.contains(&u64::MAX)
            || self.role_indeterminate_effects.contains(&u64::MAX)
            || self.role_requested_bytes.contains(&u64::MAX)
            || self.role_completed_bytes.contains(&u64::MAX)
    }
}
