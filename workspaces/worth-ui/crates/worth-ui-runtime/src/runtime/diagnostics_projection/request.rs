use worth_foundational::FoundationalMaterializedPerformanceReport;

use crate::runtime::diagnostics_projection::digest::{combine_digest, digest_debug};
use crate::runtime::diagnostics_projection::{
    WorthUiDiagnosticsProjection, WorthUiDiagnosticsProjectionCounters,
    WorthUiDiagnosticsProjectionDenial, WorthUiDiagnosticsProjectionDenialReason,
    WorthUiDiagnosticsProjectionHook, WorthUiDiagnosticsProjectionHookEffect,
    WorthUiFrameCostSurface, WorthUiPlanInspectionSurface, WorthUiQueryStatusSurface,
    WorthUiReloadStatusSurface,
};
use crate::runtime::{
    WorthUiExecutionPlanInspection, WorthUiRuntimeDiagnosticReport, WorthUiRuntimeHost,
};

#[derive(Debug)]
pub struct WorthUiRuntimeDiagnosticsProjection<'a> {
    runtime: &'a WorthUiRuntimeHost,
}

#[derive(Debug)]
pub struct WorthUiDiagnosticsProjectionRequest<'a> {
    runtime: &'a WorthUiRuntimeHost,
    report: Option<&'a WorthUiRuntimeDiagnosticReport>,
    plan_inspection: Option<&'a WorthUiExecutionPlanInspection>,
    frame_costs: WorthUiFrameCostSurface,
    hooks: Vec<WorthUiDiagnosticsProjectionHook>,
    counters: WorthUiDiagnosticsProjectionCounters,
}

impl WorthUiRuntimeHost {
    pub fn diagnostics_projection(&self) -> WorthUiRuntimeDiagnosticsProjection<'_> {
        WorthUiRuntimeDiagnosticsProjection { runtime: self }
    }
}

impl<'a> WorthUiRuntimeDiagnosticsProjection<'a> {
    pub fn from_report(
        self,
        report: &'a WorthUiRuntimeDiagnosticReport,
    ) -> WorthUiDiagnosticsProjectionRequest<'a> {
        WorthUiDiagnosticsProjectionRequest {
            runtime: self.runtime,
            report: Some(report),
            plan_inspection: None,
            frame_costs: WorthUiFrameCostSurface::absent(),
            hooks: Vec::new(),
            counters: WorthUiDiagnosticsProjectionCounters::default(),
        }
    }
}

impl<'a> WorthUiDiagnosticsProjectionRequest<'a> {
    pub fn with_plan_inspection(mut self, inspection: &'a WorthUiExecutionPlanInspection) -> Self {
        self.plan_inspection = Some(inspection);
        self
    }

    pub fn with_frame_costs<Source>(
        mut self,
        report: &FoundationalMaterializedPerformanceReport<Source>,
    ) -> Self {
        self.frame_costs = WorthUiFrameCostSurface::from_foundational_report(report);
        self
    }

    pub fn with_hook(mut self, hook: WorthUiDiagnosticsProjectionHook) -> Self {
        self.hooks.push(hook);
        self
    }

    pub fn project(
        mut self,
    ) -> Result<WorthUiDiagnosticsProjection, WorthUiDiagnosticsProjectionDenial> {
        let report = self.report_or_denial()?;
        self.reject_report_from_nonactive_runtime(report)?;
        self.reject_mismatched_plan_inspection(report)?;
        self.admit_presentation_hooks()?;
        Ok(self.materialize(report))
    }

