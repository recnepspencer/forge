mod aspect_history;
mod divergence;
mod graph;
mod records;
mod resolution;

use crate::logic::runtime::RelationalRuntime;

pub struct LineageAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl RelationalRuntime {
    pub fn lineage_access(&self) -> LineageAccess<'_> {
        LineageAccess::new(self)
    }
}

impl<'runtime> LineageAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }
}
