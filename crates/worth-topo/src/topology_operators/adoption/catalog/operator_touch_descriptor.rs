use forge_query::facade::{
    ForgeQueryAspectMutationOperation, ForgeQueryAspectTouch, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchDescriptorDenial, ForgeQueryGraphTouchLifecycleFamily,
    ForgeQueryMutationFamily,
};
use schema::facade::platform::aspects::{Aspect, TopologyAspect};

use super::TOPOLOGY_OPERATOR_RELATION_COLLECTION;

pub const TOPOLOGY_REWIRE_LOOP_SUCCESSOR_ASPECT_OPERATION: &str = "update:topology.loop.successor";
pub const TOPOLOGY_REWIRE_LOOP_SUCCESSOR_ASPECT_PATH: &str = "topology.loop.successor";

pub fn topology_operator_relation_touch_descriptor(
) -> Result<ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial> {
    ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
        TOPOLOGY_OPERATOR_RELATION_COLLECTION,
        ForgeQueryMutationFamily::Update,
        Some(ForgeQueryGraphTouchLifecycleFamily::VerifiedExistingTargetRetarget),
        [topology_rewire_loop_successor_aspect_operation()],
        [topology_rewire_loop_successor_aspect_touch()],
    )
}

pub fn topology_operator_command_batch_equivalent_touch_descriptor(
) -> Result<ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial> {
    topology_operator_relation_touch_descriptor()
}

pub(crate) fn topology_rewire_loop_successor_aspect_operation() -> ForgeQueryAspectMutationOperation
{
    ForgeQueryAspectMutationOperation::set(topology_rewire_loop_successor_aspect_touch())
}

pub(crate) fn topology_rewire_loop_successor_aspect_touch() -> ForgeQueryAspectTouch {
    ForgeQueryAspectTouch::whole_aspect(Aspect::Topology(TopologyAspect::Structure).aspect_key())
}
