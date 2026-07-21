use crate::runtime::host_observation::diagnostics::{
    WorthUiDiagnosticMaterialization, WorthUiDiagnosticRichnessPolicy,
    WorthUiDiagnosticSupportReport, WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCounters,
    WorthUiSupportReportPolicy,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeDiagnosticReport {
    active_artifact_digest: u64,
    active_plan_digest: u64,
    rows: Vec<WorthUiRuntimeDiagnostic>,
    materialization: WorthUiDiagnosticMaterialization,
    support_report: WorthUiDiagnosticSupportReport,
    counters: WorthUiRuntimeDiagnosticCounters,
}

impl WorthUiRuntimeDiagnosticReport {
    pub(crate) fn materialize(
        active_artifact_digest: u64,
        active_plan_digest: u64,
        rows: Vec<WorthUiRuntimeDiagnostic>,
        policy: WorthUiDiagnosticRichnessPolicy,
    ) -> Self {
        let tier = policy.tier();
        let support_policy = WorthUiSupportReportPolicy::from_diagnostic_policy(policy);
        let materialization = WorthUiDiagnosticMaterialization::from_tier(tier);
        let mut counters = WorthUiRuntimeDiagnosticCounters::default();
        for row in &rows {
            counters.record_source_row();
            if tier.emits_rows() {
                counters.record_emitted_row();
            }
            if tier.emits_phase_references() && row.phase_reference_digest().is_some() {
                counters.record_phase_reference();
            }
            if tier.emits_query_links()
                && matches!(
                    row.source(),
                    crate::runtime::host_observation::diagnostics::WorthUiDiagnosticSource::QueryStop { .. }
                )
            {
                counters.record_query_link();
            }
        }
        let rows = if tier.emits_rows() { rows } else { Vec::new() };
        let support_report = if support_policy.may_materialize_support_sections() {
            counters.record_support_section();
            counters.record_rich_materialization();
            WorthUiDiagnosticSupportReport::materialized(1)
        } else {
            if matches!(
                tier,
                crate::runtime::host_observation::diagnostics::WorthUiDiagnosticRichnessTier::Full
            ) {
                counters.record_rich_materialization();
            }
            WorthUiDiagnosticSupportReport::elided()
        };
        Self {
            active_artifact_digest,
            active_plan_digest,
            rows,
            materialization,
            support_report,
            counters,
        }
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn active_plan_digest(&self) -> u64 {
        self.active_plan_digest
    }

    pub fn rows(&self) -> &[WorthUiRuntimeDiagnostic] {
        &self.rows
    }

    pub fn materialization(&self) -> WorthUiDiagnosticMaterialization {
        self.materialization
    }

    pub fn support_report(&self) -> &WorthUiDiagnosticSupportReport {
        &self.support_report
    }

    pub fn counters(&self) -> WorthUiRuntimeDiagnosticCounters {
        self.counters
    }
}
