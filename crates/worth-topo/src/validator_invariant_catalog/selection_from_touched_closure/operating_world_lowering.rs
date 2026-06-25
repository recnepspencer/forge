use forge_query::facade::ForgeQueryGraphObligationOperatingWorldDescriptor;

use crate::topology_operators::{
    TopologyTouchedOperatingWorld, TopologyTouchedOperatingWorldPosture,
};

pub(super) fn query_operating_world_descriptor_from_topology_world(
    world: &TopologyTouchedOperatingWorld,
) -> ForgeQueryGraphObligationOperatingWorldDescriptor {
    match world.posture() {
        TopologyTouchedOperatingWorldPosture::Mainline => {
            ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority()
        }
        TopologyTouchedOperatingWorldPosture::Branch => {
            ForgeQueryGraphObligationOperatingWorldDescriptor::branch()
        }
        TopologyTouchedOperatingWorldPosture::Preview => {
            ForgeQueryGraphObligationOperatingWorldDescriptor::preview()
        }
        TopologyTouchedOperatingWorldPosture::ConfiguredDomainHandle => {
            ForgeQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle()
        }
    }
}
