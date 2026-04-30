use crate::facade::*;

pub(crate) type AsyncNodeTestRuntime = SignalRuntime<(), (), (), (), ()>;

pub(crate) fn async_node_capability_declaration(node: NodeId) -> AsyncNodeCapabilityDeclaration {
    AsyncNodeCapabilityDeclaration::new(
        node,
        AsyncNodePayloadContract::new(AsyncNodePayloadContractId::new(7))
            .with_max_payload_bytes(1024),
    )
}

pub(crate) fn async_node_capability_with_dependents(
    node: NodeId,
    dependents: impl IntoIterator<Item = NodeId>,
) -> AsyncNodeCapabilityDeclaration {
    async_node_capability_declaration(node).with_declared_dependent_cancellation_nodes(dependents)
}
