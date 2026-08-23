use super::declarations::{
    UiNativePhysicalSignalAspect, UiNativePhysicalSignalDeclarations,
    UiNativePhysicalSignalOperation, PHYSICAL_SIGNAL_ASPECT_COUNT,
};
use super::locality::UiNativePhysicalSignalLocality;
use super::routing::UiNativePhysicalSignalWork;
use worth_signal::facade::adapters::{InvalidationPerformedCounter, NodeContract};
use worth_signal::facade::core::AsyncCapableNode;
use worth_signal::facade::specialist::EvaluationOutput;
use worth_signal::facade::{
    mark_dirty_with_regions, AspectMask, AspectVersion, EvaluationContext, NodeEvaluationResult,
    NodeId, ResourceRequestHandle, SignalError, SignalGraph, SignalRuntime,
};

const PHYSICAL_SIGNAL_CURRENT_REQUEST_CAPACITY: usize =
    super::declarations::PHYSICAL_SIGNAL_ROUTE_CAPACITY;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct UiNativePhysicalSignalCurrentRequest {
    locality: UiNativePhysicalSignalLocality,
    handle: ResourceRequestHandle,
}

pub(super) struct UiNativePhysicalSignalContext {
    pub(super) clock_revision: u64,
    aspect_revisions: [u64; PHYSICAL_SIGNAL_ASPECT_COUNT],
    fact_revision: u64,
    exact_locality: Option<UiNativePhysicalSignalLocality>,
    current_requests:
        [Option<UiNativePhysicalSignalCurrentRequest>; PHYSICAL_SIGNAL_CURRENT_REQUEST_CAPACITY],
}

