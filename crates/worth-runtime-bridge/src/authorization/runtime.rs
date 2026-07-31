use std::collections::BTreeMap;
use std::sync::Arc;

use worth_signal::facade::{
    InstalledSignalAuthorizationPolicy, SignalAuthorizationClauseContract,
    SignalAuthorizationClauseObservation, SignalAuthorizationDependencyCardinality,
    SignalAuthorizationObservation, SignalAuthorizationPolicyDefinition,
    SignalAuthorizationPolicyIdentity, SignalAuthorizationRequirementContract,
    SignalAuthorizationRequirementObservation, SignalAuthorizationRuleContract,
    SignalAuthorizationRuleEffect, SignalAuthorizationRuleObservation, SignalGraph,
};

use super::evidence::BridgeAuthorizationCorrespondenceAuthority;
use super::{
    BridgeAuthorizationCorrespondenceIdentity, BridgeAuthorizationDecisionEvidence,
    BridgeAuthorizationDenial, BridgeAuthorizationDenialKind,
    BridgeAuthorizationInstallationRequest, BridgeAuthorizationObservation,
    BridgeAuthorizationRuleContract, BridgeAuthorizationRuleEffect,
};

struct BridgeInstalledAuthorizationCorrespondence {
    identity: BridgeAuthorizationCorrespondenceIdentity,
    binding_identity: worth_query_installation::facade::ApplicationSchemaBindingIdentity,
    ability: String,
    scope_entity: String,
    policy: String,
    rules: Vec<BridgeAuthorizationRuleContract>,
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
            request.rules.iter().map(lower_rule_contract),
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
                rules: request.rules,
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
        rules: &[BridgeAuthorizationRuleContract],
    ) -> bool {
        self.correspondences
            .get(&correspondence)
            .is_some_and(|installed| {
                installed.identity == correspondence
                    && &installed.binding_identity == binding_identity
                    && installed.ability == ability
                    && installed.scope_entity == scope_entity
                    && installed.policy == policy
                    && installed.rules == rules
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
        if !same_shape(&installed.rules, &observation.rules) {
            return Err(denial(
                BridgeAuthorizationDenialKind::ObservationShapeMismatch,
                &installed.policy,
            ));
        }
        let signal_observation = SignalAuthorizationObservation::new(
            observation.dependency_identity,
            observation.rules.iter().map(|rule| {
                SignalAuthorizationRuleObservation::all(
                    lower_effect(rule.effect),
                    rule.requirements.iter().map(|requirement| {
                        SignalAuthorizationRequirementObservation::any(
                            requirement.clauses.iter().map(|clause| {
                                let dependencies = clause.dependencies();
                                SignalAuthorizationClauseObservation::new(
                                    clause.matched(),
                                    clause.exhaustive(),
                                    SignalAuthorizationDependencyCardinality {
                                        entities: dependencies.entities,
                                        relations: dependencies.relations,
                                        adjacency_lists: dependencies.adjacency_lists,
                                        fields: dependencies.fields,
                                    },
                                )
                            }),
                        )
                    }),
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
    if request.rules.is_empty() {
        return Err(denial(
            BridgeAuthorizationDenialKind::EmptyPolicy,
            &request.policy,
        ));
    }
    if !request
        .rules
        .iter()
        .any(|rule| rule.effect() == BridgeAuthorizationRuleEffect::Required)
    {
        return Err(denial(
            BridgeAuthorizationDenialKind::MissingRequiredRule,
            &request.policy,
        ));
    }
    if request
        .rules
        .iter()
        .any(|rule| rule.requirements().is_empty())
    {
        return Err(denial(
            BridgeAuthorizationDenialKind::EmptyRule,
            &request.policy,
        ));
    }
    if request
        .rules
        .iter()
        .flat_map(BridgeAuthorizationRuleContract::requirements)
        .any(|requirement| requirement.clauses().is_empty())
    {
        return Err(denial(
            BridgeAuthorizationDenialKind::EmptyRequirement,
            &request.policy,
        ));
    }
    Ok(())
}

fn same_shape(
    contracts: &[BridgeAuthorizationRuleContract],
    observations: &[super::BridgeAuthorizationRuleObservation],
) -> bool {
    contracts.len() == observations.len()
        && contracts
            .iter()
            .zip(observations)
            .all(|(contract, observed)| {
                contract.effect() == observed.effect
                    && contract.requirements().len() == observed.requirements.len()
                    && contract
                        .requirements()
                        .iter()
                        .zip(&observed.requirements)
                        .all(|(required, actual)| {
                            required.clauses().len() == actual.clauses.len()
                                && required.clauses().iter().zip(&actual.clauses).all(
                                    |(expected, clause)| expected.identity() == clause.identity(),
                                )
                        })
            })
}

fn lower_rule_contract(rule: &BridgeAuthorizationRuleContract) -> SignalAuthorizationRuleContract {
    SignalAuthorizationRuleContract::all(
        lower_effect(rule.effect()),
        rule.requirements().iter().map(|requirement| {
            SignalAuthorizationRequirementContract::any(
                requirement
                    .clauses()
                    .iter()
                    .map(|_| SignalAuthorizationClauseContract::new()),
            )
        }),
    )
}

const fn lower_effect(effect: BridgeAuthorizationRuleEffect) -> SignalAuthorizationRuleEffect {
    match effect {
        BridgeAuthorizationRuleEffect::Required => SignalAuthorizationRuleEffect::Required,
        BridgeAuthorizationRuleEffect::Prohibited => SignalAuthorizationRuleEffect::Prohibited,
    }
}

fn denial(
    kind: BridgeAuthorizationDenialKind,
    subject: impl Into<String>,
) -> BridgeAuthorizationDenial {
    BridgeAuthorizationDenial::new(kind, subject)
}
