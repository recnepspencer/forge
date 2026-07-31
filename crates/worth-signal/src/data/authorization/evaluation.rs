use std::sync::Arc;

use crate::data::aspect::InstalledSignalGraphCapability;
use crate::data::graph::SignalGraph;

use super::{
    InstalledSignalAuthorizationPolicy, SignalAuthorizationAuthority, SignalAuthorizationDecision,
    SignalAuthorizationDecisionEvidence, SignalAuthorizationDenial,
    SignalAuthorizationEvaluationCounters, SignalAuthorizationObservation,
    SignalAuthorizationPolicyDefinition, SignalAuthorizationRuleContract,
    SignalAuthorizationRuleEffect, SignalAuthorizationRuleObservation,
};

impl SignalGraph {
    pub fn install_authorization_policy(
        &mut self,
        graph: &InstalledSignalGraphCapability,
        definition: SignalAuthorizationPolicyDefinition,
    ) -> Result<InstalledSignalAuthorizationPolicy, SignalAuthorizationDenial> {
        validate_definition(self, graph, &definition)?;
        self.authorization_policy_identities
            .insert(*definition.identity.bytes());
        Ok(InstalledSignalAuthorizationPolicy {
            graph_instance_id: self.runtime_instance_id(),
            identity: definition.identity,
            rules: definition.rules,
            authority: Arc::new(SignalAuthorizationAuthority { _seal: () }),
        })
    }

    pub fn evaluate_authorization(
        &self,
        policy: &InstalledSignalAuthorizationPolicy,
        observation: SignalAuthorizationObservation,
    ) -> Result<SignalAuthorizationDecisionEvidence, SignalAuthorizationDenial> {
        validate_observation(self, policy, &observation)?;
        let mut counters = SignalAuthorizationEvaluationCounters::default();
        let mut required_rules_match = true;
        let mut prohibited_rule_matches = false;
        for rule in &observation.rules {
            let rule_matches = evaluate_rule(rule, &mut counters);
            match rule.effect {
                SignalAuthorizationRuleEffect::Required => {
                    required_rules_match &= rule_matches;
                    counters.required_rules_matched += usize::from(rule_matches);
                }
                SignalAuthorizationRuleEffect::Prohibited => {
                    prohibited_rule_matches |= rule_matches;
                    counters.prohibited_rules_matched += usize::from(rule_matches);
                }
            }
        }
        let decision = if required_rules_match && !prohibited_rule_matches {
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

fn validate_definition(
    graph: &SignalGraph,
    capability: &InstalledSignalGraphCapability,
    definition: &SignalAuthorizationPolicyDefinition,
) -> Result<(), SignalAuthorizationDenial> {
    if capability.graph_instance_id() != graph.runtime_instance_id() {
        return Err(SignalAuthorizationDenial::ForeignGraph);
    }
    if definition.rules.is_empty() {
        return Err(SignalAuthorizationDenial::EmptyPolicy);
    }
    if !definition
        .rules
        .iter()
        .any(|rule| rule.effect == SignalAuthorizationRuleEffect::Required)
    {
        return Err(SignalAuthorizationDenial::MissingRequiredRule);
    }
    for rule in &definition.rules {
        if rule.requirements.is_empty() {
            return Err(SignalAuthorizationDenial::EmptyRule);
        }
        if rule
            .requirements
            .iter()
            .any(|requirement| requirement.clauses.is_empty())
        {
            return Err(SignalAuthorizationDenial::EmptyRequirement);
        }
    }
    if graph
        .authorization_policy_identities
        .contains(definition.identity.bytes())
    {
        return Err(SignalAuthorizationDenial::DuplicatePolicy);
    }
    Ok(())
}

fn validate_observation(
    graph: &SignalGraph,
    policy: &InstalledSignalAuthorizationPolicy,
    observation: &SignalAuthorizationObservation,
) -> Result<(), SignalAuthorizationDenial> {
    if policy.graph_instance_id != graph.runtime_instance_id()
        || !graph
            .authorization_policy_identities
            .contains(policy.identity.bytes())
    {
        return Err(SignalAuthorizationDenial::StalePolicy);
    }
    if !same_shape(&policy.rules, &observation.rules) {
        return Err(SignalAuthorizationDenial::ObservationShapeMismatch);
    }
    if observation
        .rules
        .iter()
        .flat_map(|rule| &rule.requirements)
        .flat_map(|requirement| &requirement.clauses)
        .any(|clause| !clause.exhaustive)
    {
        return Err(SignalAuthorizationDenial::NonExhaustiveObservation);
    }
    Ok(())
}

fn same_shape(
    contracts: &[SignalAuthorizationRuleContract],
    observations: &[SignalAuthorizationRuleObservation],
) -> bool {
    contracts.len() == observations.len()
        && contracts
            .iter()
            .zip(observations)
            .all(|(contract, observed)| {
                contract.effect == observed.effect
                    && contract.requirements.len() == observed.requirements.len()
                    && contract
                        .requirements
                        .iter()
                        .zip(&observed.requirements)
                        .all(|(required, actual)| required.clauses.len() == actual.clauses.len())
            })
}

fn evaluate_rule(
    rule: &SignalAuthorizationRuleObservation,
    counters: &mut SignalAuthorizationEvaluationCounters,
) -> bool {
    counters.rules_evaluated += 1;
    let mut rule_matches = true;
    for requirement in &rule.requirements {
        counters.requirements_evaluated += 1;
        let requirement_matches = evaluate_requirement(requirement, counters);
        counters.requirements_matched += usize::from(requirement_matches);
        rule_matches &= requirement_matches;
    }
    rule_matches
}

fn evaluate_requirement(
    requirement: &super::SignalAuthorizationRequirementObservation,
    counters: &mut SignalAuthorizationEvaluationCounters,
) -> bool {
    let mut requirement_matches = false;
    for clause in &requirement.clauses {
        counters.clauses_evaluated += 1;
        counters.entities_depended_on += clause.dependencies.entities;
        counters.relations_depended_on += clause.dependencies.relations;
        counters.adjacency_lists_depended_on += clause.dependencies.adjacency_lists;
        counters.fields_depended_on += clause.dependencies.fields;
        counters.clauses_matched += usize::from(clause.matched);
        requirement_matches |= clause.matched;
    }
    requirement_matches
}
