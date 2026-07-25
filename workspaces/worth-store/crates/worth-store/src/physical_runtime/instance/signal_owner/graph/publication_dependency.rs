use worth_signal::facade::adapters::NodeContract;
use worth_signal::facade::core::AsyncCapableNode;
#[cfg(feature = "certification-test-authority")]
use worth_signal::facade::NodeState;
use worth_signal::facade::{
    AspectMask, AsyncNodeAdmissionClass, AsyncNodeCapabilityDeclaration,
    AsyncNodeConditionBlockClass, AsyncNodePayloadContract, AsyncNodePayloadContractId,
    DependencyEdge, NodeId,
};

use crate::physical_runtime::work::{
    PhysicalSignalPolicySelection, PhysicalSignalReadinessEvidence, PhysicalWorkSignalDeclaration,
    PhysicalWorkSignalFamily,
};
use crate::physical_runtime::{
    AdmittedPhysicalWork, BlockedPhysicalWork, PhysicalSignalAspectBindingDigest,
    PhysicalWorkIdentity, PhysicalWorkPreEffectDenial, ReadyPhysicalWork,
};

use super::PhysicalSignalGraph;

pub(super) struct PhysicalPublicationDependencyLocality {
    source: NodeId,
    capability: AsyncCapableNode,
    #[cfg(feature = "certification-test-authority")]
    blocked: PhysicalPublicationDependencyObservation,
}

#[cfg(feature = "certification-test-authority")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalPublicationDependencyObservation {
    identity: PhysicalWorkIdentity,
    class: AsyncNodeAdmissionClass,
    condition: Option<AsyncNodeConditionBlockClass>,
    node_state: NodeState,
    upstream_dependencies: u32,
}

#[cfg(feature = "certification-test-authority")]
impl PhysicalPublicationDependencyObservation {
    pub const fn identity(self) -> PhysicalWorkIdentity {
        self.identity
    }

    pub const fn class(self) -> AsyncNodeAdmissionClass {
        self.class
    }

    pub const fn condition(self) -> Option<AsyncNodeConditionBlockClass> {
        self.condition
    }

    pub const fn node_state(self) -> NodeState {
        self.node_state
    }

    pub const fn upstream_dependencies(self) -> u32 {
        self.upstream_dependencies
    }
}

impl PhysicalSignalGraph {
    pub(super) fn begin_publication_dependency(
        &mut self,
        route: PhysicalSignalAspectBindingDigest,
        admitted: AdmittedPhysicalWork,
    ) -> Result<BlockedPhysicalWork, PhysicalWorkPreEffectDenial> {
        let identity = admitted.intent().identity();
        if admitted.signal_family() != PhysicalWorkSignalFamily::Publication
            || admitted.authority().binding() != route
            || !self.locality.register(&admitted)
        {
            return Err(PhysicalWorkPreEffectDenial::CapabilityAbsent);
        }
        if !admitted.register_signal_locality(route) {
            self.release_identity(identity);
            return Err(PhysicalWorkPreEffectDenial::SignalOwnerUnavailable);
        }
        let dependency = match self.create_publication_dependency(route, identity) {
            Ok(dependency) => dependency,
            Err(failure) => {
                self.release_identity(identity);
                return Err(failure);
            }
        };
        let capability = dependency.capability.clone();
        let report = match self
            .runtime
            .admit_async_node_request(capability.request_intent_requiring_clean_dependencies())
        {
            Ok(report) => report,
            Err(_) => {
                self.retire_publication_dependency(dependency);
                self.release_identity(identity);
                return Err(PhysicalWorkPreEffectDenial::SignalOwnerUnavailable);
            }
        };
        let classification = report.classification();
        let blocked = classification.class() == AsyncNodeAdmissionClass::BlockedByCondition
            && classification.condition_block_class()
                == Some(AsyncNodeConditionBlockClass::DependencyNotReady)
            && report.resource_admission().is_none();
        if !blocked {
            if let Some(resource) = report.resource_admission() {
                self.cancel_unbound_request(identity, resource.admitted_request().handle());
            } else {
                self.release_identity(identity);
            }
            self.retire_publication_dependency(dependency);
            return Err(PhysicalWorkPreEffectDenial::SignalOwnerUnavailable);
        }
        let dependency_count = {
            let graph = self.runtime.graph_mut();
            graph
                .dependencies_of(capability.node())
                .map(|dependencies| dependencies.len() as u32)
        };
        let _upstream_dependencies = match dependency_count {
            Ok(count) => count,
            Err(_) => {
                self.retire_publication_dependency(dependency);
                self.release_identity(identity);
                return Err(PhysicalWorkPreEffectDenial::SignalOwnerUnavailable);
            }
        };
        #[cfg(feature = "certification-test-authority")]
        let dependency = dependency.with_observation(PhysicalPublicationDependencyObservation {
            identity,
            class: classification.class(),
            condition: classification.condition_block_class(),
            node_state: classification.node_state(),
            upstream_dependencies: _upstream_dependencies,
        });
        if let Err(dependency) = self
            .locality
            .attach_publication_dependency(identity, dependency)
        {
            self.retire_publication_dependency(dependency);
            self.release_identity(identity);
            return Err(PhysicalWorkPreEffectDenial::SignalOwnerUnavailable);
        }
        Ok(BlockedPhysicalWork::new(
            admitted,
            classification.class(),
            classification.condition_block_class(),
        ))
    }

