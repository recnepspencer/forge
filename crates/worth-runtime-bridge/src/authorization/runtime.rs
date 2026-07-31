use std::collections::BTreeMap;
use std::sync::Arc;

use worth_signal::facade::{
    InstalledSignalAuthorizationPolicy, SignalAuthorizationDependencyCardinality,
    SignalAuthorizationObservation, SignalAuthorizationPathContract, SignalAuthorizationPathEffect,
    SignalAuthorizationPathObservation, SignalAuthorizationPolicyDefinition,
    SignalAuthorizationPolicyIdentity, SignalGraph,
};

use super::evidence::BridgeAuthorizationCorrespondenceAuthority;
use super::{
    BridgeAuthorizationCorrespondenceIdentity, BridgeAuthorizationDecisionEvidence,
    BridgeAuthorizationDenial, BridgeAuthorizationDenialKind,
    BridgeAuthorizationInstallationRequest, BridgeAuthorizationObservation,
    BridgeAuthorizationPathContract, BridgeAuthorizationPathEffect,
};

struct BridgeInstalledAuthorizationCorrespondence {
    identity: BridgeAuthorizationCorrespondenceIdentity,
    binding_identity: worth_query_installation::facade::ApplicationSchemaBindingIdentity,
    ability: String,
    scope_entity: String,
    policy: String,
    paths: Vec<BridgeAuthorizationPathContract>,
    signal_policy: InstalledSignalAuthorizationPolicy,
    authority: Arc<BridgeAuthorizationCorrespondenceAuthority>,
}

pub struct BridgeAuthorizationRuntime {
    graph: SignalGraph,
    correspondences: BTreeMap<
        BridgeAuthorizationCorrespondenceIdentity,
        BridgeInstalledAuthorizationCorrespondence,
    >,
}

impl BridgeAuthorizationRuntime {
    pub fn new() -> Self {
        Self {
            graph: SignalGraph::new(),
            correspondences: BTreeMap::new(),
        }
    }

    pub fn install(
        &mut self,
        request: BridgeAuthorizationInstallationRequest,
    ) -> Result<BridgeAuthorizationCorrespondenceIdentity, BridgeAuthorizationDenial> {
        validate_request(&request)?;
        let identity = request.correspondence;
        if self.correspondences.contains_key(&identity) {
            return Err(denial(
                BridgeAuthorizationDenialKind::DuplicateCorrespondence,
                request.policy,
            ));
        }
        let signal_definition = SignalAuthorizationPolicyDefinition::new(
            SignalAuthorizationPolicyIdentity::new(*identity.bytes()),
            request
                .paths
                .iter()
                .map(|path| SignalAuthorizationPathContract::new(lower_effect(path.effect()))),
        );
        let graph_capability = self.graph.installed_graph_capability();
        let signal_policy = self
            .graph
            .install_authorization_policy(&graph_capability, signal_definition)
            .map_err(|_| {
                denial(
                    BridgeAuthorizationDenialKind::SignalInstallationRejected,
                    &request.policy,
                )
            })?;
        self.correspondences.insert(
            identity,
            BridgeInstalledAuthorizationCorrespondence {
                identity,
                binding_identity: request.binding_identity,
                ability: request.ability,
                scope_entity: request.scope_entity,
                policy: request.policy,
                paths: request.paths,
                signal_policy,
                authority: Arc::new(BridgeAuthorizationCorrespondenceAuthority { _seal: () }),
            },
        );
        Ok(identity)
    }

    pub fn matches_installed_policy(
        &self,
        correspondence: BridgeAuthorizationCorrespondenceIdentity,
        binding_identity: &worth_query_installation::facade::ApplicationSchemaBindingIdentity,
        ability: &str,
        scope_entity: &str,
        policy: &str,
        paths: &[BridgeAuthorizationPathContract],
    ) -> bool {
        self.correspondences
            .get(&correspondence)
            .is_some_and(|installed| {
                installed.identity == correspondence
                    && &installed.binding_identity == binding_identity
                    && installed.ability == ability
                    && installed.scope_entity == scope_entity
                    && installed.policy == policy
                    && installed.paths == paths
            })
    }

    pub fn evaluate(
        &self,
        observation: BridgeAuthorizationObservation,
    ) -> Result<BridgeAuthorizationDecisionEvidence, BridgeAuthorizationDenial> {
        let installed = self
            .correspondences
            .get(&observation.correspondence)
            .ok_or_else(|| {
                denial(
                    BridgeAuthorizationDenialKind::UnknownCorrespondence,
                    "unknown installed authorization correspondence",
                )
            })?;
        if installed.paths.len() != observation.paths.len()
            || installed
                .paths
                .iter()
                .zip(&observation.paths)
                .any(|(contract, observed)| {
                    contract.identity() != observed.identity()
                        || contract.effect() != observed.effect()
                })
        {
            return Err(denial(
                BridgeAuthorizationDenialKind::ObservationShapeMismatch,
                &installed.policy,
            ));
        }
        let signal_observation = SignalAuthorizationObservation::new(
            observation.dependency_identity,
            observation.paths.iter().map(|path| {
                SignalAuthorizationPathObservation::new(
                    lower_effect(path.effect()),
                    path.matched(),
                    path.exhaustive(),
                    SignalAuthorizationDependencyCardinality {
                        entities: path.entity_dependencies(),
                        relations: path.relation_dependencies(),
                        adjacency_lists: path.adjacency_dependencies(),
                        fields: path.field_dependencies(),
                    },
                )
            }),
        );
        let signal = self
            .graph
            .evaluate_authorization(&installed.signal_policy, signal_observation)
            .map_err(|_| {
                denial(
                    BridgeAuthorizationDenialKind::SignalEvaluationRejected,
                    &installed.policy,
                )
            })?;
        Ok(BridgeAuthorizationDecisionEvidence::mint(
            installed.identity,
            observation.dependency_identity,
            signal,
            Arc::clone(&installed.authority),
        ))
    }

    pub fn retains(&self, evidence: &BridgeAuthorizationDecisionEvidence) -> bool {
        self.correspondences
            .get(&evidence.correspondence())
            .is_some_and(|installed| {
                Arc::ptr_eq(&installed.authority, evidence.authority())
                    && installed.signal_policy.retains(evidence.signal())
                    && evidence.dependency_identity() == evidence.signal().dependency_identity()
            })
    }
}

impl Default for BridgeAuthorizationRuntime {
    fn default() -> Self {
        Self::new()
    }
}

fn validate_request(
    request: &BridgeAuthorizationInstallationRequest,
) -> Result<(), BridgeAuthorizationDenial> {
    if request.paths.is_empty() {
        return Err(denial(
            BridgeAuthorizationDenialKind::EmptyPolicy,
            &request.policy,
        ));
    }
    if !request
        .paths
        .iter()
        .any(|path| path.effect() == BridgeAuthorizationPathEffect::Allow)
    {
        return Err(denial(
            BridgeAuthorizationDenialKind::MissingAllowPath,
            &request.policy,
        ));
    }
    Ok(())
}

fn lower_effect(effect: BridgeAuthorizationPathEffect) -> SignalAuthorizationPathEffect {
    match effect {
        BridgeAuthorizationPathEffect::Allow => SignalAuthorizationPathEffect::Allow,
        BridgeAuthorizationPathEffect::Deny => SignalAuthorizationPathEffect::Deny,
    }
}

fn denial(
    kind: BridgeAuthorizationDenialKind,
    subject: impl Into<String>,
) -> BridgeAuthorizationDenial {
    BridgeAuthorizationDenial::new(kind, subject)
}
