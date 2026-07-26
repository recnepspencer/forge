use std::sync::Arc;

use worth_query_admission::facade::domain_computation::WorthQueryAdmittedConvergenceContract;

use crate::execution_digest::hash_parts;

use super::WorthQueryConvergenceEpochCounters;
use super::{WorthQueryBoundConvergenceReport, WorthQueryRetainedConvergenceCandidateEvidence};

pub(super) struct WorthQueryConvergenceEpochCore {
    identity: Arc<str>,
    logical_run_identity: Arc<str>,
    contract: WorthQueryAdmittedConvergenceContract,
    counters: WorthQueryConvergenceEpochCounters,
    incumbents: Vec<WorthQueryRetainedConvergenceCandidateEvidence>,
    latest_report: Option<WorthQueryBoundConvergenceReport>,
}

impl WorthQueryConvergenceEpochCore {
    pub(super) fn new(
        operation_binding_identity: &str,
        logical_run_identity: &str,
        managed_run_identity: &str,
        graph_authority_identity: &str,
        contract: WorthQueryAdmittedConvergenceContract,
        counters: WorthQueryConvergenceEpochCounters,
    ) -> Self {
        let identity = Arc::from(hash_parts(&[
            "worth_query_convergence_epoch_v1".into(),
            format!("operation-binding:{operation_binding_identity}"),
            format!("logical-run:{logical_run_identity}"),
            format!("managed-run:{managed_run_identity}"),
            format!("graph-authority:{graph_authority_identity}"),
            format!("contract:{}", contract.identity()),
        ]));
        Self {
            identity,
            logical_run_identity: Arc::from(logical_run_identity),
            contract,
            counters,
            incumbents: Vec::new(),
            latest_report: None,
        }
    }

    pub(super) fn identity(&self) -> &str {
        &self.identity
    }

    pub(super) fn logical_run_identity(&self) -> &str {
        &self.logical_run_identity
    }

    pub(super) fn contract(&self) -> &WorthQueryAdmittedConvergenceContract {
        &self.contract
    }

    pub(super) fn counters(&self) -> &WorthQueryConvergenceEpochCounters {
        &self.counters
    }

    pub(super) fn counters_mut(&mut self) -> &mut WorthQueryConvergenceEpochCounters {
        &mut self.counters
    }

    pub(super) fn incumbents(&self) -> &[WorthQueryRetainedConvergenceCandidateEvidence] {
        &self.incumbents
    }

    pub(super) fn incumbents_mut(
        &mut self,
    ) -> &mut Vec<WorthQueryRetainedConvergenceCandidateEvidence> {
        &mut self.incumbents
    }

    pub(super) fn latest_report(&self) -> Option<&WorthQueryBoundConvergenceReport> {
        self.latest_report.as_ref()
    }

    pub(super) fn replace_latest_report(&mut self, report: WorthQueryBoundConvergenceReport) {
        self.latest_report = Some(report);
    }
}
