mod entries;
mod handles;
mod segmented;
mod slot;

pub(crate) use handles::{DependencySetId, SubscriberSetId};
#[cfg(test)]
pub(crate) use segmented::checked_segment_component_for_test;
pub(crate) use segmented::{DependencyEdgeStore, SubscriberEdgeStore};
pub(crate) use slot::Slot;
