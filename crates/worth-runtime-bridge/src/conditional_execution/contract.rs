use std::sync::Arc;

use worth_proof::TransitionOutcome;
use worth_query_installation::facade::{
    WorthQueryConditionalNodeLocation, WorthQueryPortableConditionalNodeDeclaration,
};
use worth_signal::facade::{InstalledSignalConditionalContract, SignalGraph};

use super::{BridgeConditionalDenial, BridgeConditionalDenialKind, BridgeConditionalProviderSet};
use crate::correspondence::BridgeInstalledSemanticCorrespondence;
use crate::facade::RuntimeBridge;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeInstalledConditionalLoweringIdentity(String);

impl BridgeInstalledConditionalLoweringIdentity {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BridgeInstalledConditionalLoweringCounters {
    pub correspondence_admissions: usize,
    pub signal_targets_joined: usize,
    pub provider_checks: usize,
    pub signal_contract_installations: usize,
}

pub struct BridgeConditionalInstallationRequest {
    pub declaration: WorthQueryPortableConditionalNodeDeclaration,
    pub location: WorthQueryConditionalNodeLocation,
    pub registrations: Vec<crate::correspondence::BridgeSemanticCorrespondenceRegistration>,
    pub providers: BridgeConditionalProviderSet,
}

pub struct BridgeInstalledConditionalLowering {
    pub(crate) identity: BridgeInstalledConditionalLoweringIdentity,
    pub(crate) declaration: WorthQueryPortableConditionalNodeDeclaration,
    pub(crate) location: WorthQueryConditionalNodeLocation,
    pub(crate) correspondences: Vec<BridgeInstalledSemanticCorrespondence>,
    pub(super) semantic_observation_plan:
        Option<super::semantic_observation_plan::BridgeConditionalSemanticObservationPlan>,
    pub(crate) signal_contract: InstalledSignalConditionalContract,
    pub(crate) providers: BridgeConditionalProviderSet,
    pub(crate) counters: BridgeInstalledConditionalLoweringCounters,
}

impl BridgeInstalledConditionalLowering {
    pub fn identity(&self) -> &BridgeInstalledConditionalLoweringIdentity {
        &self.identity
    }
    pub fn location(&self) -> &WorthQueryConditionalNodeLocation {
        &self.location
    }
    pub fn declaration(&self) -> &WorthQueryPortableConditionalNodeDeclaration {
        &self.declaration
    }
    pub fn signal_graph_instance_id(&self) -> u64 {
        self.signal_contract.graph_instance_id()
    }
    pub fn signal_node(&self) -> worth_signal::facade::NodeId {
        self.signal_contract.node()
    }
    pub fn counters(&self) -> BridgeInstalledConditionalLoweringCounters {
        self.counters
    }
    pub fn correspondence_count(&self) -> usize {
        self.correspondences.len()
    }

    pub fn validate_query_authority_continuity(
        &self,
        operation_identity: &str,
        runtime_authority: u64,
        installation_generation: u64,
        graph_authorities: &[(
            &str,
            &Arc<worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority>,
        )],
    ) -> Result<(), BridgeConditionalDenial> {
        if self.correspondences.iter().any(|correspondence| {
            let basis = correspondence.basis();
            basis.query_basis.as_ref() != operation_identity
                || basis.query_runtime_authority() != runtime_authority
                || basis.query_installation_generation() != installation_generation
        }) {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::OperationAuthorityMismatch,
                "conditional lowering no longer joins the bound operation authority basis",
            ));
        }
        if self.correspondences.iter().any(|correspondence| {
            let basis = correspondence.basis();
            !graph_authorities.iter().any(|(role, authority)| {
                *role == basis.declared_graph_role()
                    && Arc::ptr_eq(authority, &basis.graph_authority)
            })
        }) {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::GraphAuthorityMismatch,
                "conditional lowering graph participation is absent from the bound operation",
            ));
        }
        Ok(())
    }

    pub fn validate_signal_decision_contract(
        &self,
        evidence: &worth_signal::facade::SignalConditionalDecisionEvidence,
    ) -> Result<(), BridgeConditionalDenial> {
        if evidence.condition() != self.signal_contract.condition()
            || evidence.semantic_condition() != self.signal_contract.semantic_condition()
            || evidence.dependency_aspects() != self.signal_contract.dependency_aspects()
            || evidence.trigger_aspects() != self.signal_contract.trigger_aspects()
            || evidence.dependency_comparator() != self.signal_contract.dependency_comparator()
            || evidence.output_comparator() != self.signal_contract.output_comparator()
            || evidence.artifact_reuse() != self.signal_contract.artifact_reuse()
        {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::SignalContractMismatch,
                "Signal decision evidence does not retain the installed conditional contract",
            ));
        }
        Ok(())
    }
}

/// Bridge-owned ordinary conditional runtime. It retains the exact Signal
/// graph; Query never receives a raw graph or a detached node capability.
pub struct BridgeOwnedSignalRuntime {
    pub(super) bridge: RuntimeBridge,
    pub(super) graph: SignalGraph,
    conditional_nodes: Vec<worth_signal::facade::NodeId>,
    pub(super) conditional_observations: std::collections::BTreeMap<
        (worth_signal::facade::NodeId, usize),
        worth_foundational::facade::ContractValidatedAspectArtifact,
    >,
}

