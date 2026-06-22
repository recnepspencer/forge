mod counters;
mod group;
mod group_key;
mod identity;
mod interval_grouping;
mod point_grouping;

pub use counters::PlanarBooleanEventGroupingCounters;
#[cfg(test)]
pub(crate) use group::PlanarBooleanEventGroupInput;
pub use group::{PlanarBooleanEventGroup, PlanarBooleanEventGroupKind};

pub(crate) use interval_grouping::group_interval_events;
pub(crate) use point_grouping::group_point_events;
