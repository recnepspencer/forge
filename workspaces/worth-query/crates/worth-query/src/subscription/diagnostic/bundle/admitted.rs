use crate::evidence_identity::WorthQueryEvidenceIdentity;

use super::super::trace::QuerySubscriptionDiagnosticTrace;
use super::evidence::{
    QuerySubscriptionDiagnosticCounters, QuerySubscriptionDiagnosticSemanticLabels,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionAdmittedDiagnosticBundle {
    trace: QuerySubscriptionDiagnosticTrace,
    semantic_labels: QuerySubscriptionDiagnosticSemanticLabels,
    pub(in crate::subscription) support_report_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) lifecycle_certification_identity: WorthQueryEvidenceIdentity,
    pub(in crate::subscription) continuation_identity: Option<WorthQueryEvidenceIdentity>,
    pub(in crate::subscription) preview_isolation_identity: Option<WorthQueryEvidenceIdentity>,
    pub(in crate::subscription) lifecycle_closeout_identity: Option<WorthQueryEvidenceIdentity>,
    pub(in crate::subscription) bundle_identity: WorthQueryEvidenceIdentity,
    counters: QuerySubscriptionDiagnosticCounters,
}

impl QuerySubscriptionAdmittedDiagnosticBundle {
    pub fn trace(&self) -> &QuerySubscriptionDiagnosticTrace {
        &self.trace
    }

    pub fn semantic_labels(&self) -> &QuerySubscriptionDiagnosticSemanticLabels {
        &self.semantic_labels
    }

    pub fn support_report_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.support_report_identity
    }

    pub fn lifecycle_certification_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.lifecycle_certification_identity
    }

    pub fn continuation_identity(&self) -> Option<&WorthQueryEvidenceIdentity> {
        self.continuation_identity.as_ref()
    }

    pub fn preview_isolation_identity(&self) -> Option<&WorthQueryEvidenceIdentity> {
        self.preview_isolation_identity.as_ref()
    }

    pub fn lifecycle_closeout_identity(&self) -> Option<&WorthQueryEvidenceIdentity> {
        self.lifecycle_closeout_identity.as_ref()
    }

    pub fn bundle_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.bundle_identity
    }

    pub fn counters(&self) -> &QuerySubscriptionDiagnosticCounters {
        &self.counters
    }
}

pub(super) struct AdmittedDiagnosticBundleParts {
    pub(super) trace: QuerySubscriptionDiagnosticTrace,
    pub(super) semantic_labels: QuerySubscriptionDiagnosticSemanticLabels,
    pub(super) support_report_identity: WorthQueryEvidenceIdentity,
    pub(super) lifecycle_certification_identity: WorthQueryEvidenceIdentity,
    pub(super) continuation_identity: Option<WorthQueryEvidenceIdentity>,
    pub(super) preview_isolation_identity: Option<WorthQueryEvidenceIdentity>,
    pub(super) lifecycle_closeout_identity: Option<WorthQueryEvidenceIdentity>,
    pub(super) bundle_identity: WorthQueryEvidenceIdentity,
    pub(super) counters: QuerySubscriptionDiagnosticCounters,
}

impl QuerySubscriptionAdmittedDiagnosticBundle {
    pub(super) fn from_parts(parts: AdmittedDiagnosticBundleParts) -> Self {
        Self {
            trace: parts.trace,
            semantic_labels: parts.semantic_labels,
            support_report_identity: parts.support_report_identity,
            lifecycle_certification_identity: parts.lifecycle_certification_identity,
            continuation_identity: parts.continuation_identity,
            preview_isolation_identity: parts.preview_isolation_identity,
            lifecycle_closeout_identity: parts.lifecycle_closeout_identity,
            bundle_identity: parts.bundle_identity,
            counters: parts.counters,
        }
    }
}
