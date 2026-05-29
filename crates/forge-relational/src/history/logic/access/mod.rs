mod ancestry;
mod aspect_history_queries;
mod commit_surfaces;

use crate::logic::runtime::RelationalRuntime;

pub struct HistoryAccess<'runtime> {
    runtime: &'runtime RelationalRuntime,
}

impl RelationalRuntime {
    pub(crate) fn current_version_id(&self) -> crate::identity::data::VersionId {
        self.history.current_version_id()
    }

    pub(crate) fn history_access(&self) -> HistoryAccess<'_> {
        HistoryAccess::new(self)
    }
}

impl<'runtime> HistoryAccess<'runtime> {
    pub(crate) fn new(runtime: &'runtime RelationalRuntime) -> Self {
        Self { runtime }
    }
}