    fn report_or_denial(
        &self,
    ) -> Result<&'a WorthUiRuntimeDiagnosticReport, WorthUiDiagnosticsProjectionDenial> {
        self.report.ok_or_else(|| {
            self.denial(
                WorthUiDiagnosticsProjectionDenialReason::MissingRuntimeDiagnosticReport,
                None,
            )
        })
    }

    fn reject_report_from_nonactive_runtime(
        &self,
        report: &WorthUiRuntimeDiagnosticReport,
    ) -> Result<(), WorthUiDiagnosticsProjectionDenial> {
        let active = self.runtime.inspect_active();
        if report.active_artifact_digest() == active.artifact_digest()
            && report.active_plan_digest() == active.active_plan_digest()
        {
            return Ok(());
        }
        Err(self.denial(
            WorthUiDiagnosticsProjectionDenialReason::RuntimeReportDigestMismatch,
            Some(report.active_plan_digest()),
        ))
    }

    fn reject_mismatched_plan_inspection(
        &self,
        report: &WorthUiRuntimeDiagnosticReport,
    ) -> Result<(), WorthUiDiagnosticsProjectionDenial> {
        let Some(inspection) = self.plan_inspection else {
            return Ok(());
        };
        if inspection.plan_digest().raw() == report.active_plan_digest() {
            return Ok(());
        }
        Err(self.denial(
            WorthUiDiagnosticsProjectionDenialReason::PlanInspectionDigestMismatch,
            Some(inspection.plan_digest().raw()),
        ))
    }

    fn admit_presentation_hooks(&mut self) -> Result<(), WorthUiDiagnosticsProjectionDenial> {
        for hook in &self.hooks {
            if matches!(
                hook.effect(),
                WorthUiDiagnosticsProjectionHookEffect::IdentityRewriteAttempt { .. }
            ) {
                return Err(self.denial(
                    WorthUiDiagnosticsProjectionDenialReason::HookAttemptedIdentityRewrite,
                    Some(hook.projection_digest()),
                ));
            }
            self.counters.record_hook();
        }
        Ok(())
    }

    fn materialize(
        mut self,
        report: &WorthUiRuntimeDiagnosticReport,
    ) -> WorthUiDiagnosticsProjection {
        let rows = report.rows().to_vec();
        self.counters.record_runtime_rows(rows.len());
        let reload_status = WorthUiReloadStatusSurface::from_runtime_rows(
            report.active_artifact_digest(),
            report.active_plan_digest(),
            &rows,
        );
        self.counters
            .record_query_rows(query_row_count(&rows, self.plan_inspection));
        self.counters
            .record_plan_rows(self.plan_inspection.map_or(0, |inspection| {
                inspection.nodes().len() + inspection.lanes().len()
            }));
        self.counters
            .record_frame_cost_rows(self.frame_costs.rows().len());
        for _ in reload_status.failures() {
            self.counters.record_reload_row();
        }
        let plan_inspection = self.plan_inspection.map_or_else(
            || WorthUiPlanInspectionSurface::absent(report.active_plan_digest()),
            WorthUiPlanInspectionSurface::from_inspection,
        );
        let query_status = WorthUiQueryStatusSurface::from_sources(&rows, self.plan_inspection);
        let projection_digest = self.projection_digest(report, &rows);
        WorthUiDiagnosticsProjection::new(
            report.active_artifact_digest(),
            report.active_plan_digest(),
            projection_digest,
            rows,
            reload_status,
            plan_inspection,
            self.frame_costs,
            query_status,
            self.counters,
        )
    }

    fn projection_digest(
        &self,
        report: &WorthUiRuntimeDiagnosticReport,
        rows: &[crate::runtime::WorthUiRuntimeDiagnostic],
    ) -> u64 {
        let mut digest = report.active_artifact_digest();
        digest = combine_digest(digest, report.active_plan_digest());
        digest = combine_digest(digest, digest_debug(&rows));
        if let Some(inspection) = self.plan_inspection {
            digest = combine_digest(digest, inspection.plan_digest().raw());
        }
        digest = combine_digest(digest, self.frame_costs.source_digest());
        for hook in &self.hooks {
            digest = combine_digest(digest, hook.projection_digest());
        }
        digest
    }

    fn denial(
        &self,
        reason: WorthUiDiagnosticsProjectionDenialReason,
        evidence_digest: Option<u64>,
    ) -> WorthUiDiagnosticsProjectionDenial {
        WorthUiDiagnosticsProjectionDenial::new(
            reason,
            self.runtime.inspect_active().active_plan_digest(),
            evidence_digest,
        )
    }
}

fn query_row_count(
    rows: &[crate::runtime::WorthUiRuntimeDiagnostic],
    inspection: Option<&WorthUiExecutionPlanInspection>,
) -> usize {
    let diagnostic_rows = rows
        .iter()
        .filter(|row| {
            matches!(
                row.source(),
                crate::runtime::WorthUiDiagnosticSource::QueryStop { .. }
            )
        })
        .count();
    let inspection_rows = inspection.map_or(0, |inspection| {
        inspection
            .nodes()
            .iter()
            .filter(|node| node.query_inspection_links().is_some())
            .count()
    });
    diagnostic_rows + inspection_rows
}
