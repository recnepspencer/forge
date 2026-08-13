//! Atomic installation of authorization policy definitions.

use std::collections::BTreeSet;
use std::sync::Arc;

use crate::data::aspect::InstalledSignalGraphCapability;
use crate::data::graph::SignalGraph;

use super::{
    InstalledSignalAuthorizationPolicy, SignalAuthorizationAuthority, SignalAuthorizationDenial,
    SignalAuthorizationPolicyDefinition, SignalAuthorizationRuleEffect,
};

impl SignalGraph {
    pub fn install_authorization_policy(
        &mut self,
        graph: &InstalledSignalGraphCapability,
        definition: SignalAuthorizationPolicyDefinition,
    ) -> Result<InstalledSignalAuthorizationPolicy, SignalAuthorizationDenial> {
        let mut installed = self.install_authorization_policies(graph, [definition])?;
        Ok(installed
            .pop()
            .expect("one validated definition installs one policy"))
    }

    pub fn install_authorization_policies(
        &mut self,
        graph: &InstalledSignalGraphCapability,
        definitions: impl IntoIterator<Item = SignalAuthorizationPolicyDefinition>,
    ) -> Result<Vec<InstalledSignalAuthorizationPolicy>, SignalAuthorizationDenial> {
        let definitions = definitions.into_iter().collect::<Vec<_>>();
        validate_installation_batch(self, graph, &definitions)?;

        let runtime_instance_id = self.runtime_instance_id();
        let policies = definitions
            .into_iter()
            .map(|definition| InstalledSignalAuthorizationPolicy {
                graph_instance_id: runtime_instance_id,
                identity: definition.identity,
                rules: definition.rules,
                authority: Arc::new(SignalAuthorizationAuthority { _seal: () }),
            })
            .collect::<Vec<_>>();
        self.authorization_policy_identities
            .extend(policies.iter().map(|policy| *policy.identity.bytes()));
        Ok(policies)
    }
}

fn validate_installation_batch(
    graph: &SignalGraph,
    capability: &InstalledSignalGraphCapability,
    definitions: &[SignalAuthorizationPolicyDefinition],
) -> Result<(), SignalAuthorizationDenial> {
    if capability.graph_instance_id() != graph.runtime_instance_id() {
        return Err(SignalAuthorizationDenial::ForeignGraph);
    }
    let mut batch_identities = BTreeSet::new();
    for definition in definitions {
        validate_definition(graph, definition)?;
        if !batch_identities.insert(*definition.identity.bytes()) {
            return Err(SignalAuthorizationDenial::DuplicatePolicy);
        }
    }
    Ok(())
}

fn validate_definition(
    graph: &SignalGraph,
    definition: &SignalAuthorizationPolicyDefinition,
) -> Result<(), SignalAuthorizationDenial> {
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
