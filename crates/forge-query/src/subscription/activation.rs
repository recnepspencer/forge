use crate::identity::hash_parts;

use super::admission::QuerySubscriptionAdmissionArtifact;
use super::counters::QuerySubscriptionDeclarationCounters;
use super::future_selection::QuerySubscriptionFutureSelection;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionActivationInput {
    activation_digest: String,
    admission_digest: String,
    query_declaration_digest: String,
    bridge_declaration_digest: String,
    future_selection: QuerySubscriptionFutureSelection,
    basis_binding_digest: String,
    checkpoint_identity_digest: String,
    signal_strategy_digest: String,
    counters: QuerySubscriptionDeclarationCounters,
}

impl SubscriptionActivationInput {
    pub fn activation_digest(&self) -> &str {
        &self.activation_digest
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }

    pub fn query_declaration_digest(&self) -> &str {
        &self.query_declaration_digest
    }

    pub fn bridge_declaration_digest(&self) -> &str {
        &self.bridge_declaration_digest
    }

    pub fn future_selection(&self) -> &QuerySubscriptionFutureSelection {
        &self.future_selection
    }

    pub fn basis_binding_digest(&self) -> &str {
        &self.basis_binding_digest
    }

    pub fn checkpoint_identity_digest(&self) -> &str {
        &self.checkpoint_identity_digest
    }

    pub fn signal_strategy_digest(&self) -> &str {
        &self.signal_strategy_digest
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
    let checkpoint_identity_digest = hash_parts(&[
        "query_subscription_active_checkpoint_identity_v1".to_string(),
        format!("query_declaration:{}", admission.query_declaration_digest()),
        format!("basis:{}", admission.basis_binding_digest()),
        format!(
            "future_selection:{}",
            admission.future_selection().projection_digest()
        ),
        format!("signal_strategy:{}", admission.signal_strategy_digest()),
    ]);
    let activation_digest = hash_parts(&[
        "query_subscription_activation_input_v1".to_string(),
        format!("admission:{}", admission.admission_digest()),
        format!("query_declaration:{}", admission.query_declaration_digest()),
        format!(
            "bridge_declaration:{}",
            admission.bridge_declaration_digest()
        ),
        format!(
            "future_selection:{}",
            admission.future_selection().projection_digest()
        ),
        format!("basis:{}", admission.basis_binding_digest()),
        format!("checkpoint:{}", checkpoint_identity_digest),
        format!("signal_strategy:{}", admission.signal_strategy_digest()),
    ]);
    SubscriptionActivationInput {
        activation_digest,
        admission_digest: admission.admission_digest().to_string(),
        query_declaration_digest: admission.query_declaration_digest().to_string(),
        bridge_declaration_digest: admission.bridge_declaration_digest().to_string(),
        future_selection: admission.future_selection().clone(),
        basis_binding_digest: admission.basis_binding_digest().to_string(),
        checkpoint_identity_digest,
        signal_strategy_digest: admission.signal_strategy_digest().to_string(),
        counters,
    }
}
