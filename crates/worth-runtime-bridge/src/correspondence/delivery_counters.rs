/// Work performed while matching and delivering one installed correspondence.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CorrespondenceDeliveryCounters {
    pub(crate) source_load_attempts: usize,
    pub(crate) source_envelopes_loaded: usize,
    pub(crate) allocation_registry_lock_attempts: usize,
    pub(crate) allocation_source_set_checks: usize,
    pub(crate) signal_basis_target_checks: usize,
    pub(crate) signal_capability_admissions: usize,
    pub(crate) failed_deliveries: usize,
    pub(crate) truth_targets_admitted: usize,
    pub(crate) correspondence_lookups: usize,
    pub(crate) semantic_match_checks: usize,
    pub(crate) aspect_rejections: usize,
    pub(crate) binding_rejections: usize,
    pub(crate) change_kind_rejections: usize,
    pub(crate) locality_rejections: usize,
    pub(crate) projection_rejections: usize,
    pub(crate) relevant_change_checks: usize,
    pub(crate) projection_paths_inspected: usize,
    pub(crate) source_widening_target_checks: usize,
    pub(crate) signal_seeds_emitted: usize,
    pub(crate) node_fan_out: usize,
    pub(crate) slots_touched: usize,
}

impl CorrespondenceDeliveryCounters {
    pub const fn zero() -> Self {
        Self {
            source_load_attempts: 0,
            source_envelopes_loaded: 0,
            allocation_registry_lock_attempts: 0,
            allocation_source_set_checks: 0,
            signal_basis_target_checks: 0,
            signal_capability_admissions: 0,
            failed_deliveries: 0,
            truth_targets_admitted: 0,
            correspondence_lookups: 0,
            semantic_match_checks: 0,
            aspect_rejections: 0,
            binding_rejections: 0,
            change_kind_rejections: 0,
            locality_rejections: 0,
            projection_rejections: 0,
            relevant_change_checks: 0,
            projection_paths_inspected: 0,
            source_widening_target_checks: 0,
            signal_seeds_emitted: 0,
            node_fan_out: 0,
            slots_touched: 0,
        }
    }

    pub const fn truth_targets_admitted(self) -> usize {
        self.truth_targets_admitted
    }
    pub const fn source_load_attempts(self) -> usize {
        self.source_load_attempts
    }
    pub const fn source_envelopes_loaded(self) -> usize {
        self.source_envelopes_loaded
    }
    pub const fn allocation_registry_lock_attempts(self) -> usize {
        self.allocation_registry_lock_attempts
    }
    pub const fn allocation_source_set_checks(self) -> usize {
        self.allocation_source_set_checks
    }
    pub const fn signal_basis_target_checks(self) -> usize {
        self.signal_basis_target_checks
    }
    pub const fn signal_capability_admissions(self) -> usize {
        self.signal_capability_admissions
    }
    pub const fn failed_deliveries(self) -> usize {
        self.failed_deliveries
    }
    pub const fn correspondence_lookups(self) -> usize {
        self.correspondence_lookups
    }
    pub const fn semantic_match_checks(self) -> usize {
        self.semantic_match_checks
    }
    pub const fn aspect_rejections(self) -> usize {
        self.aspect_rejections
    }
    pub const fn binding_rejections(self) -> usize {
        self.binding_rejections
    }
    pub const fn change_kind_rejections(self) -> usize {
        self.change_kind_rejections
    }
    pub const fn locality_rejections(self) -> usize {
        self.locality_rejections
    }
    pub const fn projection_rejections(self) -> usize {
        self.projection_rejections
    }
    pub const fn relevant_change_checks(self) -> usize {
        self.relevant_change_checks
    }
    pub const fn projection_paths_inspected(self) -> usize {
        self.projection_paths_inspected
    }
    pub const fn source_widening_target_checks(self) -> usize {
        self.source_widening_target_checks
    }
    pub const fn signal_seeds_emitted(self) -> usize {
        self.signal_seeds_emitted
    }
    pub const fn node_fan_out(self) -> usize {
        self.node_fan_out
    }
    pub const fn slots_touched(self) -> usize {
        self.slots_touched
    }
}