impl UiNativePhysicalSignalContext {
    fn new() -> Self {
        Self {
            clock_revision: 0,
            aspect_revisions: [0; PHYSICAL_SIGNAL_ASPECT_COUNT],
            fact_revision: 0,
            exact_locality: None,
            current_requests: [None; PHYSICAL_SIGNAL_CURRENT_REQUEST_CAPACITY],
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct UiNativePhysicalSignalPerformed {
    locality: UiNativePhysicalSignalLocality,
    fact_revision: u64,
    read_scopes: u8,
    evaluated_nodes: u64,
}

impl UiNativePhysicalSignalPerformed {
    pub(super) const fn work(self) -> UiNativePhysicalSignalWork {
        self.locality.work()
    }

    pub(super) const fn locality(self) -> UiNativePhysicalSignalLocality {
        self.locality
    }

    pub(super) const fn fact_revision(self) -> u64 {
        self.fact_revision
    }

    pub(super) const fn read_scopes(self) -> u8 {
        self.read_scopes
    }

    pub(super) const fn evaluated_nodes(self) -> u64 {
        self.evaluated_nodes
    }
}

pub(super) struct UiNativePhysicalSignalTopology {
    aspects: [worth_signal::facade::Aspect; PHYSICAL_SIGNAL_ASPECT_COUNT],
    sources: [NodeId; PHYSICAL_SIGNAL_ASPECT_COUNT],
    pub(super) operations: [[AsyncCapableNode; PHYSICAL_SIGNAL_CURRENT_REQUEST_CAPACITY]; 3],
    operation_reads: [AspectMask; 3],
}

pub(super) struct UiNativePhysicalSignalGraph {
    pub(super) runtime: SignalRuntime<(), (), (), UiNativePhysicalSignalContext, ()>,
    pub(super) context: UiNativePhysicalSignalContext,
    pub(super) topology: UiNativePhysicalSignalTopology,
    performed_transitions: u64,
    performed_nodes: u64,
    last_performed: Option<UiNativePhysicalSignalPerformed>,
}

impl UiNativePhysicalSignalGraph {
    pub(super) fn build(declarations: UiNativePhysicalSignalDeclarations) -> Self {
        let (mut runtime, topology) = build_runtime(declarations);
        let context = UiNativePhysicalSignalContext::new();
        runtime
            .evaluate_dirty(&context, &|view| evaluate_node(&topology, view))
            .unwrap_or_else(|error| panic!("physical Signal graph must initialize: {error}"));
        Self {
            runtime,
            context,
            topology,
            performed_transitions: 0,
            performed_nodes: 0,
            last_performed: None,
        }
    }

    pub(super) fn perform_transition(
        &mut self,
        operation: UiNativePhysicalSignalOperation,
        work: UiNativePhysicalSignalWork,
    ) -> Result<UiNativePhysicalSignalPerformed, ()> {
        let reads = self.topology.operation_reads[operation.index()];
        let locality = UiNativePhysicalSignalLocality::new(operation, work);
        let fact_revision = self.context.fact_revision.checked_add(1).ok_or(())?;
        self.context.fact_revision = fact_revision;
        self.context.exact_locality = Some(locality);
        for (index, aspect) in self.topology.aspects.iter().copied().enumerate() {
            if reads.contains(AspectMask::from([aspect])) {
                self.context.aspect_revisions[index] = fact_revision;
            }
        }

        let context = &self.context;
        let topology = &self.topology;
        let (_, receipt) = self
            .runtime
            .observe_invalidation_execution(|runtime| {
                for (index, aspect) in topology.aspects.iter().copied().enumerate() {
                    if reads.contains(AspectMask::from([aspect])) {
                        let typed_aspect = UiNativePhysicalSignalAspect::from_index(index);
                        mark_dirty_with_regions(
                            runtime.graph_mut(),
                            topology.sources[index],
                            aspect,
                            &[locality.changed_region(typed_aspect)],
                        )?;
                    }
                }
                runtime.evaluate_dirty(context, &|view| evaluate_node(topology, view))
            })
            .map_err(|_| ())?;
        let evaluated = receipt
            .realized_counters()
            .value(InvalidationPerformedCounter::NodesEvaluated);
        if evaluated == 0 {
            return Err(());
        }
        self.performed_transitions = self.performed_transitions.saturating_add(1);
        self.performed_nodes = self.performed_nodes.saturating_add(evaluated);
        let performed = UiNativePhysicalSignalPerformed {
            locality,
            fact_revision,
            read_scopes: locality.scopes_for(reads).iter().flatten().count() as u8,
            evaluated_nodes: evaluated,
        };
        self.last_performed = Some(performed);
        Ok(performed)
    }

    pub(super) fn record_current(
        &mut self,
        operation: UiNativePhysicalSignalOperation,
        work: UiNativePhysicalSignalWork,
        handle: ResourceRequestHandle,
    ) -> Result<(), ()> {
        if self
            .context
            .current_requests
            .iter()
            .flatten()
            .any(|request| request.locality.work() == work || request.handle == handle)
        {
            return Err(());
        }
        let slot = self
            .context
            .current_requests
            .iter_mut()
            .find(|slot| slot.is_none())
            .ok_or(())?;
        *slot = Some(UiNativePhysicalSignalCurrentRequest {
            locality: UiNativePhysicalSignalLocality::new(operation, work),
            handle,
        });
        Ok(())
    }

    pub(super) fn replace_current(
        &mut self,
        handle: ResourceRequestHandle,
        predecessor: UiNativePhysicalSignalWork,
        successor: UiNativePhysicalSignalWork,
    ) -> bool {
        if self
            .context
            .current_requests
            .iter()
            .flatten()
            .any(|request| request.locality.work() == successor)
        {
            return false;
        }
        let Some(request) = self
            .context
            .current_requests
            .iter_mut()
            .flatten()
            .find(|request| request.handle == handle && request.locality.work() == predecessor)
        else {
            return false;
        };
        request.locality =
            UiNativePhysicalSignalLocality::new(request.locality.operation(), successor);
        true
    }

    pub(super) fn replace_current_handle(
        &mut self,
        work: UiNativePhysicalSignalWork,
        previous: ResourceRequestHandle,
        successor: ResourceRequestHandle,
    ) -> bool {
        let Some(request) = self
            .context
            .current_requests
            .iter_mut()
            .flatten()
            .find(|request| request.handle == previous && request.locality.work() == work)
        else {
            return false;
        };
        request.handle = successor;
        true
    }

    pub(super) fn remove_current(
        &mut self,
        work: UiNativePhysicalSignalWork,
        handle: ResourceRequestHandle,
    ) -> bool {
        let Some(slot) = self.context.current_requests.iter_mut().find(|slot| {
            slot.is_some_and(|request| request.locality.work() == work && request.handle == handle)
        }) else {
            return false;
        };
        *slot = None;
        true
    }

    pub(super) fn contains_current(
        &self,
        work: UiNativePhysicalSignalWork,
        handle: ResourceRequestHandle,
    ) -> bool {
        self.context
            .current_requests
            .iter()
            .flatten()
            .any(|request| request.locality.work() == work && request.handle == handle)
    }

    pub(super) fn contains_work(&self, work: UiNativePhysicalSignalWork) -> bool {
        self.context
            .current_requests
            .iter()
            .flatten()
            .any(|request| request.locality.work() == work)
    }

    pub(super) const fn performed_transitions(&self) -> u64 {
        self.performed_transitions
    }

    pub(super) const fn performed_nodes(&self) -> u64 {
        self.performed_nodes
    }

    pub(super) const fn last_performed(&self) -> Option<UiNativePhysicalSignalPerformed> {
        self.last_performed
    }
}

fn build_runtime(
    declarations: UiNativePhysicalSignalDeclarations,
) -> (
    SignalRuntime<(), (), (), UiNativePhysicalSignalContext, ()>,
    UiNativePhysicalSignalTopology,
) {
    let mut graph = SignalGraph::new();
    let aspects = UiNativePhysicalSignalAspect::all();
    let sources = std::array::from_fn(|index| {
        graph
            .node()
            .partitioned_output()
            .with_contract(NodeContract::reads(AspectMask::EMPTY).with_produces(aspects[index]))
            .build()
    });
    let operations = std::array::from_fn(|index| {
        std::array::from_fn(|_| {
            let node = graph
                .node()
                .with_contract(NodeContract::reads(declarations.resources[index].reads()))
                .on_demand()
                .build();
            let declaration = declarations.resources[index].capability(node);
            (node, declaration)
        })
    });
    let mut runtime = SignalRuntime::build_for::<UiNativePhysicalSignalContext>(graph);
    let capabilities = operations.map(|slots| {
        slots.map(|(node, declaration)| {
            runtime
                .attach_async_capability(declaration)
                .unwrap_or_else(|error| {
                    panic!("physical Signal capability {node:?} must attach: {error}")
                })
        })
    });
    (
        runtime,
        UiNativePhysicalSignalTopology {
            aspects,
            sources,
            operations: capabilities,
            operation_reads: std::array::from_fn(|index| declarations.resources[index].reads()),
        },
    )
}

fn evaluate_node(
    topology: &UiNativePhysicalSignalTopology,
    view: &mut EvaluationContext<'_, UiNativePhysicalSignalContext>,
) -> Result<EvaluationOutput, SignalError> {
    if let Some(index) = topology
        .sources
        .iter()
        .position(|source| *source == view.node())
    {
        let result = NodeEvaluationResult::from_version(AspectVersion::from_updates([(
            topology.aspects[index],
            view.domain().aspect_revisions[index],
        )]));
        let result = if let Some(locality) = view.domain().exact_locality {
            result.with_changed_region(
                locality.changed_region(UiNativePhysicalSignalAspect::from_index(index)),
            )
        } else {
            result
        };
        return Ok(view.finish(result));
    }
    if let Some(index) = topology.operations.iter().position(|slots| {
        slots
            .iter()
            .any(|operation| operation.node() == view.node())
    }) {
        let reads = topology.operation_reads[index];
        let operation = UiNativePhysicalSignalOperation::from_index(index);
        let Some(locality) = view.domain().exact_locality else {
            return Ok(view.finish(NodeEvaluationResult::from_version(AspectVersion::zero())));
        };
        if locality.operation() != operation {
            return Err(SignalError::internal(
                "physical work fact routed to the wrong operation",
            ));
        }
        for (source_index, source) in topology.sources.iter().enumerate() {
            let aspect = topology.aspects[source_index];
            if reads.contains(AspectMask::from([aspect])) {
                if view.domain().aspect_revisions[source_index] != view.domain().fact_revision {
                    return Err(SignalError::internal(
                        "physical work fact version diverged from its exact basis",
                    ));
                }
                let _ = view.read_partitioned_aspect_version(
                    *source,
                    aspect,
                    locality.subscription(UiNativePhysicalSignalAspect::from_index(source_index)),
                )?;
            }
        }
    }
    Ok(view.finish(NodeEvaluationResult::from_version(AspectVersion::zero())))
}
