mod materialization;
mod reader;
mod visibility;

pub(crate) use reader::VisibilityReadContext;

use crate::logic::runtime::RelationalRuntime;

impl RelationalRuntime {
    pub fn visibility_reads(&self) -> VisibilityReadContext<'_> {
        VisibilityReadContext::new(self)
    }
}
