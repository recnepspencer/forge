use std::collections::BTreeMap;
use std::sync::Arc;

use worth_query_installation::facade::{
    WorthQueryExecutionResourceEnvelope, WorthQueryExecutionStrategyContract,
    WorthQueryExecutionStrategyName,
};

use super::WorthQueryExecutionResourceSupportSnapshot;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryExecutionResourceAdmissionCounters {
    pub runtime_authority_checks: usize,
    pub input_contract_checks: usize,
    pub execution_contract_checks: usize,
    pub resource_contract_lookups: usize,
    pub support_snapshot_checks: usize,
    pub strategy_checks: usize,
    pub envelope_dimension_checks: usize,
    pub provider_session_mints: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryExecutionResourceAdmissionPosture {
    Exact,
    Degraded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAdmittedExecutionResourcePlan {
    identity: Arc<str>,
    request_identity: Arc<str>,
    envelope_identity: Arc<str>,
    envelope: Arc<WorthQueryExecutionResourceEnvelope>,
    support_snapshot: WorthQueryExecutionResourceSupportSnapshot,
    strategy: WorthQueryExecutionStrategyContract,
    posture: WorthQueryExecutionResourceAdmissionPosture,
    counters: WorthQueryExecutionResourceAdmissionCounters,
}

impl WorthQueryAdmittedExecutionResourcePlan {
    pub(crate) fn new(
        identity: String,
        request_identity: String,
        support_snapshot: WorthQueryExecutionResourceSupportSnapshot,
        strategy: WorthQueryExecutionStrategyContract,
        counters: WorthQueryExecutionResourceAdmissionCounters,
    ) -> Self {
        let envelope_identity = Arc::<str>::from(super::identity::admitted_envelope_identity(
            strategy.envelope(),
        ));
        let envelope = Arc::new(strategy.envelope().clone());
        let posture = if strategy.envelope().degradation().is_some() {
            WorthQueryExecutionResourceAdmissionPosture::Degraded
        } else {
            WorthQueryExecutionResourceAdmissionPosture::Exact
        };
        Self {
            identity: identity.into(),
            request_identity: request_identity.into(),
            envelope_identity,
            envelope,
            support_snapshot,
            strategy,
            posture,
            counters,
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn envelope_identity(&self) -> &str {
        &self.envelope_identity
    }

    pub fn support_snapshot(&self) -> &WorthQueryExecutionResourceSupportSnapshot {
        &self.support_snapshot
    }

    pub fn strategy(&self) -> &WorthQueryExecutionStrategyName {
        self.strategy.name()
    }

    pub fn envelope(&self) -> &WorthQueryExecutionResourceEnvelope {
        &self.envelope
    }

    pub fn posture(&self) -> WorthQueryExecutionResourceAdmissionPosture {
        self.posture
    }

    pub fn counters(&self) -> WorthQueryExecutionResourceAdmissionCounters {
        self.counters
    }

    pub(crate) fn record_provider_session_mint(&mut self) {
        self.counters.provider_session_mints += 1;
    }

    pub(crate) fn shared_envelope(&self) -> Arc<WorthQueryExecutionResourceEnvelope> {
        Arc::clone(&self.envelope)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryExecutionResourceAttemptEvidence {
    identity: Arc<str>,
    admission_identity: Arc<str>,
    request_identity: Arc<str>,
    strategy: Arc<str>,
    envelope_identity: Arc<str>,
    support_snapshot_identity: Arc<str>,
    provider_session_identity: Arc<str>,
    provider_session_attempt_identity: Arc<str>,
}

impl WorthQueryExecutionResourceAttemptEvidence {
    pub(crate) fn capture(
        plan: &WorthQueryAdmittedExecutionResourcePlan,
        session: &super::WorthQueryExecutionProviderSession,
    ) -> Self {
        let identity = Arc::<str>::from(crate::identity::hash_parts(&[
            "worth_query_execution_resource_attempt_evidence_v1".into(),
            format!("admission:{}", plan.identity()),
            format!("request:{}", plan.request_identity()),
            format!("strategy:{}", plan.strategy().as_str()),
            format!("envelope:{}", plan.envelope_identity()),
            format!("support:{}", plan.support_snapshot().identity()),
            format!("session:{}", session.identity()),
            format!("session-attempt:{}", session.attempt_identity()),
        ]));
        Self {
            identity,
            admission_identity: Arc::from(plan.identity()),
            request_identity: Arc::from(plan.request_identity()),
            strategy: Arc::from(plan.strategy().as_str()),
            envelope_identity: Arc::from(plan.envelope_identity()),
            support_snapshot_identity: Arc::from(plan.support_snapshot().identity()),
            provider_session_identity: Arc::from(session.identity()),
            provider_session_attempt_identity: Arc::from(session.attempt_identity()),
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn admission_identity(&self) -> &str {
        &self.admission_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn strategy(&self) -> &str {
        &self.strategy
    }

    pub fn envelope_identity(&self) -> &str {
        &self.envelope_identity
    }

    pub fn support_snapshot_identity(&self) -> &str {
        &self.support_snapshot_identity
    }

    pub fn provider_session_identity(&self) -> &str {
        &self.provider_session_identity
    }

    pub fn provider_session_attempt_identity(&self) -> &str {
        &self.provider_session_attempt_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAdmittedWorkflowResourcePlan {
    operation: WorthQueryAdmittedExecutionResourcePlan,
    stages: BTreeMap<String, Arc<WorthQueryAdmittedExecutionResourcePlan>>,
    identity: Arc<str>,
}

impl WorthQueryAdmittedWorkflowResourcePlan {
    pub(crate) fn new(
        operation: WorthQueryAdmittedExecutionResourcePlan,
        stages: BTreeMap<String, WorthQueryAdmittedExecutionResourcePlan>,
    ) -> Self {
        let stages = stages
            .into_iter()
            .map(|(identity, plan)| (identity, Arc::new(plan)))
            .collect::<BTreeMap<_, _>>();
        let identity = Arc::<str>::from(crate::identity::hash_parts(&[
            "worth_query_admitted_workflow_resource_plan_v1".into(),
            format!("operation:{}", operation.identity()),
            format!(
                "stages:{}",
                stages
                    .iter()
                    .map(|(stage, plan)| format!("{stage}:{}", plan.identity()))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        ]));
        Self {
            operation,
            stages,
            identity,
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn operation(&self) -> &WorthQueryAdmittedExecutionResourcePlan {
        &self.operation
    }

    pub fn stage(&self, identity: &str) -> Option<&WorthQueryAdmittedExecutionResourcePlan> {
        self.stages.get(identity).map(Arc::as_ref)
    }

    pub(crate) fn shared_stage(
        &self,
        identity: &str,
    ) -> Option<Arc<WorthQueryAdmittedExecutionResourcePlan>> {
        self.stages.get(identity).map(Arc::clone)
    }
}
