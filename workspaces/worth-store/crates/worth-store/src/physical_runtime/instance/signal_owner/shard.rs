use std::{
    sync::{
        mpsc::{self, SyncSender},
        Arc,
    },
    thread::JoinHandle,
};

use worth_signal::facade::{
    AspectVersion, NodeEvaluationResult, ResourceRequestHandle, RuntimeClockBasis, SignalError,
    SignalGraph, SignalRuntime,
};

use crate::physical_runtime::work::{
    AdmittedPhysicalWork, BlockedPhysicalWork, InstalledPhysicalSignalTopology,
    PendingPhysicalSignalTopology, PhysicalSignalAspectBindingDigest,
    PhysicalSignalAspectBindingSet, PhysicalWorkAspectDelta, PhysicalWorkPreEffectDenial,
    PhysicalWorkReadiness, ReadyPhysicalWork,
};

use super::{PhysicalSignalConstructionFailure, PhysicalSignalDeltaApplicationFailure};

const SHARD_COMMAND_CAPACITY: usize = 8;

struct PhysicalSignalContext {
    version: u64,
}

enum PhysicalSignalShardCommand {
    Observe(SyncSender<RuntimeClockBasis>),
    Apply(
        PhysicalWorkAspectDelta,
        SyncSender<Result<(), PhysicalSignalDeltaApplicationFailure>>,
    ),
    Request(
        AdmittedPhysicalWork,
        SyncSender<Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial>>,
    ),
    Revalidate(
        AdmittedPhysicalWork,
        ResourceRequestHandle,
        SyncSender<Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial>>,
    ),
    Dispose,
}

pub(super) struct PhysicalSignalShardOwner {
    route: PhysicalSignalAspectBindingDigest,
    commands: SyncSender<PhysicalSignalShardCommand>,
    worker: Option<JoinHandle<()>>,
}

struct PhysicalSignalShard {
    route: PhysicalSignalAspectBindingDigest,
    bindings: Arc<PhysicalSignalAspectBindingSet>,
    runtime: SignalRuntime<(), (), (), PhysicalSignalContext, ()>,
    topology: InstalledPhysicalSignalTopology,
    context: PhysicalSignalContext,
}

impl PhysicalSignalShardOwner {
    pub(super) fn spawn(
        route: PhysicalSignalAspectBindingDigest,
        bindings: Arc<PhysicalSignalAspectBindingSet>,
    ) -> Result<Self, PhysicalSignalConstructionFailure> {
        let (commands, receiver) = mpsc::sync_channel(SHARD_COMMAND_CAPACITY);
        let (ready_sender, ready_receiver) = mpsc::sync_channel(0);
        let worker = std::thread::Builder::new()
            .name("worth-store-physical-signal-shard".to_owned())
            .spawn(move || {
                let result = PhysicalSignalShard::build(route, bindings);
                let Ok(mut shard) = result else {
                    let _ = ready_sender.send(result.map(|_| ()));
                    return;
                };
                if ready_sender.send(Ok(())).is_err() {
                    return;
                }
                while let Ok(command) = receiver.recv() {
                    match command {
                        PhysicalSignalShardCommand::Observe(reply) => {
                            let _ = reply.send(shard.runtime.clock_basis());
                        }
                        PhysicalSignalShardCommand::Apply(delta, reply) => {
                            let _ = reply.send(shard.apply_delta(&delta));
                        }
                        PhysicalSignalShardCommand::Request(admitted, reply) => {
                            let _ = reply.send(shard.request(admitted));
                        }
                        PhysicalSignalShardCommand::Revalidate(admitted, active, reply) => {
                            let _ = reply.send(shard.revalidate(admitted, active));
                        }
                        PhysicalSignalShardCommand::Dispose => break,
                    }
                }
            })
            .map_err(|_| PhysicalSignalConstructionFailure::WorkerSpawnRejected)?;
        match ready_receiver.recv() {
            Ok(Ok(())) => Ok(Self {
                route,
                commands,
                worker: Some(worker),
            }),
            Ok(Err(failure)) => {
                let _ = worker.join();
                Err(failure)
            }
            Err(_) => {
                let _ = worker.join();
                Err(PhysicalSignalConstructionFailure::WorkerReadinessLost)
            }
        }
    }