impl BridgeOwnedSignalRuntime {
    pub fn new(
        mut bridge: RuntimeBridge,
        mut graph: SignalGraph,
    ) -> Result<Self, BridgeConditionalDenial> {
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
            graph,
            conditional_nodes: Vec::new(),
            conditional_observations: std::collections::BTreeMap::new(),
        })
    }

    pub fn install(
        &mut self,
        mut request: BridgeConditionalInstallationRequest,
    ) -> Result<Arc<BridgeInstalledConditionalLowering>, BridgeConditionalDenial> {
        super::installation_admission::validate_declaration_pairing(&request)?;
        super::installation_admission::validate_supported_postures(&request.declaration)?;
        super::provider_admission::validate_provider_shape(
            &request.declaration,
            &request.providers,
        )?;
        if request.registrations.is_empty() {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::EmptyCorrespondenceSet,
                "conditional lowering requires one correspondence per declared dependency",
            ));
        }
        request
            .registrations
            .sort_by_key(|registration| registration.dependency().dependency_ordinal());
        let (graph_instance_id, node) = declared_signal_node(&request.registrations)?;
        if graph_instance_id != self.graph.installed_graph_capability().graph_instance_id() {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::ForeignSignalGraph,
                "conditional target registrations belong to another Signal graph",
            ));
        }
        if self.conditional_nodes.contains(&node) {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::SignalNodeAlreadyBound,
                "a Signal node cannot back multiple conditional declarations before explicit sharing admission",
            ));
        }
        let dependency_registry = self.extended_dependency_registry(&request.registrations)?;
        let semantic_observation_plan =
            super::semantic_observation_plan::compile_semantic_observation_plan(
                &request.declaration,
                &request.registrations,
            )?;
        let mut staged_bridge = self.bridge.clone();
        staged_bridge.query_dependency_registry = dependency_registry.clone();
        let mut counters = BridgeInstalledConditionalLoweringCounters::default();
        let TransitionOutcome::Success(node_capability) = self.graph.admit_installed_node(node)
        else {
            return Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::SignalContractInstallation,
                "correspondence target node is stale",
            ));
        };
        let prepared = crate::correspondence::prepare_correspondence_batch(
            &staged_bridge,
            request
                .registrations
                .iter()
                .map(|registration| registration.dependency().clone())
                .collect(),
            &self.graph,
        )
        .map_err(|error| {
            BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::CorrespondenceAdmission,
                format!(
                    "conditional correspondence batch was denied without committing partial allocation: {error:?}"
                ),
            )
        })?;
        let dependency_aspects = prepared.dependency_aspects();
        let condition_aspects =
            prepared.condition_aspects(request.declaration.condition().dependencies());
        let definition = super::lowering::lower_signal_contract(
            &request.declaration,
            dependency_aspects,
            condition_aspects,
        )?;
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
            })?;
        let correspondences = prepared.commit();
        self.bridge.query_dependency_registry = dependency_registry;
        counters.correspondence_admissions = correspondences.len();
        counters.signal_targets_joined = correspondences
            .iter()
            .map(BridgeInstalledSemanticCorrespondence::target_count)
            .sum();
        counters.signal_contract_installations = 1;
        self.conditional_nodes.push(node);
        counters.provider_checks = super::provider_admission::PROVIDER_DIMENSION_CHECK_COUNT;
        let identity = BridgeInstalledConditionalLoweringIdentity(
            super::lowering_identity::installed_lowering_identity(
                self.bridge.signal_runtime_key,
                &signal_contract,
                request.declaration.identity(),
                &correspondences,
            ),
        );
        Ok(Arc::new(BridgeInstalledConditionalLowering {
            identity,
            declaration: request.declaration,
            location: request.location,
            correspondences,
            semantic_observation_plan,
            signal_contract,
            providers: request.providers,
            counters,
        }))
    }

    fn extended_dependency_registry(
        &self,
        registrations: &[crate::correspondence::BridgeSemanticCorrespondenceRegistration],
    ) -> Result<crate::correspondence::AdmittedQueryDependencyRegistry, BridgeConditionalDenial>
    {
        let mut authoritative = self
            .bridge
            .query_dependency_registry
            .authoritative_registrations()
            .to_vec();
        authoritative.extend_from_slice(registrations);
        crate::correspondence::AdmittedQueryDependencyRegistry::freeze(authoritative).map_err(
            |error| {
                BridgeConditionalDenial::new(
                    BridgeConditionalDenialKind::CorrespondenceAdmission,
                    format!("{error:?}"),
                )
            },
        )
    }
}

fn declared_signal_node(
    registrations: &[crate::correspondence::BridgeSemanticCorrespondenceRegistration],
) -> Result<(u64, worth_signal::facade::NodeId), BridgeConditionalDenial> {
    let mut targets = registrations
        .iter()
        .flat_map(|registration| registration.targets.iter());
    let first_target = targets.next().ok_or_else(|| {
        BridgeConditionalDenial::new(
            BridgeConditionalDenialKind::EmptyCorrespondenceSet,
            "conditional registrations retained no Signal target",
        )
    })?;
    let graph = first_target.graph_instance_id();
    let first = first_target.node;
    if targets.any(|target| target.graph_instance_id() != graph || target.node != first) {
        return Err(BridgeConditionalDenial::new(
            BridgeConditionalDenialKind::MixedSignalNodes,
            "one conditional declaration cannot lower across multiple Signal nodes",
        ));
    }
    Ok((graph, first))
}
