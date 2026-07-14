use worth_foundational::FoundationalMaterializedPerformanceReport;
use worth_foundational::{
    FoundationalPerformanceBudgetKind, FoundationalPerformanceCounterRow,
    FoundationalPerformanceSupportingEvidenceRow, FoundationalPerformanceWorkClass,
};

use super::digest::{combine_digest, stable_text_digest};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiFrameCostSurface {
    source_digest: u64,
    rows: Vec<WorthUiFrameCostRow>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiFrameCostSurfaceKind {
    FoundationalCounter,
    FoundationalEvidence,
    FoundationalBudgetDecision,
    FoundationalDeniedWork,
    FoundationalWidenedWork,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiFrameCostRow {
    kind: WorthUiFrameCostSurfaceKind,
    evidence_digest: u64,
}

impl WorthUiFrameCostSurface {
    pub(crate) fn absent() -> Self {
        Self {
            source_digest: 0,
            rows: Vec::new(),
        }
    }

    pub fn from_foundational_report<Source>(
        report: &FoundationalMaterializedPerformanceReport<Source>,
    ) -> Self {
        let mut rows = Vec::new();
        rows.extend(report.counter_rows().iter().map(|row| {
            WorthUiFrameCostRow::new(
                WorthUiFrameCostSurfaceKind::FoundationalCounter,
                counter_row_digest(row),
            )
        }));
        rows.extend(report.supporting_evidence_rows().iter().map(|row| {
            WorthUiFrameCostRow::new(
                WorthUiFrameCostSurfaceKind::FoundationalEvidence,
                supporting_evidence_row_digest(row),
            )
        }));
        rows.extend(report.budget_decisions().iter().map(|row| {
            let digest = combine_digest(
                budget_kind_digest(row.kind()),
                u64::from(row.requested_units()).rotate_left(11)
                    ^ u64::from(row.admitted_units()).rotate_left(23),
            );
            WorthUiFrameCostRow::new(
                WorthUiFrameCostSurfaceKind::FoundationalBudgetDecision,
                digest,
            )
        }));
        rows.extend(report.denied_work().iter().map(|row| {
            WorthUiFrameCostRow::new(
                WorthUiFrameCostSurfaceKind::FoundationalDeniedWork,
                work_class_digest(*row),
            )
        }));
        rows.extend(report.widened_work().iter().map(|row| {
            WorthUiFrameCostRow::new(
                WorthUiFrameCostSurfaceKind::FoundationalWidenedWork,
                work_class_digest(*row),
            )
        }));
        Self {
            source_digest: frame_cost_source_digest(&rows),
            rows,
        }
    }

    pub fn source_digest(&self) -> u64 {
        self.source_digest
    }

    pub fn rows(&self) -> &[WorthUiFrameCostRow] {
        &self.rows
    }
}

impl WorthUiFrameCostRow {
    fn new(kind: WorthUiFrameCostSurfaceKind, evidence_digest: u64) -> Self {
        Self {
            kind,
            evidence_digest,
        }
    }

    pub fn kind(&self) -> WorthUiFrameCostSurfaceKind {
        self.kind
    }

    pub fn evidence_digest(&self) -> u64 {
        self.evidence_digest
    }
}

fn frame_cost_source_digest(rows: &[WorthUiFrameCostRow]) -> u64 {
    rows.iter()
        .fold(stable_text_digest("frame_cost"), |digest, row| {
            combine_digest(
                combine_digest(digest, row.evidence_digest()),
                surface_kind_digest(row.kind()),
            )
        })
}

fn counter_row_digest(row: &FoundationalPerformanceCounterRow) -> u64 {
    combine_digest(
        stable_text_digest(row.name().as_str()),
        row.observed_count().rotate_left(7),
    )
}

fn supporting_evidence_row_digest(row: &FoundationalPerformanceSupportingEvidenceRow) -> u64 {
    combine_digest(
        stable_text_digest(row.code().as_str()),
        work_class_digest(row.related_work()),
    )
}

fn surface_kind_digest(kind: WorthUiFrameCostSurfaceKind) -> u64 {
    stable_text_digest(match kind {
        WorthUiFrameCostSurfaceKind::FoundationalCounter => "foundational.counter",
        WorthUiFrameCostSurfaceKind::FoundationalEvidence => "foundational.evidence",
        WorthUiFrameCostSurfaceKind::FoundationalBudgetDecision => "foundational.budget_decision",
        WorthUiFrameCostSurfaceKind::FoundationalDeniedWork => "foundational.denied_work",
        WorthUiFrameCostSurfaceKind::FoundationalWidenedWork => "foundational.widened_work",
    })
}

fn budget_kind_digest(kind: FoundationalPerformanceBudgetKind) -> u64 {
    stable_text_digest(match kind {
        FoundationalPerformanceBudgetKind::Breadth => "budget.breadth",
        FoundationalPerformanceBudgetKind::Density => "budget.density",
        FoundationalPerformanceBudgetKind::Locality => "budget.locality",
        FoundationalPerformanceBudgetKind::FreshnessSensitive => "budget.freshness_sensitive",
    })
}

fn work_class_digest(work_class: FoundationalPerformanceWorkClass) -> u64 {
    stable_text_digest(match work_class {
        FoundationalPerformanceWorkClass::AuthoritativeMutation => "work.authoritative_mutation",
        FoundationalPerformanceWorkClass::ValidationPlanning => "work.validation_planning",
        FoundationalPerformanceWorkClass::PublicationDelivery => "work.publication_delivery",
        FoundationalPerformanceWorkClass::ReplayReconstruction => "work.replay_reconstruction",
        FoundationalPerformanceWorkClass::SupportReportAssembly => "work.support_report_assembly",
        FoundationalPerformanceWorkClass::ForensicParity => "work.forensic_parity",
    })
}
