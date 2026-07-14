#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CheckpointRecoveryCounterSnapshot {
    candidate_count: u64,
    locator_check_count: u64,
    manifest_validation_count: u64,
    integrity_damage_check_count: u64,
    cutover_decision_count: u64,
    retention_decision_count: u64,
}

impl CheckpointRecoveryCounterSnapshot {
    pub const fn new() -> Self {
        Self {
            candidate_count: 0,
            locator_check_count: 0,
            manifest_validation_count: 0,
            integrity_damage_check_count: 0,
            cutover_decision_count: 0,
            retention_decision_count: 0,
        }
    }

    pub const fn with_candidate(mut self) -> Self {
        self.candidate_count += 1;
        self
    }

    pub const fn with_locator_check(mut self) -> Self {
        self.locator_check_count += 1;
        self
    }

    pub const fn with_manifest_validation(mut self) -> Self {
        self.manifest_validation_count += 1;
        self
    }

    pub const fn with_integrity_damage_check(mut self) -> Self {
        self.integrity_damage_check_count += 1;
        self
    }

    pub const fn with_cutover_decision(mut self) -> Self {
        self.cutover_decision_count += 1;
        self
    }

    pub const fn with_retention_decision(mut self) -> Self {
        self.retention_decision_count += 1;
        self
    }

    pub const fn candidate_count(self) -> u64 {
        self.candidate_count
    }

    pub const fn locator_check_count(self) -> u64 {
        self.locator_check_count
    }

    pub const fn manifest_validation_count(self) -> u64 {
        self.manifest_validation_count
    }

    pub const fn integrity_damage_check_count(self) -> u64 {
        self.integrity_damage_check_count
    }

    pub const fn cutover_decision_count(self) -> u64 {
        self.cutover_decision_count
    }

    pub const fn retention_decision_count(self) -> u64 {
        self.retention_decision_count
    }
}
