mod materialization;
mod projection;
mod reader;
mod visibility;

pub use projection::{EntityRecordProjection, RelationRecordProjection, VisibilityProjectionView};
pub use reader::VisibilityReadContext;

use crate::logic::runtime::RelationalRuntime;

impl RelationalRuntime {
    pub(crate) fn visibility_reads(&self) -> VisibilityReadContext<'_> {
        VisibilityReadContext::new(self)
    }
}
