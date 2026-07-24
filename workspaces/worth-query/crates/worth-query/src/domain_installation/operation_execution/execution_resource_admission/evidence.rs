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
        let posture = if strategy.envelope().degradation().is_some() {
            WorthQueryExecutionResourceAdmissionPosture::Degraded
        } else {
            WorthQueryExecutionResourceAdmissionPosture::Exact
        };
        Self {
            identity: identity.into(),
            request_identity: request_identity.into(),
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

    pub fn support_snapshot(&self) -> &WorthQueryExecutionResourceSupportSnapshot {
        &self.support_snapshot
    }

    pub fn strategy(&self) -> &WorthQueryExecutionStrategyName {
        self.strategy.name()
    }

    pub fn envelope(&self) -> &WorthQueryExecutionResourceEnvelope {
        self.strategy.envelope()
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAdmittedWorkflowResourcePlan {
    operation: WorthQueryAdmittedExecutionResourcePlan,
    stages: BTreeMap<String, WorthQueryAdmittedExecutionResourcePlan>,
    identity: Arc<str>,
}

impl WorthQueryAdmittedWorkflowResourcePlan {
    pub(crate) fn new(
        operation: WorthQueryAdmittedExecutionResourcePlan,
        stages: BTreeMap<String, WorthQueryAdmittedExecutionResourcePlan>,
    ) -> Self {
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
        self.stages.get(identity)
    }
}