    pub(super) fn advance_publication_dependency(
        &mut self,
        route: PhysicalSignalAspectBindingDigest,
        blocked: BlockedPhysicalWork,
    ) -> Result<ReadyPhysicalWork, PhysicalWorkPreEffectDenial> {
        let identity = blocked.intent().identity();
        if blocked.authority().binding() != route {
            self.release_identity(identity);
            return Err(PhysicalWorkPreEffectDenial::CapabilityAbsent);
        }
        let Some(capability) = self
            .locality
            .publication_dependency(identity)
            .map(|dependency| dependency.capability.clone())
        else {
            self.release_identity(identity);
            return Err(PhysicalWorkPreEffectDenial::CapabilityAbsent);
        };
        if self.evaluate_target(capability.node()).is_err() {
            self.release_identity(identity);
            return Err(PhysicalWorkPreEffectDenial::SignalOwnerUnavailable);
        }
        let admitted = blocked.into_admitted();
        let report = match self
            .runtime
            .admit_async_node_request(capability.request_intent_requiring_clean_dependencies())
        {
            Ok(report) => report,
            Err(_) => {
                self.release_identity(identity);
                return Err(PhysicalWorkPreEffectDenial::SignalOwnerUnavailable);
            }
        };
        let classification = report.classification();
        if classification.class() != AsyncNodeAdmissionClass::AdmittedNewLineage
            || classification.condition_block_class().is_some()
        {
            if let Some(resource) = report.resource_admission() {
                self.cancel_unbound_request(identity, resource.admitted_request().handle());
            } else {
                self.release_identity(identity);
            }
            return Err(PhysicalWorkPreEffectDenial::DependencyBlocked);
        }
        let Some(resource) = report.resource_admission() else {
            self.release_identity(identity);
            return Err(PhysicalWorkPreEffectDenial::DependencyBlocked);
        };
        let request = resource.admitted_request();
        if !self.locality.bind(identity, request.handle()) {
            self.cancel_unbound_request(identity, request.handle());
            return Err(PhysicalWorkPreEffectDenial::SignalOwnerUnavailable);
        }
        Ok(ReadyPhysicalWork::new(
            admitted,
            PhysicalSignalReadinessEvidence {
                signal_request: request.handle(),
                supersession: None,
                replaces: None,
                attempt: request.attempt(),
                capability_registry: capability.registry_digest().clone(),
                capability_bundle: capability.bundle_digest().clone(),
                payload_contract: capability.payload_contract_digest().clone(),
            },
        ))
    }

    #[cfg(feature = "certification-test-authority")]
    pub(super) fn publication_dependency_observations(
        &self,
    ) -> Vec<PhysicalPublicationDependencyObservation> {
        self.locality.publication_dependency_observations()
    }

