mod branch_management;
mod commit_publication;
mod replay_retention;

use crate::logic::runtime::RelationalRuntime;

pub struct HistoryAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl RelationalRuntime {
    pub fn history_authority(&mut self) -> HistoryAuthority<'_> {
        HistoryAuthority::new(self)
    }
}

impl<'runtime> HistoryAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }
}
