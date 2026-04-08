#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeBulkDecisionRecordKind {
    ParallelLegality,
    ParallelProfitability,
    ParallelAdmission,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeBulkDecisionRecord {
    kind: BridgeBulkDecisionRecordKind,
    class_label: Arc<str>,
    reason_label: Arc<str>,
    digest: Arc<str>,
}

impl BridgeBulkDecisionRecord {
    pub(crate) fn new(
        kind: BridgeBulkDecisionRecordKind,
        class_label: Arc<str>,
        reason_label: Arc<str>,
    ) -> Self {
        let basis = format!(
            "bridge-bulk-decision-record|kind={}|class={}|reason={}",
            super::super::planner::bulk_decision_kind_label(kind),
            class_label,
            reason_label,
        );
        Self {
            kind,
            class_label,
            reason_label,
            digest: digest_string("bridge-bulk-decision-record", &basis),
        }
    }

    pub fn kind(&self) -> BridgeBulkDecisionRecordKind {
        self.kind
    }

    pub fn class_label(&self) -> &str {
        self.class_label.as_ref()
    }

    pub fn reason_label(&self) -> &str {
        self.reason_label.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeBulkDecisionLog {
    records: Arc<[BridgeBulkDecisionRecord]>,
    digest: Arc<str>,
}

impl BridgeBulkDecisionLog {
    pub(crate) fn new(records: Vec<BridgeBulkDecisionRecord>) -> Self {
        let mut basis = format!("bridge-bulk-decision-log|record-count={}", records.len());
        for record in &records {
            basis.push_str("|record=");
            basis.push_str(record.digest());
        }
        Self {
            records: records.into(),
            digest: digest_string("bridge-bulk-decision-log", &basis),
        }
    }

    pub fn records(&self) -> &[BridgeBulkDecisionRecord] {
        &self.records
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeBulkPlanningFailureKind {
    WorkloadSummaryConstructionFailure,
    ZeroRoutedItemWorkload,
    UnsupportedPacketClass,
    InvalidReductionBasis,
    InvalidParallelAdmissionBasis,
    PacketOverlapDetected,
    ReductionIdentityConflict,
    ParallelPreparationNotProfitable,
    ReducerBufferCeilingExceeded,
    DiagnosticsFragmentCeilingExceeded,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeBulkPlanningFailure {
    kind: BridgeBulkPlanningFailureKind,
    boundary: Arc<str>,
    detail: Arc<str>,
    digest: Arc<str>,
}

impl BridgeBulkPlanningFailure {
    pub(crate) fn new(
        kind: BridgeBulkPlanningFailureKind,
        boundary: impl Into<Arc<str>>,
        detail: impl Into<Arc<str>>,
    ) -> Self {
        let boundary = boundary.into();
        let detail = detail.into();
        let basis = format!(
            "bridge-bulk-planning-failure|kind={}|boundary={}|detail={}",
            super::super::planner::planning_failure_kind_label(kind),
            boundary,
            detail,
        );
        Self {
            kind,
            boundary,
            detail,
            digest: digest_string("bridge-bulk-planning-failure", &basis),
        }
    }

    pub fn kind(&self) -> BridgeBulkPlanningFailureKind {
        self.kind
    }

    pub fn boundary(&self) -> &str {
        self.boundary.as_ref()
    }

    pub fn detail(&self) -> &str {
        self.detail.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

use super::*;
