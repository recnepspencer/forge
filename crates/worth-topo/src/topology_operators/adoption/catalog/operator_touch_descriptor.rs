use forge_query::facade::{
    ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial,
    ForgeQueryGraphTouchLifecycleFamily, ForgeQueryMutationFamily,
};

use super::TOPOLOGY_OPERATOR_RELATION_COLLECTION;

pub const TOPOLOGY_REWIRE_LOOP_SUCCESSOR_ASPECT_OPERATION: &str = "update:topology.loop.successor";
pub const TOPOLOGY_REWIRE_LOOP_SUCCESSOR_ASPECT_PATH: &str = "topology.loop.successor";

pub fn topology_operator_relation_touch_descriptor(
) -> Result<ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial> {
    ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
        TOPOLOGY_OPERATOR_RELATION_COLLECTION,
        ForgeQueryMutationFamily::Update,
        Some(ForgeQueryGraphTouchLifecycleFamily::VerifiedExistingTargetRetarget),
        [TOPOLOGY_REWIRE_LOOP_SUCCESSOR_ASPECT_OPERATION],
        [TOPOLOGY_REWIRE_LOOP_SUCCESSOR_ASPECT_PATH],
    )
}

pub fn topology_operator_command_batch_equivalent_touch_descriptor(
) -> Result<ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial> {
    topology_operator_relation_touch_descriptor()
}
