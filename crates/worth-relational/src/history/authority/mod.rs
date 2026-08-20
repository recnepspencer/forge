mod replay_retention;
#[cfg(test)]
mod test_mutation;

use crate::runtime::RelationalRuntime;

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

    pub(crate) fn runtime(&mut self) -> &mut RelationalRuntime {
        self.runtime
    }
}
