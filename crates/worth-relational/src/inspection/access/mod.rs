mod history;
mod scope;
mod storage;
mod telemetry;

use crate::runtime::RelationalRuntime;

pub(crate) use scope::{
    empty_retention_plan, summary_degradations, unavailable_scope_availability, KindScopeFilter,
    PartitionScopeFilter,
};

pub struct InspectionAccess<'runtime> {
    pub(super) runtime: &'runtime RelationalRuntime,
}

impl RelationalRuntime {
    pub(crate) fn inspection_access(&self) -> InspectionAccess<'_> {
        InspectionAccess::new(self)
    }
}

impl<'runtime> InspectionAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }
}
