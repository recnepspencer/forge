use std::{collections::BTreeMap, sync::Arc};

use worth_proof::TransitionOutcome;
use worth_signal::facade::SignalGraph;

use super::{
    BridgeConditionalDenial, BridgeConditionalDenialKind, BridgeConditionalProviderSet,
    BridgeInstalledConditionalLowering, BridgeInstalledConditionalLoweringCounters,
};
use crate::correspondence::{
    BridgeInstalledSemanticCorrespondence, BridgeSignalAspectTargetDeclaration,
};
use crate::facade::RuntimeBridge;

pub struct BridgeConditionalInstallationRequest {
    pub contract: super::BridgeConditionalContract,
    pub location: super::BridgeConditionalLocation,
    pub registrations: Vec<crate::correspondence::BridgeSemanticCorrespondenceRegistration>,
    pub providers: BridgeConditionalProviderSet,
}

struct AdmittedConditionalInstallationRequest {
    request: BridgeConditionalInstallationRequest,
    provider_admission: super::provider_admission::BridgeConditionalProviderAdmission,
    node: worth_signal::facade::NodeId,
    dependency_extension: crate::correspondence::AdmittedSemanticDependencyExtension,
    semantic_observation_plan:
        Option<super::semantic_observation_plan::BridgeConditionalSemanticObservationPlan>,
    counters: BridgeInstalledConditionalLoweringCounters,
}

/// Bridge-owned ordinary conditional runtime. It retains the exact Signal
/// graph; Query never receives a raw graph or a detached node capability.
pub struct BridgeOwnedSignalRuntime {
    pub(super) bridge: RuntimeBridge,
    baseline_semantic_dependency_registry:
        crate::correspondence::AdmittedSemanticDependencyRegistry,
    pub(super) graph: SignalGraph,
    pub(super) conditional_lowerings:
        BTreeMap<worth_signal::facade::NodeId, Arc<BridgeInstalledConditionalLowering>>,
    pub(super) conditional_observations: std::collections::BTreeMap<
        (worth_signal::facade::NodeId, usize),
        worth_foundational::facade::ContractValidatedAspectArtifact,
    >,
    pub(super) managed_clock_lanes: BTreeMap<Arc<str>, super::managed_time::BridgeManagedClockLane>,
}

impl BridgeOwnedSignalRuntime {
    /// Owns a fresh Signal graph behind the Bridge boundary.
    ///
    /// Callers that do not already own a topology-specific Signal graph use
    /// this constructor so raw Signal authority never crosses into them.
    pub fn with_owned_signal_graph(bridge: RuntimeBridge) -> Result<Self, BridgeConditionalDenial> {
        Self::new(bridge, SignalGraph::new())
    }

