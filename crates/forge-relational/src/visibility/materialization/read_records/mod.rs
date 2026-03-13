mod materialization;
mod projection;
mod reader;
mod visibility;

pub use projection::{
    EntityRecordProjection, ProjectionAspect, RelationRecordProjection, VisibilityProjectionView,
};
pub use reader::VisibilityReadContext;

use crate::logic::runtime::RelationalRuntime;

impl RelationalRuntime {
    pub fn visibility_reads(&self) -> VisibilityReadContext<'_> {
        VisibilityReadContext::new(self)
    }
}