    pub(super) fn retire_publication_dependency(
        &mut self,
        dependency: PhysicalPublicationDependencyLocality,
    ) {
        self.retire_publication_nodes(dependency.source, dependency.capability.node());
    }

    fn retire_publication_nodes(&mut self, source: NodeId, gate: NodeId) {
        let gate_retired = self.runtime.graph_mut().unregister_node(gate).is_ok();
        let source_retired = self.runtime.graph_mut().unregister_node(source).is_ok();
        self.healthy &= gate_retired && source_retired;
    }

    fn create_publication_dependency(
        &mut self,
        route: PhysicalSignalAspectBindingDigest,
        _identity: PhysicalWorkIdentity,
    ) -> Result<PhysicalPublicationDependencyLocality, PhysicalWorkPreEffectDenial> {
        let binding = self
            .bindings
            .bindings()
            .iter()
            .find(|binding| {
                binding.digest() == route
                    && binding.serves_family(PhysicalWorkSignalFamily::Publication)
            })
            .ok_or(PhysicalWorkPreEffectDenial::CapabilityAbsent)?;
        let subscription = binding
            .projection_subscription()
            .map_err(|_| PhysicalWorkPreEffectDenial::CapabilityAbsent)?;
        let (source, gate, dependency_result) = {
            let mut graph = self.runtime.graph_mut();
            let source = graph
                .node()
                .with_contract(
                    NodeContract::reads(AspectMask::EMPTY).with_produces(binding.signal_aspect()),
                )
                .build();
            let gate = graph
                .node()
                .with_contract(
                    NodeContract::reads(subscription.signal_mask())
                        .with_produces(AspectMask::EMPTY),
                )
                .on_demand()
                .build();
            let result = graph.set_dependencies(
                gate,
                [match subscription.partition() {
                    Some(partition) => DependencyEdge::with_partition_scope(
                        source,
                        binding.signal_aspect(),
                        partition.clone(),
                    ),
                    None => DependencyEdge::new(source, binding.signal_aspect()),
                }],
            );
            (source, gate, result)
        };
        if dependency_result.is_err() {
            self.retire_publication_nodes(source, gate);
            return Err(PhysicalWorkPreEffectDenial::SignalOwnerUnavailable);
        }
        let declaration =
            PhysicalWorkSignalDeclaration::for_family(PhysicalWorkSignalFamily::Publication)
                .expect("the canonical publication capability is installed");
        let payload = AsyncNodePayloadContract::new(AsyncNodePayloadContractId::new(
            declaration.payload_contract_id(),
        ))
        .with_max_payload_bytes(declaration.max_payload_bytes());
        let capability =
            match self
                .runtime
                .attach_async_capability(PhysicalSignalPolicySelection::apply(
                    AsyncNodeCapabilityDeclaration::new(gate, payload),
                )) {
                Ok(capability) => capability,
                Err(_) => {
                    self.retire_publication_nodes(source, gate);
                    return Err(PhysicalWorkPreEffectDenial::SignalOwnerUnavailable);
                }
            };
        Ok(PhysicalPublicationDependencyLocality {
            source,
            capability,
            #[cfg(feature = "certification-test-authority")]
            blocked: PhysicalPublicationDependencyObservation {
                identity: _identity,
                class: AsyncNodeAdmissionClass::BlockedByCondition,
                condition: Some(AsyncNodeConditionBlockClass::DependencyNotReady),
                node_state: NodeState::Dirty,
                upstream_dependencies: 1,
            },
        })
    }
}

impl PhysicalPublicationDependencyLocality {
    #[cfg(feature = "certification-test-authority")]
    fn with_observation(mut self, observation: PhysicalPublicationDependencyObservation) -> Self {
        self.blocked = observation;
        self
    }

    #[cfg(feature = "certification-test-authority")]
    pub(super) const fn observation(&self) -> PhysicalPublicationDependencyObservation {
        self.blocked
    }
}
