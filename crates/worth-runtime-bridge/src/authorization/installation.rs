//! Plan-before-effects installation for Bridge authorization correspondences.

use std::collections::BTreeSet;

use worth_signal::facade::{
    SignalAuthorizationPolicyDefinition, SignalAuthorizationPolicyIdentity,
};

use super::runtime::{lower_rule_contract, BridgeInstalledAuthorizationCorrespondence};
use super::{
    BridgeAuthorizationCorrespondenceIdentity, BridgeAuthorizationDenial,
    BridgeAuthorizationDenialKind, BridgeAuthorizationInstallationRequest,
    BridgeAuthorizationRuleContract, BridgeAuthorizationRuleEffect, BridgeAuthorizationRuntime,
};

pub struct BridgeAuthorizationInstallationBatch {
    requests: Vec<BridgeAuthorizationInstallationRequest>,
    identities: BTreeSet<BridgeAuthorizationCorrespondenceIdentity>,
    rejection: Option<BridgeAuthorizationDenial>,
}

impl BridgeAuthorizationInstallationBatch {
    pub fn new() -> Self {
        Self {
            requests: Vec::new(),
            identities: BTreeSet::new(),
            rejection: None,
        }
    }

    pub fn add(
        &mut self,
        request: BridgeAuthorizationInstallationRequest,
    ) -> Result<BridgeAuthorizationCorrespondenceIdentity, BridgeAuthorizationDenial> {
        if let Err(denial) = validate_request(&request) {
            self.rejection = Some(denial.clone());
            return Err(denial);
        }
        let identity = request.correspondence;
        if !self.identities.insert(identity) {
            let denial = denial(
                BridgeAuthorizationDenialKind::DuplicateCorrespondence,
                &request.policy,
            );
            self.rejection = Some(denial.clone());
            return Err(denial);
        }
        self.requests.push(request);
        Ok(identity)
    }
}

impl Default for BridgeAuthorizationInstallationBatch {
    fn default() -> Self {
        Self::new()
    }
}

impl BridgeAuthorizationRuntime {
    pub fn install(
        &mut self,
        request: BridgeAuthorizationInstallationRequest,
    ) -> Result<BridgeAuthorizationCorrespondenceIdentity, BridgeAuthorizationDenial> {
        let mut batch = BridgeAuthorizationInstallationBatch::new();
        let identity = batch.add(request)?;
        self.install_batch(batch)?;
        Ok(identity)
    }

    pub fn install_batch(
        &mut self,
        batch: BridgeAuthorizationInstallationBatch,
    ) -> Result<Vec<BridgeAuthorizationCorrespondenceIdentity>, BridgeAuthorizationDenial> {
        if let Some(rejection) = batch.rejection {
            return Err(rejection);
        }
        for request in &batch.requests {
            if self.correspondences.contains_key(&request.correspondence) {
                return Err(denial(
                    BridgeAuthorizationDenialKind::DuplicateCorrespondence,
                    &request.policy,
                ));
            }
        }

        let definitions = batch.requests.iter().map(|request| {
            SignalAuthorizationPolicyDefinition::new(
                SignalAuthorizationPolicyIdentity::new(*request.correspondence.bytes()),
                request.rules.iter().map(lower_rule_contract),
            )
        });
        let graph_capability = self.graph.installed_graph_capability();
        let signal_policies = self
            .graph
            .install_authorization_policies(&graph_capability, definitions)
            .map_err(|_| {
                denial(
                    BridgeAuthorizationDenialKind::SignalInstallationRejected,
                    "authorization installation batch",
                )
            })?;

        let mut installed_identities = Vec::with_capacity(batch.requests.len());
        for (request, signal_policy) in batch.requests.into_iter().zip(signal_policies) {
            let identity = request.correspondence;
            installed_identities.push(identity);
            self.correspondences.insert(
                identity,
                BridgeInstalledAuthorizationCorrespondence::from_installation(
                    request,
                    signal_policy,
                ),
            );
        }
        Ok(installed_identities)
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

fn denial(
    kind: BridgeAuthorizationDenialKind,
    subject: impl Into<String>,
) -> BridgeAuthorizationDenial {
    BridgeAuthorizationDenial::new(kind, subject)
}
