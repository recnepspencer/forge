mod handles;
mod entries;
#[cfg(feature = "parallel")]
mod parallel;
mod segmented;
mod slot;

#[cfg(test)]
pub(crate) use segmented::checked_segment_component_for_test;
pub(crate) use handles::{DependencySetId, SubscriberSetId};
pub(crate) use segmented::{DependencyEdgeStore, SubscriberEdgeStore};
pub(crate) use slot::Slot;
