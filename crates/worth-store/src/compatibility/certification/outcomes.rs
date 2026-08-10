use serde::Serialize;

use super::super::admission::{
    CompatibilityAdmissionCounters, CompatibilityReadAdmissionOutcome, CompatibilityRejection,
    CompatibilityRejectionKind, CompatibilityRelation, CompatibilityWriteAdmissionOutcome,
    RestoreCompatibilityReceipt,
};

use super::super::derived::{DerivedLaneCompatibilityPlan, DerivedLaneCompatibilityPosture};

use super::super::restore::{DisasterRecoveryCompatibilityPlan, RestoreCompatibilityPlan};
use super::super::rolling::RollingUpgradeAdmissionPlan;
use crate::evidence::Milestone12AdmissionReport;

use super::lane_kinds::{
    Milestone12CertificationLaneId, Milestone12CertificationLaneInput,
    Milestone12CertificationLaneKind, Milestone12CertificationLaneRejection,
    Milestone12CertificationLaneStatus,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationLaneOutcome {
    lane_id: Milestone12CertificationLaneId,
    lane_kind: Milestone12CertificationLaneKind,
    input: Milestone12CertificationLaneInput,
    status: Milestone12CertificationLaneStatus,
    relation: Option<CompatibilityRelation>,
    rejection_kind: Option<CompatibilityRejectionKind>,
    counters: Milestone12AdmissionReport,
}

impl Milestone12CertificationLaneOutcome {
    pub(crate) fn accepted(
        lane_kind: Milestone12CertificationLaneKind,
        input: Milestone12CertificationLaneInput,
        relation: CompatibilityRelation,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self {
            lane_id: lane_kind.lane_id(),
            lane_kind,
            input,
            status: Milestone12CertificationLaneStatus::Accepted,
            relation: Some(relation),
            rejection_kind: None,
            counters: Milestone12AdmissionReport::from_admission_counters(counters),
        }
    }

    pub(crate) fn rejected(
        lane_kind: Milestone12CertificationLaneKind,
        input: Milestone12CertificationLaneInput,
        rejection_kind: CompatibilityRejectionKind,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self {
            lane_id: lane_kind.lane_id(),
            lane_kind,
            input,
            status: Milestone12CertificationLaneStatus::Rejected,
            relation: None,
            rejection_kind: Some(rejection_kind),
            counters: Milestone12AdmissionReport::from_admission_counters(counters),
        }
    }

    pub(crate) fn non_admitted(
        lane_kind: Milestone12CertificationLaneKind,
        input: Milestone12CertificationLaneInput,
        status: Milestone12CertificationLaneStatus,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self {
            lane_id: lane_kind.lane_id(),
            lane_kind,
            input,
            status,
            relation: None,
            rejection_kind: None,
            counters: Milestone12AdmissionReport::from_admission_counters(counters),
        }
    }

    pub(crate) fn accepted_from_report(
        lane_kind: Milestone12CertificationLaneKind,
        input: Milestone12CertificationLaneInput,
        relation: CompatibilityRelation,
        counters: Milestone12AdmissionReport,
    ) -> Self {
        Self {
            lane_id: lane_kind.lane_id(),
            lane_kind,
            input,
            status: Milestone12CertificationLaneStatus::Accepted,
            relation: Some(relation),
            rejection_kind: None,
            counters,
        }
    }

    pub fn from_read_outcome(
        lane_kind: Milestone12CertificationLaneKind,
        input: Milestone12CertificationLaneInput,
        outcome: &CompatibilityReadAdmissionOutcome,
    ) -> Result<Self, Milestone12CertificationLaneRejection> {
        outcome_from_relation_and_rejection(
            lane_kind,
            input,
            outcome.relation(),
            outcome.rejection_kind(),
            outcome.counters(),
        )
    }

    pub fn from_write_outcome(
        lane_kind: Milestone12CertificationLaneKind,
        input: Milestone12CertificationLaneInput,
        outcome: &CompatibilityWriteAdmissionOutcome,
    ) -> Result<Self, Milestone12CertificationLaneRejection> {
        outcome_from_relation_and_rejection(
            lane_kind,
            input,
            outcome.relation(),
            outcome.rejection_kind(),
            outcome.counters(),
        )
    }

    pub fn from_derived_plan(
        lane_kind: Milestone12CertificationLaneKind,
        input: Milestone12CertificationLaneInput,
        plan: &DerivedLaneCompatibilityPlan,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        let status = match plan.posture() {
            DerivedLaneCompatibilityPosture::ReuseAdmitted
            | DerivedLaneCompatibilityPosture::SupportAdmitted => {
                Milestone12CertificationLaneStatus::Accepted
            }
            DerivedLaneCompatibilityPosture::InvalidatedForRebuild => {
                Milestone12CertificationLaneStatus::Invalidated
            }
            DerivedLaneCompatibilityPosture::RebuildAdmitted => {
                Milestone12CertificationLaneStatus::RebuildRequired
            }
            DerivedLaneCompatibilityPosture::Rejected => {
                Milestone12CertificationLaneStatus::Rejected
            }
        };
        if status == Milestone12CertificationLaneStatus::Accepted {
            Self::accepted(lane_kind, input, CompatibilityRelation::Native, counters)
        } else {
            Self::non_admitted(lane_kind, input, status, counters)
        }
    }

    pub fn from_compatibility_rejection(
        lane_kind: Milestone12CertificationLaneKind,
        input: Milestone12CertificationLaneInput,
        rejection: &CompatibilityRejection,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self::rejected(lane_kind, input, rejection.kind(), counters)
    }

    pub fn from_rolling_plan(
        input: Milestone12CertificationLaneInput,
        plan: &RollingUpgradeAdmissionPlan,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self::accepted(
            Milestone12CertificationLaneKind::RollingTwoCapabilityAdmitted,
            input,
            plan.relation(),
            counters,
        )
    }

    pub fn from_restore_plan(
        input: Milestone12CertificationLaneInput,
        plan: &RestoreCompatibilityPlan,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self::accepted(
            Milestone12CertificationLaneKind::RestoreScopedBackupAdmitted,
            input,
            plan.relation(),
            counters,
        )
    }

    pub fn from_restore_receipt(
        input: Milestone12CertificationLaneInput,
        receipt: &RestoreCompatibilityReceipt,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self::accepted(
            Milestone12CertificationLaneKind::RestoreScopedBackupAdmitted,
            input,
            receipt.receipt().relation(),
            counters,
        )
    }

    pub fn from_disaster_recovery_plan(
        lane_kind: Milestone12CertificationLaneKind,
        input: Milestone12CertificationLaneInput,
        _plan: &DisasterRecoveryCompatibilityPlan,
        counters: &CompatibilityAdmissionCounters,
    ) -> Self {
        Self::non_admitted(
            lane_kind,
            input,
            Milestone12CertificationLaneStatus::EvidenceOnly,
            counters,
        )
    }

    pub fn lane_id(&self) -> &Milestone12CertificationLaneId {
        &self.lane_id
    }

    pub fn lane_kind(&self) -> Milestone12CertificationLaneKind {
        self.lane_kind
    }

    pub fn status(&self) -> Milestone12CertificationLaneStatus {
        self.status
    }

    pub fn relation(&self) -> Option<CompatibilityRelation> {
        self.relation
    }

    pub fn rejection_kind(&self) -> Option<CompatibilityRejectionKind> {
        self.rejection_kind
    }

    pub fn counters(&self) -> &Milestone12AdmissionReport {
        &self.counters
    }
}

fn outcome_from_relation_and_rejection(
    lane_kind: Milestone12CertificationLaneKind,
    input: Milestone12CertificationLaneInput,
    relation: Option<CompatibilityRelation>,
    rejection_kind: Option<CompatibilityRejectionKind>,
    counters: &CompatibilityAdmissionCounters,
) -> Result<Milestone12CertificationLaneOutcome, Milestone12CertificationLaneRejection> {
    match (relation, rejection_kind) {
        (Some(relation), None) => Ok(Milestone12CertificationLaneOutcome::accepted(
            lane_kind, input, relation, counters,
        )),
        (None, Some(rejection_kind)) => Ok(Milestone12CertificationLaneOutcome::rejected(
            lane_kind,
            input,
            rejection_kind,
            counters,
        )),
        _ => Err(Milestone12CertificationLaneRejection::OutcomeStatusMismatch),
    }
}
