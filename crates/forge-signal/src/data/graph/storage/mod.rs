mod edge_store;
mod entries;
#[cfg(feature = "parallel")]
mod parallel;
mod slot;

#[cfg(test)]
pub(crate) use edge_store::checked_segment_component_for_test;
pub(crate) use edge_store::{
    DependencyEdgeStore, DependencySetId, SubscriberEdgeStore, SubscriberSetId,
};
pub(crate) use slot::Slot;
