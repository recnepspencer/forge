mod commit_finalization;
mod event_emission;

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
