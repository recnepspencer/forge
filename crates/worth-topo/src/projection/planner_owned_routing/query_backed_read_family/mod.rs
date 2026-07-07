mod admitted_route;
mod reuse_posture;
mod selected_route;

#[cfg(test)]
mod tests;

#[cfg(test)]
pub use admitted_route::admit_topology_query_backed_consumer_cutover;
#[cfg(test)]
pub(crate) use admitted_route::admit_topology_query_backed_read_family_route;
pub(crate) use crate::projection::query_backed_consumer_cutover::current_topology_query_backed_consumer_cutover;
#[cfg(test)]
pub(crate) use crate::projection::query_backed_consumer_cutover::current_topology_query_backed_read_family_route_input;
#[cfg(test)]
pub use crate::projection::query_backed_consumer_cutover::{
    current_query_backed_consumer_residue_manifest, QueryBackedConsumerResidueDisposition,
    QueryBackedConsumerResidueOwner,
};
pub use reuse_posture::TopologyReadModelReusePosture;
#[cfg(test)]
pub(crate) use selected_route::TopologyQueryBackedConsumerCutover;
