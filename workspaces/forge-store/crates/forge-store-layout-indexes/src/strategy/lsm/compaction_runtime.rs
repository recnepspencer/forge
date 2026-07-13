use super::{
    AdmittedLsmCompactionDemand, BaselineLsmExecutionAdmissionDenial, PreparedLsmCompaction,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmCompactionRuntime;

pub const fn lsm_compaction_runtime() -> LsmCompactionRuntime {
    LsmCompactionRuntime
}

impl LsmCompactionRuntime {
    pub fn execute(
        self,
        demand: AdmittedLsmCompactionDemand,
    ) -> Result<PreparedLsmCompaction, BaselineLsmExecutionAdmissionDenial> {
        PreparedLsmCompaction::execute(demand)
    }
}