    pub(super) const fn route(&self) -> PhysicalSignalAspectBindingDigest {
        self.route
    }

    pub(super) fn clock_basis(&self) -> Option<RuntimeClockBasis> {
        let (reply, observed) = mpsc::sync_channel(0);
        self.commands
            .send(PhysicalSignalShardCommand::Observe(reply))
            .ok()?;
        observed.recv().ok()
    }

    pub(super) fn apply_delta(
        &self,
        delta: PhysicalWorkAspectDelta,
    ) -> Result<(), PhysicalSignalDeltaApplicationFailure> {
        let (reply, observed) = mpsc::sync_channel(0);
        self.commands
            .send(PhysicalSignalShardCommand::Apply(delta, reply))
            .map_err(|_| PhysicalSignalDeltaApplicationFailure::OwnerUnavailable)?;
        observed
            .recv()
            .map_err(|_| PhysicalSignalDeltaApplicationFailure::OwnerUnavailable)?
    }

    pub(super) fn request(
        &self,
        admitted: AdmittedPhysicalWork,
    ) -> Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial> {
        let (reply, observed) = mpsc::sync_channel(0);
        self.commands
            .send(PhysicalSignalShardCommand::Request(admitted, reply))
            .map_err(|_| PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?;
        observed
            .recv()
            .map_err(|_| PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?
    }

    pub(super) fn revalidate(
        &self,
        admitted: AdmittedPhysicalWork,
        active: ResourceRequestHandle,
    ) -> Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial> {
        let (reply, observed) = mpsc::sync_channel(0);
        self.commands
            .send(PhysicalSignalShardCommand::Revalidate(
                admitted, active, reply,
            ))
            .map_err(|_| PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?;
        observed
            .recv()
            .map_err(|_| PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?
    }

    pub(super) fn stop(&mut self) {
        let _ = self.commands.send(PhysicalSignalShardCommand::Dispose);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

impl Drop for PhysicalSignalShardOwner {
    fn drop(&mut self) {
        self.stop();
    }
}

impl PhysicalSignalShard {
    fn build(
        route: PhysicalSignalAspectBindingDigest,
        bindings: Arc<PhysicalSignalAspectBindingSet>,
    ) -> Result<Self, PhysicalSignalConstructionFailure> {
        let mut graph = SignalGraph::new();
        let pending = PendingPhysicalSignalTopology::build(&mut graph, &bindings, route)
            .map_err(|_| PhysicalSignalConstructionFailure::CapabilityDeclarationRejected)?;
        let mut runtime = SignalRuntime::build_for::<PhysicalSignalContext>(graph);
        let topology = pending
            .attach(&mut runtime)
            .map_err(|_| PhysicalSignalConstructionFailure::CapabilityDeclarationRejected)?;
        let mut shard = Self {
            route,
            bindings,
            runtime,
            topology,
            context: PhysicalSignalContext { version: 0 },
        };
        shard
            .evaluate_dirty()
            .map_err(|_| PhysicalSignalConstructionFailure::DependencyInitializationRejected)?;
        Ok(shard)
    }

    fn apply_delta(
        &mut self,
        delta: &PhysicalWorkAspectDelta,
    ) -> Result<(), PhysicalSignalDeltaApplicationFailure> {
        let slot = delta.signal_aspect().index();
        let source = self
            .bindings
            .binding_for_slot(slot)
            .filter(|binding| binding.digest() == self.route && binding.digest() == delta.binding())
            .and_then(|_| self.topology.source_for_slot(slot))
            .ok_or(PhysicalSignalDeltaApplicationFailure::BindingNotInstalled)?;
        self.context.version = self.context.version.saturating_add(1);
        self.runtime
            .transaction(&mut self.context, |transaction| {
                if delta.regions().is_empty() {
                    transaction.mark_changed(source, delta.signal_aspect())
                } else {
                    transaction.mark_changed_with_regions(
                        source,
                        delta.signal_aspect(),
                        delta.regions(),
                    )
                }
            })
            .map_err(|_| PhysicalSignalDeltaApplicationFailure::SignalMutationRejected)?;
        self.evaluate_dirty()
            .map_err(|_| PhysicalSignalDeltaApplicationFailure::SignalEvaluationRejected)
    }

    fn request(
        &mut self,
        admitted: AdmittedPhysicalWork,
    ) -> Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial> {
        let capability = self
            .topology
            .capability(admitted.signal_family())
            .ok_or(PhysicalWorkPreEffectDenial::CapabilityAbsent)?;
        let report = self
            .runtime
            .admit_async_node_request(capability.request_intent_requiring_clean_dependencies())
            .map_err(|_| PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?;
        let classification = report.classification();
        let Some(resource) = report.resource_admission() else {
            return Ok(PhysicalWorkReadiness::Blocked(BlockedPhysicalWork::new(
                admitted,
                classification.class(),
                classification.condition_block_class(),
            )));
        };
        let request = resource.admitted_request();
        Ok(PhysicalWorkReadiness::Ready(ReadyPhysicalWork::new(
            admitted,
            request.handle(),
            request.attempt(),
            capability.registry_digest().clone(),
            capability.bundle_digest().clone(),
            capability.payload_contract_digest().clone(),
        )))
    }

    fn revalidate(
        &mut self,
        admitted: AdmittedPhysicalWork,
        active: ResourceRequestHandle,
    ) -> Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial> {
        let capability = self
            .topology
            .capability(admitted.signal_family())
            .ok_or(PhysicalWorkPreEffectDenial::CapabilityAbsent)?;
        let report = self
            .runtime
            .revalidate_async_node(
                capability.revalidation_intent_requiring_clean_dependencies(active),
            )
            .map_err(|_| PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?;
        let classification = report.classification();
        let Some(resource) = report.resource_revalidation() else {
            return Ok(PhysicalWorkReadiness::Blocked(
                BlockedPhysicalWork::from_revalidation(
                    admitted,
                    classification.class(),
                    classification.condition_block_class(),
                    active,
                ),
            ));
        };
        let request = resource
            .admitted_revalidation()
            .ok_or(PhysicalWorkPreEffectDenial::DependencyBlocked)?
            .admitted_request();
        Ok(PhysicalWorkReadiness::Ready(ReadyPhysicalWork::new(
            admitted,
            request.handle(),
            request.attempt(),
            capability.registry_digest().clone(),
            capability.bundle_digest().clone(),
            capability.payload_contract_digest().clone(),
        )))
    }

    fn evaluate_dirty(&mut self) -> Result<(), SignalError> {
        let topology = &self.topology;
        let bindings = &self.bindings;
        let version = self.context.version;
        self.runtime.evaluate_dirty(&self.context, &|view| {
            if let Some(binding) = bindings.bindings().iter().enumerate().find_map(
                |(slot, binding)| {
                    (topology.source_for_slot(slot) == Some(view.node())).then_some(binding)
                },
            ) {
                return Ok(view.finish(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(binding.signal_aspect(), version)]),
                )));
            }
            if let Some(family) = topology.family_for_node(view.node()) {
                for (slot, binding) in bindings.bindings().iter().enumerate() {
                    if binding.digest() != self.route || !binding.serves_family(family) {
                        continue;
                    }
                    let source = topology
                        .source_for_slot(slot)
                        .expect("installed binding and source slots remain aligned");
                    if let Some(partition) = binding.partition() {
                        let _ = view.read_partitioned_aspect_version(
                            source,
                            binding.signal_aspect(),
                            partition.clone(),
                        )?;
                    } else {
                        let _ = view.read_aspect_version(source, binding.signal_aspect())?;
                    }
                }
            }
            Ok(view.finish(NodeEvaluationResult::from_version(AspectVersion::zero())))
        })?;
        Ok(())
    }
}
