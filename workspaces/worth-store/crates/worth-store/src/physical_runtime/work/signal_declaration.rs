use worth_signal::facade::adapters::NodeContract;
use worth_signal::facade::core::AsyncCapableNode;
use worth_signal::facade::{
    AspectMask, AsyncNodeCapabilityDeclaration, AsyncNodePayloadContract,
    AsyncNodePayloadContractId, DependencyEdge, NodeId, SignalError, SignalGraph, SignalRuntime,
};

use super::{
    PhysicalSignalAspectBindingDigest, PhysicalSignalAspectBindingSet, PhysicalSignalAspectRole,
    PhysicalWorkSignalFamily, PHYSICAL_ASYNC_CAPABILITIES,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkSignalDeclaration {
    family: PhysicalWorkSignalFamily,
    payload_contract_id: u64,
    max_payload_bytes: u64,
}

pub(in crate::physical_runtime) struct PendingPhysicalSignalTopology {
    sources: Box<[NodeId]>,
    capabilities: Vec<PendingPhysicalSignalCapability>,
}

struct PendingPhysicalSignalCapability {
    route: PhysicalSignalAspectBindingDigest,
    declaration: PhysicalWorkSignalDeclaration,
    node: NodeId,
    capability: AsyncNodeCapabilityDeclaration,
}

pub(in crate::physical_runtime) struct InstalledPhysicalSignalTopology {
    sources: Box<[NodeId]>,
    capabilities: Box<[InstalledPhysicalSignalCapability]>,
}

pub(in crate::physical_runtime) struct InstalledPhysicalSignalCapability {
    route: PhysicalSignalAspectBindingDigest,
    family: PhysicalWorkSignalFamily,
    node: AsyncCapableNode,
}

impl PhysicalWorkSignalDeclaration {
    pub const fn family(self) -> PhysicalWorkSignalFamily {
        self.family
    }

    pub const fn payload_contract_id(self) -> u64 {
        self.payload_contract_id
    }

    pub const fn max_payload_bytes(self) -> u64 {
        self.max_payload_bytes
    }

    fn from_spec(spec: super::profile::PhysicalAsyncCapabilitySpec) -> Self {
        Self {
            family: spec.family(),
            payload_contract_id: spec.contract_id(),
            max_payload_bytes: spec.max_payload_bytes(),
        }
    }

    pub(in crate::physical_runtime) fn for_family(
        family: PhysicalWorkSignalFamily,
    ) -> Option<Self> {
        PHYSICAL_ASYNC_CAPABILITIES
            .into_iter()
            .find(|spec| spec.family() == family)
            .map(Self::from_spec)
    }
}

impl PendingPhysicalSignalTopology {
    pub(in crate::physical_runtime) fn build(
        graph: &mut SignalGraph,
        bindings: &PhysicalSignalAspectBindingSet,
    ) -> Result<Self, SignalError> {
        let sources = declare_binding_sources(graph, bindings);
        let mut capabilities = Vec::with_capacity(
            bindings
                .len()
                .saturating_mul(PHYSICAL_ASYNC_CAPABILITIES.len()),
        );
        for (slot, binding) in bindings.bindings().iter().enumerate() {
            for spec in PHYSICAL_ASYNC_CAPABILITIES {
                if let Some(capability) =
                    declare_binding_capability(graph, sources[slot], binding, spec)?
                {
                    capabilities.push(capability);
                }
            }
        }
        Ok(Self {
            sources,
            capabilities,
        })
    }

    pub(in crate::physical_runtime) fn attach<D, I, E, Ctx, T>(
        self,
        runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
    ) -> Result<InstalledPhysicalSignalTopology, SignalError>
    where
        D: Copy + Ord + std::fmt::Debug + 'static,
        I: Copy + Ord,
        T: Copy + Ord,
    {
        let mut installed = Vec::with_capacity(self.capabilities.len());
        for pending in self.capabilities {
            let node = runtime.attach_async_capability(pending.capability)?;
            debug_assert_eq!(node.node(), pending.node);
            installed.push(InstalledPhysicalSignalCapability {
                route: pending.route,
                family: pending.declaration.family,
                node,
            });
        }
        Ok(InstalledPhysicalSignalTopology {
            sources: self.sources,
            capabilities: installed.into_boxed_slice(),
        })
    }
}

fn declare_binding_sources(
    graph: &mut SignalGraph,
    bindings: &PhysicalSignalAspectBindingSet,
) -> Box<[NodeId]> {
    bindings
        .bindings()
        .iter()
        .map(|binding| {
            graph
                .node()
                .with_contract(
                    NodeContract::reads(AspectMask::EMPTY).with_produces(binding.signal_aspect()),
                )
                .build()
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn declare_binding_capability(
    graph: &mut SignalGraph,
    source: NodeId,
    binding: &super::PhysicalSignalAspectBinding,
    spec: super::profile::PhysicalAsyncCapabilitySpec,
) -> Result<Option<PendingPhysicalSignalCapability>, SignalError> {
    let declaration = PhysicalWorkSignalDeclaration::from_spec(spec);
    if !binding.serves_family(declaration.family) {
        return Ok(None);
    }
    let is_dependency = matches!(
        binding.role(),
        PhysicalSignalAspectRole::Dependency | PhysicalSignalAspectRole::DependencyAndOutput
    );
    let consumed = if is_dependency {
        binding.signal_mask()
    } else {
        AspectMask::EMPTY
    };
    let node = graph
        .node()
        .with_contract(NodeContract::reads(consumed).with_produces(AspectMask::EMPTY))
        .on_demand()
        .build();
    if is_dependency {
        declare_dependency_edge(graph, source, node, binding)?;
    }
    let payload = AsyncNodePayloadContract::new(AsyncNodePayloadContractId::new(
        declaration.payload_contract_id,
    ))
    .with_max_payload_bytes(declaration.max_payload_bytes);
    Ok(Some(PendingPhysicalSignalCapability {
        route: binding.digest(),
        declaration,
        node,
        capability: super::profile::PhysicalSignalPolicySelection::apply(
            AsyncNodeCapabilityDeclaration::new(node, payload),
        ),
    }))
}

fn declare_dependency_edge(
    graph: &mut SignalGraph,
    source: NodeId,
    node: NodeId,
    binding: &super::PhysicalSignalAspectBinding,
) -> Result<(), SignalError> {
    let subscription = binding
        .projection_subscription()
        .expect("profile admission proved dependency projection mask");
    debug_assert!(subscription.signal_mask().contains(binding.signal_mask()));
    graph.set_dependencies(
        node,
        [match subscription.partition() {
            Some(partition) => DependencyEdge::with_partition_scope(
                source,
                binding.signal_aspect(),
                partition.clone(),
            ),
            None => DependencyEdge::new(source, binding.signal_aspect()),
        }],
    )
}

impl InstalledPhysicalSignalTopology {
    pub(in crate::physical_runtime) fn family_for_node(
        &self,
        node: NodeId,
    ) -> Option<PhysicalWorkSignalFamily> {
        self.capabilities
            .iter()
            .find(|capability| capability.node.node() == node)
            .map(|capability| capability.family)
    }

    pub(in crate::physical_runtime) fn route_for_node(
        &self,
        node: NodeId,
    ) -> Option<PhysicalSignalAspectBindingDigest> {
        self.capabilities
            .iter()
            .find(|capability| capability.node.node() == node)
            .map(|capability| capability.route)
    }

    pub(in crate::physical_runtime) fn source_for_slot(&self, slot: usize) -> Option<NodeId> {
        self.sources.get(slot).copied()
    }

    pub(in crate::physical_runtime) fn capability(
        &self,
        route: PhysicalSignalAspectBindingDigest,
        family: PhysicalWorkSignalFamily,
    ) -> Option<&AsyncCapableNode> {
        self.capabilities
            .iter()
            .find(|capability| capability.route == route && capability.family == family)
            .map(|capability| &capability.node)
    }
}
