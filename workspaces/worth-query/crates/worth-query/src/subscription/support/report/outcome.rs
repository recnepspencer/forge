use crate::evidence_identity::WorthQueryEvidenceIdentity;
use crate::identity_authority::{QueryProjectionIdentity, QuerySubscriptionIdentityKind};

use super::super::super::evidence_projection::subscription_evidence_projection;
use super::super::matrix::QuerySubscriptionSupportMatrix;
use super::super::subject::{QuerySubscriptionSupportPosture, QuerySubscriptionSupportSubject};
use super::counters::QuerySubscriptionSupportCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionSupportReport {
    support_subject: QuerySubscriptionSupportSubject,
    support_posture: QuerySubscriptionSupportPosture,
    support_matrix: QuerySubscriptionSupportMatrix,
    source_identity: WorthQueryEvidenceIdentity,
    counter_snapshot_identity: WorthQueryEvidenceIdentity,
    lookup_receipt_identity: WorthQueryEvidenceIdentity,
    report_identity: WorthQueryEvidenceIdentity,
    counters: QuerySubscriptionSupportCounters,
}

impl QuerySubscriptionSupportReport {
    pub(super) fn new(
        support_subject: QuerySubscriptionSupportSubject,
        support_posture: QuerySubscriptionSupportPosture,
        support_matrix: QuerySubscriptionSupportMatrix,
        source_identity: WorthQueryEvidenceIdentity,
        counter_snapshot_identity: WorthQueryEvidenceIdentity,
        lookup_receipt_identity: WorthQueryEvidenceIdentity,
        report_identity: WorthQueryEvidenceIdentity,
        counters: QuerySubscriptionSupportCounters,
    ) -> Self {
        Self {
            support_subject,
            support_posture,
            support_matrix,
            source_identity,
            counter_snapshot_identity,
            lookup_receipt_identity,
            report_identity,
            counters,
        }
    }
    pub fn support_subject(&self) -> &QuerySubscriptionSupportSubject {
        &self.support_subject
    }

    pub fn support_posture(&self) -> &QuerySubscriptionSupportPosture {
        &self.support_posture
    }

    pub fn support_matrix(&self) -> &QuerySubscriptionSupportMatrix {
        &self.support_matrix
    }

    pub fn source_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.source_identity)
    }

    pub fn source_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.source_identity
    }

    pub fn counter_snapshot_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.counter_snapshot_identity)
    }

    pub fn counter_snapshot_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.counter_snapshot_identity
    }

    pub fn lookup_receipt_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.lookup_receipt_identity)
    }

    pub fn lookup_receipt_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.lookup_receipt_identity
    }

    pub fn report_projection(
        &self,
    ) -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
        subscription_evidence_projection(&self.report_identity)
    }

    pub fn report_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.report_identity
    }

    pub fn counters(&self) -> &QuerySubscriptionSupportCounters {
        &self.counters
    }
}
