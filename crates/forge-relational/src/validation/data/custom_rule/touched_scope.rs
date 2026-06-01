mod collection;
mod planned_records;

pub use collection::TouchedStructuralSet;
pub use planned_records::{
    CustomInvariantTouchedSummary, PlannedRelationEndpointUpdate, StructuralCountView,
};

pub(crate) use collection::collect_touched_structural_set;
