use std::sync::Arc;

use crate::data::aspect::InstalledSignalGraphCapability;
use crate::data::graph::SignalGraph;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SignalAuthorizationPolicyIdentity([u8; 32]);

impl SignalAuthorizationPolicyIdentity {
    pub const fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub const fn bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignalAuthorizationPathEffect {
    Allow,
    Deny,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SignalAuthorizationPathContract {
    effect: SignalAuthorizationPathEffect,
}

impl SignalAuthorizationPathContract {
    pub const fn new(effect: SignalAuthorizationPathEffect) -> Self {
        Self { effect }
    }

    pub const fn effect(&self) -> SignalAuthorizationPathEffect {
        self.effect
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignalAuthorizationPolicyDefinition {
    identity: SignalAuthorizationPolicyIdentity,
    paths: Vec<SignalAuthorizationPathContract>,
}

impl SignalAuthorizationPolicyDefinition {
    pub fn new(
        identity: SignalAuthorizationPolicyIdentity,
        paths: impl IntoIterator<Item = SignalAuthorizationPathContract>,
    ) -> Self {
        Self {
            identity,
            paths: paths.into_iter().collect(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SignalAuthorizationDependencyCardinality {
    pub entities: usize,
    pub relations: usize,
    pub adjacency_lists: usize,
    pub fields: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignalAuthorizationPathObservation {
    effect: SignalAuthorizationPathEffect,
    matched: bool,
    exhaustive: bool,
    dependencies: SignalAuthorizationDependencyCardinality,
}

impl SignalAuthorizationPathObservation {
    pub const fn new(
        effect: SignalAuthorizationPathEffect,
        matched: bool,
        exhaustive: bool,
        dependencies: SignalAuthorizationDependencyCardinality,
    ) -> Self {
        Self {
            effect,
            matched,
            exhaustive,
            dependencies,
        }
    }
}

pub struct SignalAuthorizationObservation {
    dependency_identity: [u8; 32],
    paths: Vec<SignalAuthorizationPathObservation>,
}

impl SignalAuthorizationObservation {
    pub fn new(
        dependency_identity: [u8; 32],
        paths: impl IntoIterator<Item = SignalAuthorizationPathObservation>,
    ) -> Self {
        Self {
            dependency_identity,
            paths: paths.into_iter().collect(),
        }
    }
}

struct SignalAuthorizationAuthority {
    _seal: (),
}

pub struct InstalledSignalAuthorizationPolicy {
    graph_instance_id: u64,
    identity: SignalAuthorizationPolicyIdentity,
    paths: Vec<SignalAuthorizationPathContract>,
    authority: Arc<SignalAuthorizationAuthority>,
}

impl InstalledSignalAuthorizationPolicy {
    pub const fn graph_instance_id(&self) -> u64 {
        self.graph_instance_id
    }

    pub const fn identity(&self) -> SignalAuthorizationPolicyIdentity {
        self.identity
    }

    pub fn retains(&self, evidence: &SignalAuthorizationDecisionEvidence) -> bool {
        self.graph_instance_id == evidence.graph_instance_id
            && self.identity == evidence.policy_identity
            && Arc::ptr_eq(&self.authority, &evidence.authority)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignalAuthorizationDecision {
    Allowed,
    Denied,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SignalAuthorizationEvaluationCounters {
    pub paths_evaluated: usize,
    pub allow_paths_matched: usize,
    pub deny_paths_matched: usize,
    pub entities_depended_on: usize,
    pub relations_depended_on: usize,
    pub adjacency_lists_depended_on: usize,
    pub fields_depended_on: usize,
}

pub struct SignalAuthorizationDecisionEvidence {
    graph_instance_id: u64,
    policy_identity: SignalAuthorizationPolicyIdentity,
    dependency_identity: [u8; 32],
    decision: SignalAuthorizationDecision,
    counters: SignalAuthorizationEvaluationCounters,
    authority: Arc<SignalAuthorizationAuthority>,
}

impl SignalAuthorizationDecisionEvidence {
    pub const fn policy_identity(&self) -> SignalAuthorizationPolicyIdentity {
        self.policy_identity
    }

    pub const fn dependency_identity(&self) -> &[u8; 32] {
        &self.dependency_identity
    }

    pub const fn decision(&self) -> SignalAuthorizationDecision {
        self.decision
    }

    pub const fn counters(&self) -> SignalAuthorizationEvaluationCounters {
        self.counters
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SignalAuthorizationDenial {
    ForeignGraph,
    EmptyPolicy,
    MissingAllowPath,
    DuplicatePolicy,
    StalePolicy,
    ObservationShapeMismatch,
    NonExhaustiveObservation,
}

impl SignalGraph {
    pub fn install_authorization_policy(
        &mut self,
        graph: &InstalledSignalGraphCapability,
        definition: SignalAuthorizationPolicyDefinition,
    ) -> Result<InstalledSignalAuthorizationPolicy, SignalAuthorizationDenial> {
        if graph.graph_instance_id() != self.runtime_instance_id() {
            return Err(SignalAuthorizationDenial::ForeignGraph);
        }
        if definition.paths.is_empty() {
            return Err(SignalAuthorizationDenial::EmptyPolicy);
        }
        if !definition
            .paths
            .iter()
            .any(|path| path.effect == SignalAuthorizationPathEffect::Allow)
        {
            return Err(SignalAuthorizationDenial::MissingAllowPath);
        }
        if !self
            .authorization_policy_identities
            .insert(*definition.identity.bytes())
        {
            return Err(SignalAuthorizationDenial::DuplicatePolicy);
        }
        Ok(InstalledSignalAuthorizationPolicy {
            graph_instance_id: self.runtime_instance_id(),
            identity: definition.identity,
            paths: definition.paths,
            authority: Arc::new(SignalAuthorizationAuthority { _seal: () }),
        })
    }

    pub fn evaluate_authorization(
        &self,
        policy: &InstalledSignalAuthorizationPolicy,
        observation: SignalAuthorizationObservation,
    ) -> Result<SignalAuthorizationDecisionEvidence, SignalAuthorizationDenial> {
        if policy.graph_instance_id != self.runtime_instance_id()
            || !self
                .authorization_policy_identities
                .contains(policy.identity.bytes())
        {
            return Err(SignalAuthorizationDenial::StalePolicy);
        }
        if policy.paths.len() != observation.paths.len()
            || policy
                .paths
                .iter()
                .zip(&observation.paths)
                .any(|(contract, observed)| contract.effect != observed.effect)
        {
            return Err(SignalAuthorizationDenial::ObservationShapeMismatch);
        }
        if observation.paths.iter().any(|path| !path.exhaustive) {
            return Err(SignalAuthorizationDenial::NonExhaustiveObservation);
        }
        let mut counters = SignalAuthorizationEvaluationCounters::default();
        for path in &observation.paths {
            counters.paths_evaluated += 1;
            counters.entities_depended_on += path.dependencies.entities;
            counters.relations_depended_on += path.dependencies.relations;
            counters.adjacency_lists_depended_on += path.dependencies.adjacency_lists;
            counters.fields_depended_on += path.dependencies.fields;
            if path.matched {
                match path.effect {
                    SignalAuthorizationPathEffect::Allow => counters.allow_paths_matched += 1,
                    SignalAuthorizationPathEffect::Deny => counters.deny_paths_matched += 1,
                }
            }
        }
        let decision = if counters.allow_paths_matched > 0 && counters.deny_paths_matched == 0 {
            SignalAuthorizationDecision::Allowed
        } else {
            SignalAuthorizationDecision::Denied
        };
        Ok(SignalAuthorizationDecisionEvidence {
            graph_instance_id: self.runtime_instance_id(),
            policy_identity: policy.identity,
            dependency_identity: observation.dependency_identity,
            decision,
            counters,
            authority: Arc::clone(&policy.authority),
        })
    }
}

#[cfg(test)]
mod tests;
