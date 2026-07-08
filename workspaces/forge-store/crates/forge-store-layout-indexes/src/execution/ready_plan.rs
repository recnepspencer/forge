use crate::execution::lowered_plan::S8LoweredAccessPlan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S8ExecutionReadyAccessPlan {
    lowered: S8LoweredAccessPlan,
}

impl S8ExecutionReadyAccessPlan {
    pub(crate) const fn new(lowered: S8LoweredAccessPlan) -> Self {
        Self { lowered }
    }

    pub(crate) const fn lowered(&self) -> S8LoweredAccessPlan {
        self.lowered
    }
}
