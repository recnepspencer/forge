mod inspection_counters;
mod lineage_counters;
mod merge_counters;
mod preparation_counters;
mod query_projection_counters;
mod replay_authority_basis_counters;
mod schema_continuity_counters;
#[cfg(test)]
mod test_complexity_observability;
mod validation_counters;
mod working_state_counters;

use crate::runtime::RelationalRuntime;

pub(crate) use replay_authority_basis_counters::ReplayLineageAuthorityIndexedSource;

pub struct PerformanceAccess<'runtime> {
    pub(super) runtime: &'runtime RelationalRuntime,
}

impl RelationalRuntime {
    pub(crate) fn performance_access(&self) -> PerformanceAccess<'_> {
        PerformanceAccess::new(self)
    }
}

impl<'runtime> PerformanceAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }
}
