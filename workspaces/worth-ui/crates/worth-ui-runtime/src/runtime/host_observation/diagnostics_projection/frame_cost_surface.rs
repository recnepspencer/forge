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
    Counter,
    Evidence,
    BudgetDecision,
    DeniedWork,
    WidenedWork,
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
                WorthUiFrameCostSurfaceKind::Counter,
                counter_row_digest(row),
            )
        }));
        rows.extend(report.supporting_evidence_rows().iter().map(|row| {
            WorthUiFrameCostRow::new(
                WorthUiFrameCostSurfaceKind::Evidence,
                supporting_evidence_row_digest(row),
            )
        }));
        rows.extend(report.budget_decisions().iter().map(|row| {
            let digest = combine_digest(
                budget_kind_digest(row.kind()),
                u64::from(row.requested_units()).rotate_left(11)
                    ^ u64::from(row.admitted_units()).rotate_left(23),
            );
            WorthUiFrameCostRow::new(WorthUiFrameCostSurfaceKind::BudgetDecision, digest)
        }));
        rows.extend(report.denied_work().iter().map(|row| {
            WorthUiFrameCostRow::new(
                WorthUiFrameCostSurfaceKind::DeniedWork,
                work_class_digest(*row),
            )
        }));
        rows.extend(report.widened_work().iter().map(|row| {
            WorthUiFrameCostRow::new(
                WorthUiFrameCostSurfaceKind::WidenedWork,
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
        WorthUiFrameCostSurfaceKind::Counter => "foundational.counter",
        WorthUiFrameCostSurfaceKind::Evidence => "foundational.evidence",
        WorthUiFrameCostSurfaceKind::BudgetDecision => "foundational.budget_decision",
        WorthUiFrameCostSurfaceKind::DeniedWork => "foundational.denied_work",
        WorthUiFrameCostSurfaceKind::WidenedWork => "foundational.widened_work",
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
        FoundationalPerformanceWorkClass::AuthoritativeRead => "work.authoritative_read",
        FoundationalPerformanceWorkClass::AuthoritativeMutation => "work.authoritative_mutation",
        FoundationalPerformanceWorkClass::AuthoritativeObservation => {
            "work.authoritative_observation"
        }
        FoundationalPerformanceWorkClass::ValidationPlanning => "work.validation_planning",
        FoundationalPerformanceWorkClass::PublicationDelivery => "work.publication_delivery",
        FoundationalPerformanceWorkClass::ReplayReconstruction => "work.replay_reconstruction",
        FoundationalPerformanceWorkClass::SupportReportAssembly => "work.support_report_assembly",
        FoundationalPerformanceWorkClass::ForensicParity => "work.forensic_parity",
        FoundationalPerformanceWorkClass::StructuralCounterCapture => {
            "work.structural_counter_capture"
        }
        FoundationalPerformanceWorkClass::DiagnosticFactCapture => "work.diagnostic_fact_capture",
        FoundationalPerformanceWorkClass::DescriptiveLineageRecordMaintenance => {
            "work.descriptive_lineage_record_maintenance"
        }
        FoundationalPerformanceWorkClass::ProvenanceFactCapture => "work.provenance_fact_capture",
        FoundationalPerformanceWorkClass::ReplaySidecarMaintenance => {
            "work.replay_sidecar_maintenance"
        }
    })
}