    pub fn new(
        mut bridge: RuntimeBridge,
        mut graph: SignalGraph,
    ) -> Result<Self, BridgeConditionalDenial> {
        let baseline_semantic_dependency_registry = bridge.semantic_dependency_registry.clone();
        crate::correspondence::isolate_allocation_state(&mut bridge).map_err(|_| {
            BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::CorrespondenceAdmission,
                "conditional runtime could not isolate its authoritative allocation state",
            )
        })?;
        graph
            .claim_aspect_lowering_owner(&bridge.signal_aspect_lowering_owner)
            .map_err(|_| {
                BridgeConditionalDenial::new(
                    BridgeConditionalDenialKind::ForeignSignalGraph,
                    "Signal graph already belongs to another lowering owner",
                )
            })?;
        Ok(Self {
            bridge,
            baseline_semantic_dependency_registry,
            graph,
            conditional_lowerings: BTreeMap::new(),
            conditional_observations: std::collections::BTreeMap::new(),
            managed_clock_lanes: BTreeMap::new(),
        })
    }

    pub fn install(
        &mut self,
        request: BridgeConditionalInstallationRequest,
    ) -> Result<Arc<BridgeInstalledConditionalLowering>, BridgeConditionalDenial> {
        let admitted = self.admit_conditional_installation_request(request)?;
        let mut counters = admitted.counters;
        counters.signal_node_admissions += 1;
        let TransitionOutcome::Success(node_capability) =
            self.graph.admit_installed_node(admitted.node)
        else {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::SignalContractInstallation,
                "correspondence target node is stale",
            )
            .with_lowering_counters(counters));
        };
        counters.correspondence_batch_preparations += 1;
        let prepared = crate::correspondence::prepare_registered_correspondence_batch(
            &self.bridge,
            admitted.dependency_extension.registrations(),
            &self.graph,
        )
        .map_err(|error| {
            BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::CorrespondenceAdmission,
                format!(
                    "conditional correspondence batch was denied without committing partial allocation: {error:?}"
                ),
            )
            .with_lowering_counters(counters)
        })?;
        let dependency_aspects = prepared.dependency_aspects();
        let condition_aspects =
            prepared.condition_aspects(admitted.request.contract.condition_dependency_ordinals());
        counters.signal_contract_lowerings += 1;
        let definition = super::lowering::lower_signal_contract(
            &admitted.request.contract,
            dependency_aspects,
            condition_aspects,
        )
        .map_err(|denial| denial.with_lowering_counters(counters))?;
        let signal_contract = self
            .graph
            .install_conditional_contract(
                &self.bridge.signal_aspect_lowering_owner,
                node_capability,
                definition,
            )
            .map_err(|_| {
                BridgeConditionalDenial::new(
                    BridgeConditionalDenialKind::SignalContractInstallation,
                    "Signal rejected the owner-bound conditional contract",
                )
                .with_lowering_counters(counters)
            })?;
        let correspondences = prepared.commit();
        self.commit_conditional_lowering(admitted, signal_contract, correspondences, counters)
    }

    fn admit_conditional_installation_request(
        &self,
        mut request: BridgeConditionalInstallationRequest,
    ) -> Result<AdmittedConditionalInstallationRequest, BridgeConditionalDenial> {
        let mut counters = BridgeInstalledConditionalLoweringCounters::default();
        counters.contract_admission_checks += 1;
        super::installation_admission::validate_declaration_pairing(&request)
            .map_err(|denial| denial.with_lowering_counters(counters))?;
        counters.provider_checks += super::provider_admission::PROVIDER_DIMENSION_CHECK_COUNT;
        let provider_admission =
            super::provider_admission::admit_provider_set(&request.contract, &request.providers)
                .map_err(|denial| denial.with_lowering_counters(counters))?;
        let node =
            self.admit_conditional_signal_target(&mut request.registrations, &mut counters)?;
        let dependency_extension =
            self.admit_conditional_dependency_extension(&request.registrations, &mut counters)?;
        counters.semantic_observation_plan_compilations += 1;
        let semantic_observation_plan =
            super::semantic_observation_plan::compile_semantic_observation_plan(
                &request.contract,
                &request.registrations,
            )
            .map_err(|denial| denial.with_lowering_counters(counters))?;
        Ok(AdmittedConditionalInstallationRequest {
            request,
            provider_admission,
            node,
            dependency_extension,
            semantic_observation_plan,
            counters,
        })
    }

    fn admit_conditional_signal_target(
        &self,
        registrations: &mut [crate::correspondence::BridgeSemanticCorrespondenceRegistration],
        counters: &mut BridgeInstalledConditionalLoweringCounters,
    ) -> Result<worth_signal::facade::NodeId, BridgeConditionalDenial> {
        if registrations.is_empty() {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::EmptyCorrespondenceSet,
                "conditional lowering requires one correspondence per declared dependency",
            )
            .with_lowering_counters(*counters));
        }
        counters.correspondence_registrations_inspected += registrations.len();
        registrations.sort_by_key(|registration| registration.dependency().dependency_ordinal());
        let (graph_instance_id, node) = declared_signal_node(registrations, counters)?;
        counters.signal_graph_checks += 1;
        if graph_instance_id != self.graph.installed_graph_capability().graph_instance_id() {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::ForeignSignalGraph,
                "conditional target registrations belong to another Signal graph",
            )
            .with_lowering_counters(*counters));
        }
        counters.signal_node_ownership_checks += 1;
        if self.conditional_lowerings.contains_key(&node) {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::SignalNodeAlreadyBound,
                "a Signal node cannot back multiple conditional declarations before explicit sharing admission",
            )
            .with_lowering_counters(*counters));
        }
        Ok(node)
    }

    fn admit_conditional_dependency_extension(
        &self,
        registrations: &[crate::correspondence::BridgeSemanticCorrespondenceRegistration],
        counters: &mut BridgeInstalledConditionalLoweringCounters,
    ) -> Result<crate::correspondence::AdmittedSemanticDependencyExtension, BridgeConditionalDenial>
    {
        counters.dependency_registry_compilations += 1;
        let extension = self
            .bridge
            .semantic_dependency_registry
            .admit_extension(registrations)
            .map_err(|denial| {
                counters.dependency_registry_existing_key_lookups =
                    denial.counters.existing_key_lookups;
                counters.dependency_registry_batch_key_lookups = denial.counters.batch_key_lookups;
                BridgeConditionalDenial::new(
                    BridgeConditionalDenialKind::CorrespondenceAdmission,
                    format!("{:?}", denial.error),
                )
                .with_lowering_counters(*counters)
            })?;
        counters.dependency_registry_existing_key_lookups =
            extension.counters().existing_key_lookups;
        counters.dependency_registry_batch_key_lookups = extension.counters().batch_key_lookups;
        Ok(extension)
    }

    fn commit_conditional_lowering(
        &mut self,
        admitted: AdmittedConditionalInstallationRequest,
        signal_contract: worth_signal::facade::InstalledSignalConditionalContract,
        correspondences: Vec<BridgeInstalledSemanticCorrespondence>,
        mut counters: BridgeInstalledConditionalLoweringCounters,
    ) -> Result<Arc<BridgeInstalledConditionalLowering>, BridgeConditionalDenial> {
        counters.dependency_registry_commits = admitted
            .dependency_extension
            .commit(&mut self.bridge.semantic_dependency_registry);
        counters.correspondence_admissions = correspondences.len();
        counters.signal_targets_joined = correspondences
            .iter()
            .map(BridgeInstalledSemanticCorrespondence::target_count)
            .sum();
        counters.signal_contract_installations = 1;
        let (authority, projection) =
            super::lowering_authority::mint_bridge_conditional_lowering_identity(
                super::lowering_identity::installed_lowering_identity(
                    self.bridge.signal_runtime_key,
                    &signal_contract,
                    admitted.request.contract.identity(),
                    &correspondences,
                ),
            );
        let lowering = Arc::new(BridgeInstalledConditionalLowering {
            bridge_runtime_key: self.bridge.signal_runtime_key,
            _authority: authority,
            projection,
            contract: admitted.request.contract,
            location: admitted.request.location,
            correspondences,
            semantic_observation_plan: admitted.semantic_observation_plan,
            signal_contract,
            providers: admitted.request.providers,
            provider_admission: admitted.provider_admission,
            lease: Arc::new(super::liveness::BridgeConditionalLoweringLease::issue()),
            counters,
        });
        self.conditional_lowerings
            .insert(admitted.node, Arc::clone(&lowering));
        Ok(lowering)
    }

    pub fn successor_installation_runtime(&self) -> Result<Self, BridgeConditionalDenial> {
        let mut bridge = self.bridge.clone();
        bridge.semantic_dependency_registry = self.baseline_semantic_dependency_registry.clone();
        Self::new(bridge, self.graph.clone())
    }

    pub fn baseline_semantic_dependency_count(&self) -> usize {
        self.baseline_semantic_dependency_registry
            .authoritative_count()
    }

    pub fn active_semantic_dependency_count(&self) -> usize {
        self.bridge
            .semantic_dependency_registry
            .authoritative_count()
    }

    pub fn revoke_conditional_liveness(&mut self) {
        for lowering in self.conditional_lowerings.values() {
            lowering.lease.revoke_liveness();
        }
    }

    pub fn rebind_signal_target(
        &self,
        target: &BridgeSignalAspectTargetDeclaration,
    ) -> Result<BridgeSignalAspectTargetDeclaration, BridgeConditionalDenial> {
        target.rebind_to_graph(&self.graph).ok_or_else(|| {
            BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::ForeignSignalGraph,
                "conditional target cannot be admitted into the successor Signal graph",
            )
        })
    }
}

impl Drop for BridgeOwnedSignalRuntime {
    fn drop(&mut self) {
        self.revoke_conditional_liveness();
        self.revoke_managed_clock_liveness();
    }
}

fn declared_signal_node(
    registrations: &[crate::correspondence::BridgeSemanticCorrespondenceRegistration],
    counters: &mut BridgeInstalledConditionalLoweringCounters,
) -> Result<(u64, worth_signal::facade::NodeId), BridgeConditionalDenial> {
    let mut targets = registrations
        .iter()
        .flat_map(|registration| registration.targets.iter());
    let first_target = targets.next().ok_or_else(|| {
        BridgeConditionalDenial::new(
            BridgeConditionalDenialKind::EmptyCorrespondenceSet,
            "conditional registrations retained no Signal target",
        )
        .with_lowering_counters(*counters)
    })?;
    counters.correspondence_targets_inspected += 1;
    let graph = first_target.graph_instance_id();
    let first = first_target.node;
    for target in targets {
        counters.correspondence_targets_inspected += 1;
        if target.graph_instance_id() != graph || target.node != first {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::MixedSignalNodes,
                "one conditional declaration cannot lower across multiple Signal nodes",
            )
            .with_lowering_counters(*counters));
        }
    }
    Ok((graph, first))
}
