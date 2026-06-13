use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::admission::QuerySubscriptionAdmissionArtifact;
use super::counters::QuerySubscriptionDeclarationCounters;
use super::evidence_identities::{
    activation_checkpoint_identity, activation_input_identity,
};
use super::future_selection::QuerySubscriptionFutureSelection;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionActivationInput {
    activation_identity: ForgeQueryEvidenceIdentity,
    admission_identity: ForgeQueryEvidenceIdentity,
    query_declaration_for_reporting: String,
    query_declaration_identity: ForgeQueryEvidenceIdentity,
    bridge_declaration_identity: ForgeQueryEvidenceIdentity,
    future_selection: QuerySubscriptionFutureSelection,
    basis_binding_identity: ForgeQueryEvidenceIdentity,
    checkpoint_identity: ForgeQueryEvidenceIdentity,
    signal_strategy_identity: ForgeQueryEvidenceIdentity,
    counters: QuerySubscriptionDeclarationCounters,
}

impl SubscriptionActivationInput {
    pub fn activation_for_reporting(&self) -> &str {
        self.activation_identity.as_str()
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.activation_identity
    }

    pub fn admission_for_reporting(&self) -> &str {
        self.admission_identity.as_str()
    }

    pub fn admission_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.admission_identity
    }

    pub fn query_declaration_for_reporting(&self) -> &str {
        &self.query_declaration_for_reporting
    }

    pub fn query_declaration_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.query_declaration_identity
    }

    pub fn bridge_declaration_for_reporting(&self) -> &str {
        self.bridge_declaration_identity.as_str()
    }

    pub fn bridge_declaration_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.bridge_declaration_identity
    }

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn basis_binding_for_reporting(&self) -> &str {
        self.basis_binding_identity.as_str()
    }

    pub fn basis_binding_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_binding_identity
    }

    pub fn checkpoint_for_reporting(&self) -> &str {
        self.checkpoint_identity.as_str()
    }

    pub fn checkpoint_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.checkpoint_identity
    }

    pub fn signal_strategy_for_reporting(&self) -> &str {
        self.signal_strategy_identity.as_str()
    }

    pub fn signal_strategy_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.signal_strategy_identity
    }

    pub fn counters(&self) -> &QuerySubscriptionDeclarationCounters {
        &self.counters
    }

}

pub fn prepare_subscription_activation(
    admission: QuerySubscriptionAdmissionArtifact,
) -> SubscriptionActivationInput {
    let mut counters = admission.counters().clone();
    counters.activation_input_count = 1;
    let checkpoint_identity = activation_checkpoint_identity(
        admission.query_declaration_identity(),
        admission.basis_binding_identity(),
        admission.future_selection().projection_identity(),
        admission.signal_strategy_identity(),
    );
    let activation_identity = activation_input_identity(
        admission.evidence_identity(),
        admission.query_declaration_identity(),
        admission.bridge_declaration_identity(),
        admission.basis_binding_identity(),
        &checkpoint_identity,
        admission.signal_strategy_identity(),
        admission.future_selection().projection_identity(),
        &counters.evidence_identity(),
    );
    SubscriptionActivationInput {
        activation_identity,
        admission_identity: admission.evidence_identity().clone(),
        query_declaration_for_reporting: admission.query_declaration_for_reporting().to_string(),
        query_declaration_identity: admission.query_declaration_identity().clone(),
        bridge_declaration_identity: admission.bridge_declaration_identity().clone(),
        future_selection: admission.future_selection().clone(),
        basis_binding_identity: admission.basis_binding_identity().clone(),
        checkpoint_identity,
        signal_strategy_identity: admission.signal_strategy_identity().clone(),
        counters,
    }
}
