use std::sync::Arc;

use worth_signal::facade::{
    AspectVersion, NodeEvaluationResult, ResourceRequestHandle, RuntimeClockBasis, SignalError,
    SignalGraph, SignalRuntime,
};

use crate::physical_runtime::work::{
    AdmittedPhysicalWork, BlockedPhysicalWork, InstalledPhysicalSignalTopology,
    PendingPhysicalSignalTopology, PhysicalSignalAspectBindingDigest,
    PhysicalSignalAspectBindingSet, PhysicalSignalReadinessEvidence, PhysicalWorkAspectDelta,
    PhysicalWorkPreEffectDenial, PhysicalWorkReadiness, ReadyPhysicalWork,
};

use super::{
    route::PhysicalSignalRouteCommand, PhysicalSignalAdmissionStatus,
    PhysicalSignalConstructionFailure, PhysicalSignalDeltaApplicationFailure,
};

struct PhysicalSignalContext {
    version: u64,
}

pub(super) struct PhysicalSignalGraph {
    bindings: Arc<PhysicalSignalAspectBindingSet>,
    runtime: SignalRuntime<(), (), (), PhysicalSignalContext, ()>,
    topology: InstalledPhysicalSignalTopology,
    context: PhysicalSignalContext,
}

impl PhysicalSignalGraph {
    pub(super) fn build(
        bindings: Arc<PhysicalSignalAspectBindingSet>,
    ) -> Result<Self, PhysicalSignalConstructionFailure> {
        let mut graph = SignalGraph::new();
        let pending = PendingPhysicalSignalTopology::build(&mut graph, &bindings)
            .map_err(|_| PhysicalSignalConstructionFailure::CapabilityDeclarationRejected)?;
        let mut runtime = SignalRuntime::build_for::<PhysicalSignalContext>(graph);
        let topology = pending
            .attach(&mut runtime)
            .map_err(|_| PhysicalSignalConstructionFailure::CapabilityDeclarationRejected)?;
        let mut owner = Self {
            bindings,
            runtime,
            topology,
            context: PhysicalSignalContext { version: 0 },
        };
        owner
            .evaluate_dirty()
            .map_err(|_| PhysicalSignalConstructionFailure::DependencyInitializationRejected)?;
        Ok(owner)
    }

    pub(super) fn clock_basis(&self) -> RuntimeClockBasis {
        self.runtime.clock_basis()
    }

    pub(super) fn apply_command(
        &mut self,
        route_slot: usize,
        command: PhysicalSignalRouteCommand,
        admission: &PhysicalSignalAdmissionStatus,
    ) -> bool {
        let Some(route) = self
            .bindings
            .binding_for_slot(route_slot)
            .map(|binding| binding.digest())
        else {
            return false;
        };
        match command {
            PhysicalSignalRouteCommand::Apply(delta, reply) => {
                let _ = reply.send(self.apply_delta(route, &delta));
            }
            PhysicalSignalRouteCommand::Request(admitted, reply) => {
                let _ = reply.send(self.request(route, admitted));
            }
            PhysicalSignalRouteCommand::Revalidate(admitted, active, reply) => {
                let _ = reply.send(self.revalidate(route, admitted, active));
            }
            #[cfg(feature = "certification-test-authority")]
            PhysicalSignalRouteCommand::FailForCertification(reply) => {
                admission.revoke();
                let _ = reply.send(());
                return false;
            }
        }
        true
    }

    fn apply_delta(
        &mut self,
        route: PhysicalSignalAspectBindingDigest,
        delta: &PhysicalWorkAspectDelta,
    ) -> Result<(), PhysicalSignalDeltaApplicationFailure> {
        let slot = delta.signal_aspect().index();
        let source = self
            .bindings
            .binding_for_slot(slot)
            .filter(|binding| binding.digest() == route && binding.digest() == delta.binding())
            .and_then(|_| self.topology.source_for_slot(slot))
            .ok_or(PhysicalSignalDeltaApplicationFailure::BindingNotInstalled)?;
        self.context.version = self
            .context
            .version
            .checked_add(1)
            .ok_or(PhysicalSignalDeltaApplicationFailure::VersionExhausted)?;
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
        route: PhysicalSignalAspectBindingDigest,
        admitted: AdmittedPhysicalWork,
    ) -> Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial> {
        let capability = self
            .topology
            .capability(route, admitted.signal_family())
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
            PhysicalSignalReadinessEvidence {
                signal_request: request.handle(),
                revalidated_from: None,
                attempt: request.attempt(),
                capability_registry: capability.registry_digest().clone(),
                capability_bundle: capability.bundle_digest().clone(),
                payload_contract: capability.payload_contract_digest().clone(),
            },
        )))
    }

    fn revalidate(
        &mut self,
        route: PhysicalSignalAspectBindingDigest,
        admitted: AdmittedPhysicalWork,
        active: ResourceRequestHandle,
    ) -> Result<PhysicalWorkReadiness, PhysicalWorkPreEffectDenial> {
        let capability = self
            .topology
            .capability(route, admitted.signal_family())
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
        let revalidation = resource
            .admitted_revalidation()
            .ok_or(PhysicalWorkPreEffectDenial::DependencyBlocked)?;
        let request = revalidation.admitted_request();
        let supersession = revalidation
            .supersession_record()
            .filter(|record| {
                record.previous() == active && record.replacing() == request.handle()
            })
            .ok_or(PhysicalWorkPreEffectDenial::SignalOwnerUnavailable)?;
        Ok(PhysicalWorkReadiness::Ready(ReadyPhysicalWork::new(
            admitted,
            PhysicalSignalReadinessEvidence {
                signal_request: request.handle(),
                revalidated_from: Some(supersession.previous()),
                attempt: request.attempt(),
                capability_registry: capability.registry_digest().clone(),
                capability_bundle: capability.bundle_digest().clone(),
                payload_contract: capability.payload_contract_digest().clone(),
            },
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
            if let (Some(route), Some(family)) = (
                topology.route_for_node(view.node()),
                topology.family_for_node(view.node()),
            ) {
                if let Some(binding) = bindings
                    .bindings()
                    .iter()
                    .find(|binding| binding.digest() == route && binding.serves_family(family))
                {
                    let slot = binding.signal_aspect().index();
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
