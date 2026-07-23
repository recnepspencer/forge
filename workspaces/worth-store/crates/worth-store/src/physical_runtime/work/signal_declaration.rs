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
    declaration: PhysicalWorkSignalDeclaration,
    node: NodeId,
    capability: AsyncNodeCapabilityDeclaration,
}

pub(in crate::physical_runtime) struct InstalledPhysicalSignalTopology {
    sources: Box<[NodeId]>,
    capabilities: Box<[InstalledPhysicalSignalCapability]>,
}

pub(in crate::physical_runtime) struct InstalledPhysicalSignalCapability {
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
}

impl PendingPhysicalSignalTopology {
    pub(in crate::physical_runtime) fn build(
        graph: &mut SignalGraph,
        bindings: &PhysicalSignalAspectBindingSet,
        route: PhysicalSignalAspectBindingDigest,
    ) -> Result<Self, SignalError> {
        let sources = bindings
            .bindings()
            .iter()
            .map(|binding| {
                graph
                    .node()
                    .with_contract(
                        NodeContract::reads(AspectMask::EMPTY)
                            .with_produces(binding.signal_aspect()),
                    )
                    .build()
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let mut capabilities = Vec::with_capacity(PHYSICAL_ASYNC_CAPABILITIES.len());
        for spec in PHYSICAL_ASYNC_CAPABILITIES {
            let declaration = PhysicalWorkSignalDeclaration::from_spec(spec);
            let consumed = bindings
                .bindings()
                .iter()
                .filter(|binding| {
                    binding.digest() == route
                        && binding.serves_family(declaration.family)
                        && matches!(
                            binding.role(),
                            PhysicalSignalAspectRole::Dependency
                                | PhysicalSignalAspectRole::DependencyAndOutput
                        )
                })
                .fold(AspectMask::EMPTY, |mask, binding| {
                    mask.union(AspectMask::from_aspect(binding.signal_aspect()))
                });
            let node = graph
                .node()
                .with_contract(NodeContract::reads(consumed).with_produces(AspectMask::EMPTY))
                .on_demand()
                .build();
            graph.set_dependencies(
                node,
                bindings
                    .bindings()
                    .iter()
                    .zip(sources.iter().copied())
                    .filter(|(binding, _)| {
                        binding.digest() == route
                            && binding.serves_family(declaration.family)
                            && matches!(
                                binding.role(),
                                PhysicalSignalAspectRole::Dependency
                                    | PhysicalSignalAspectRole::DependencyAndOutput
                            )
                    })
                    .map(|(binding, source)| {
                        let subscription = binding
                            .projection_subscription()
                            .expect("profile admission proved dependency projection mask");
                        debug_assert!(subscription
                            .signal_mask()
                            .contains(AspectMask::from_aspect(binding.signal_aspect())));
                        match subscription.partition() {
                            Some(partition) => DependencyEdge::with_partition_scope(
                                source,
                                binding.signal_aspect(),
                                partition.clone(),
                            ),
                            None => DependencyEdge::new(source, binding.signal_aspect()),
                        }
                    }),
            )?;
            let payload = AsyncNodePayloadContract::new(AsyncNodePayloadContractId::new(
                declaration.payload_contract_id,
            ))
            .with_max_payload_bytes(declaration.max_payload_bytes);
            capabilities.push(PendingPhysicalSignalCapability {
                declaration,
                node,
                capability: super::profile::PhysicalSignalPolicySelection::apply(
                    AsyncNodeCapabilityDeclaration::new(node, payload),
                ),
            });
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

    pub(in crate::physical_runtime) fn source_for_slot(&self, slot: usize) -> Option<NodeId> {
        self.sources.get(slot).copied()
    }

    pub(in crate::physical_runtime) fn capability(
        &self,
        family: PhysicalWorkSignalFamily,
    ) -> Option<&AsyncCapableNode> {
        self.capabilities
            .iter()
            .find(|capability| capability.family == family)
            .map(|capability| &capability.node)
    }
}
