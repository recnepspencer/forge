mod candidate_recording;
mod candidate_validation;
mod commit_finalization;
mod event_emission;
mod phase_types;
mod promotion;
mod promotion_commit;
mod promotion_execution;
mod promotion_planning;

use crate::logic::runtime::RelationalRuntime;

pub struct LineageAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl RelationalRuntime {
    pub fn lineage_authority(&mut self) -> LineageAuthority<'_> {
        LineageAuthority::new(self)
    }
}

impl<'runtime> LineageAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }
}
