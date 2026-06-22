use super::input::SpatialGeometryEvidenceTouchRejectedInputKind;
use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceLedgerError, WorkloadEvidenceStage, WorkloadEvidenceSupport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialGeometryEvidenceTouchDenialKind {
    SourceSubstitution,
    LedgerIncompleteness,
    SupportPosture,
    CounterHonesty,
    GuardFailure,
    StageLinkFailure,
    DiagnosticOnly,
    QueryGap,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialGeometryEvidenceTouchDenial {
    kind: SpatialGeometryEvidenceTouchDenialKind,
    locality: &'static str,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialGeometryEvidenceTouchDenialPrecedence {
    source_substitution: Option<SpatialGeometryEvidenceTouchRejectedInputKind>,
    ledger_incompleteness: Option<WorkloadEvidenceLedgerError>,
    support_posture: Option<(WorkloadEvidenceStage, WorkloadEvidenceSupport)>,
    counter_honesty: Option<WorkloadEvidenceLedgerError>,
    guard_failure: Option<WorkloadEvidenceLedgerError>,
    stage_link_failure: Option<WorkloadEvidenceLedgerError>,
    query_gap: Option<&'static str>,
}

impl SpatialGeometryEvidenceTouchDenial {
    pub(crate) fn source_substitution(
        input: SpatialGeometryEvidenceTouchRejectedInputKind,
        detail: impl Into<String>,
    ) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchDenialKind::SourceSubstitution,
            input.locality(),
            detail,
        )
    }

    pub(crate) fn support_posture(
        stage: WorkloadEvidenceStage,
        support: WorkloadEvidenceSupport,
    ) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchDenialKind::SupportPosture,
            "workload evidence support",
            format!(
                "{} has support {:?}, not admitted receipt support",
                stage.human_name(),
                support
            ),
        )
    }

    pub(crate) fn ledger_incompleteness(error: WorkloadEvidenceLedgerError) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchDenialKind::LedgerIncompleteness,
            "workload evidence complete ledger",
            error.human_reason(),
        )
    }

    pub(crate) fn counter_honesty(error: WorkloadEvidenceLedgerError) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchDenialKind::CounterHonesty,
            "workload evidence counters",
            error.human_reason(),
        )
    }

    pub(crate) fn guard_failure(error: WorkloadEvidenceLedgerError) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchDenialKind::GuardFailure,
            "workload evidence guard",
            error.human_reason(),
        )
    }

    pub(crate) fn stage_link_failure(error: WorkloadEvidenceLedgerError) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchDenialKind::StageLinkFailure,
            "workload evidence stage link",
            error.human_reason(),
        )
    }

    pub(crate) fn diagnostic_only(detail: impl Into<String>) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchDenialKind::DiagnosticOnly,
            "receipt-only diagnostic preview",
            detail,
        )
    }

    pub(crate) fn query_gap(detail: impl Into<String>) -> Self {
        Self::new(
            SpatialGeometryEvidenceTouchDenialKind::QueryGap,
            "forge-query lowering gap",
            detail,
        )
    }

    fn new(
        kind: SpatialGeometryEvidenceTouchDenialKind,
        locality: &'static str,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            locality,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> SpatialGeometryEvidenceTouchDenialKind {
        self.kind
    }

    pub fn locality(&self) -> &'static str {
        self.locality
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn human_reason(&self) -> String {
        format!("{}: {}", self.locality, self.detail)
    }
}

impl SpatialGeometryEvidenceTouchDenialPrecedence {
    pub fn new() -> Self {
        Self {
            source_substitution: None,
            ledger_incompleteness: None,
            support_posture: None,
            counter_honesty: None,
            guard_failure: None,
            stage_link_failure: None,
            query_gap: None,
        }
    }

    pub fn with_source_substitution(
        mut self,
        input: SpatialGeometryEvidenceTouchRejectedInputKind,
    ) -> Self {
        self.source_substitution = Some(input);
        self
    }

    pub fn with_support_posture(
        mut self,
        stage: WorkloadEvidenceStage,
        support: WorkloadEvidenceSupport,
    ) -> Self {
        self.support_posture = Some((stage, support));
        self
    }

    pub fn with_ledger_incompleteness(mut self, error: WorkloadEvidenceLedgerError) -> Self {
        self.ledger_incompleteness = Some(error);
        self
    }

    pub fn with_counter_honesty(mut self, error: WorkloadEvidenceLedgerError) -> Self {
        self.counter_honesty = Some(error);
        self
    }

    pub fn with_guard_failure(mut self, error: WorkloadEvidenceLedgerError) -> Self {
        self.guard_failure = Some(error);
        self
    }

    pub fn with_stage_link_failure(mut self, error: WorkloadEvidenceLedgerError) -> Self {
        self.stage_link_failure = Some(error);
        self
    }

    pub fn with_query_gap(mut self, gap: &'static str) -> Self {
        self.query_gap = Some(gap);
        self
    }

    pub fn deny(self) -> Option<SpatialGeometryEvidenceTouchDenial> {
        if let Some(input) = self.source_substitution {
            return Some(SpatialGeometryEvidenceTouchDenial::source_substitution(
                input,
                format!("{:?} cannot construct spatial touch authority", input),
            ));
        }
        if let Some((stage, support)) = self.support_posture {
            return Some(SpatialGeometryEvidenceTouchDenial::support_posture(
                stage, support,
            ));
        }
        if let Some(error) = self.ledger_incompleteness {
            return Some(SpatialGeometryEvidenceTouchDenial::ledger_incompleteness(
                error,
            ));
        }
        if let Some(error) = self.counter_honesty {
            return Some(SpatialGeometryEvidenceTouchDenial::counter_honesty(error));
        }
        if let Some(error) = self.guard_failure {
            return Some(SpatialGeometryEvidenceTouchDenial::guard_failure(error));
        }
        if let Some(error) = self.stage_link_failure {
            return Some(SpatialGeometryEvidenceTouchDenial::stage_link_failure(
                error,
            ));
        }
        self.query_gap
            .map(SpatialGeometryEvidenceTouchDenial::query_gap)
    }
}

impl Default for SpatialGeometryEvidenceTouchDenialPrecedence {
    fn default() -> Self {
        Self::new()
    }
}
