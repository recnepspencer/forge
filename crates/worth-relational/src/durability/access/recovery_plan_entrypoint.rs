use crate::capabilities::{DurabilityRead, RuntimeConfigSource};
use crate::durability::access::in_memory_recovery_plan::in_memory_recovery_plan;
use crate::durability::access::persisted_recovery_plan::persisted_recovery_plan;
use crate::durability::data::{DurabilityMode, RecoveryPlan, RecoveryVerificationMode};
use crate::history::data::BranchHead;
use crate::logic::runtime::RelationalRuntime;

pub struct DurabilityAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl<'runtime> DurabilityAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn recovery_plan(&self, verification_mode: RecoveryVerificationMode) -> RecoveryPlan {
        match self.runtime.runtime_config().durability.policy.mode {
            DurabilityMode::InMemoryCanonical => {
                in_memory_recovery_plan(self.runtime, verification_mode)
            }
            DurabilityMode::PersistedSegmentedLocalFs => {
                persisted_recovery_plan(self.runtime, verification_mode)
            }
        }
    }

    pub fn durable_log(&self) -> &[crate::replay::data::CanonicalCommitEnvelope] {
        DurabilityRead::durable_log(self.runtime)
    }

    pub fn durable_branch_heads(&self) -> Vec<BranchHead> {
        self.runtime.history().branches()
    }
}
