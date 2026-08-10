use crate::evidence_identity::WorthQueryEvidenceIdentity;

use super::super::stage::QuerySubscriptionDiagnosticStage;
use super::super::trace::QuerySubscriptionDiagnosticTrace;
use super::evidence::{
    QuerySubscriptionDiagnosticCounters, QuerySubscriptionDiagnosticSemanticLabels,
};
use super::failure::QuerySubscriptionDiagnosticFailure;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionDeniedDiagnosticBundle {
    trace: QuerySubscriptionDiagnosticTrace,
    semantic_labels: QuerySubscriptionDiagnosticSemanticLabels,
    failure: QuerySubscriptionDiagnosticFailure,
    omitted_stages: Vec<QuerySubscriptionDiagnosticStage>,
    pub(in crate::subscription) support_report_identity: Option<WorthQueryEvidenceIdentity>,
    pub(in crate::subscription) bundle_identity: WorthQueryEvidenceIdentity,
    counters: QuerySubscriptionDiagnosticCounters,
}

impl QuerySubscriptionDeniedDiagnosticBundle {
    pub fn trace(&self) -> &QuerySubscriptionDiagnosticTrace {
        &self.trace
    }

    pub fn semantic_labels(&self) -> &QuerySubscriptionDiagnosticSemanticLabels {
        &self.semantic_labels
    }

    pub fn failure(&self) -> &QuerySubscriptionDiagnosticFailure {
        &self.failure
    }

    pub fn omitted_stages(&self) -> &[QuerySubscriptionDiagnosticStage] {
        &self.omitted_stages
    }

    pub fn support_report_identity(&self) -> Option<&WorthQueryEvidenceIdentity> {
        self.support_report_identity.as_ref()
    }

    pub fn bundle_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.bundle_identity
    }

    pub fn counters(&self) -> &QuerySubscriptionDiagnosticCounters {
        &self.counters
    }
}

pub(super) struct DeniedDiagnosticBundleParts {
    pub(super) trace: QuerySubscriptionDiagnosticTrace,
    pub(super) semantic_labels: QuerySubscriptionDiagnosticSemanticLabels,
    pub(super) failure: QuerySubscriptionDiagnosticFailure,
    pub(super) omitted_stages: Vec<super::super::stage::QuerySubscriptionDiagnosticStage>,
    pub(super) support_report_identity: Option<WorthQueryEvidenceIdentity>,
    pub(super) bundle_identity: WorthQueryEvidenceIdentity,
    pub(super) counters: QuerySubscriptionDiagnosticCounters,
}

impl QuerySubscriptionDeniedDiagnosticBundle {
    pub(super) fn from_parts(parts: DeniedDiagnosticBundleParts) -> Self {
        Self {
            trace: parts.trace,
            semantic_labels: parts.semantic_labels,
            failure: parts.failure,
            omitted_stages: parts.omitted_stages,
            support_report_identity: parts.support_report_identity,
            bundle_identity: parts.bundle_identity,
            counters: parts.counters,
        }
    }
}
