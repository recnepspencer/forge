#[cfg(test)]
mod candidate_recording;
#[cfg(test)]
mod candidate_validation;
mod commit_finalization;
#[cfg(test)]
mod diagnostic_fields;
mod event_emission;
#[cfg(test)]
mod phase_types;
#[cfg(test)]
mod promotion;
#[cfg(test)]
mod promotion_commit;
#[cfg(test)]
mod promotion_execution;
#[cfg(test)]
mod promotion_planning;

#[cfg(test)]
pub(crate) use promotion_commit::LineageDurableAppendAdmission;

use crate::runtime::RelationalRuntime;

pub struct LineageAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl RelationalRuntime {
    pub(crate) fn lineage_authority(&mut self) -> LineageAuthority<'_> {
        LineageAuthority::new(self)
    }
}

impl<'runtime> LineageAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }
}
