#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationalControlReplayBudget {
    max_active_workflows: usize,
    max_single_recovery_object_bytes: u64,
    max_active_recovery_object_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalControlReplayResource {
    ActiveWorkflows,
    SingleRecoveryObjectBytes,
    ActiveRecoveryObjectBytes,
}

impl OperationalControlReplayBudget {
    pub const PRODUCTION_DEFAULT: Self = Self {
        max_active_workflows: 4_096,
        max_single_recovery_object_bytes: 64 * 1024 * 1024,
        max_active_recovery_object_bytes: 256 * 1024 * 1024,
    };

    pub const fn new(
        max_active_workflows: usize,
        max_single_recovery_object_bytes: u64,
        max_active_recovery_object_bytes: u64,
    ) -> Option<Self> {
        if max_active_workflows == 0
            || max_single_recovery_object_bytes == 0
            || max_single_recovery_object_bytes > max_active_recovery_object_bytes
        {
            None
        } else {
            Some(Self {
                max_active_workflows,
                max_single_recovery_object_bytes,
                max_active_recovery_object_bytes,
            })
        }
    }

    pub const fn max_active_workflows(self) -> usize {
        self.max_active_workflows
    }

    pub const fn max_single_recovery_object_bytes(self) -> u64 {
        self.max_single_recovery_object_bytes
    }

    pub const fn max_active_recovery_object_bytes(self) -> u64 {
        self.max_active_recovery_object_bytes
    }
}

impl Default for OperationalControlReplayBudget {
    fn default() -> Self {
        Self::PRODUCTION_DEFAULT
    }
}
