use forge_foundational::{
    performance_api::lower_lane::reports::{
        attach_counter_backed_performance_receipt, plan_performance_report,
        FoundationalPerformanceAttachmentTargetKind,
    },
    profiles, AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, FoundationalPerformanceReportMaterializationBoundary,
    FoundationalPerformanceReportRequest, FoundationalProfileSet, RetentionDeliveryProfile,
    SupportPostureProfile,
};

use super::denial::{WorthUiSteadyFrameCounterDenial, WorthUiSteadyFrameCounterDenialReason};
use super::foundational_bridge::WorthUiSteadyFrameFoundationalEvidence;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiFrameReportMaterializationBoundary {
    ClaimInspectionOnly,
    ReportAssembly,
    SupportExpansion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSteadyFrameReportPlan {
    materialization_boundary: WorthUiFrameReportMaterializationBoundary,
    source_receipt_count: usize,
    foundational_boundaries: Vec<FoundationalPerformanceReportMaterializationBoundary>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiSteadyFrameReportPlanner {
    materialization_boundary: WorthUiFrameReportMaterializationBoundary,
}

impl WorthUiSteadyFrameReportPlanner {
    pub fn claim_inspection_only() -> Self {
        Self {
            materialization_boundary:
                WorthUiFrameReportMaterializationBoundary::ClaimInspectionOnly,
        }
    }

    pub fn report_assembly() -> Self {
        Self {
            materialization_boundary: WorthUiFrameReportMaterializationBoundary::ReportAssembly,
        }
    }

    pub fn support_report() -> Self {
        Self {
            materialization_boundary: WorthUiFrameReportMaterializationBoundary::SupportExpansion,
        }
    }

    pub fn plan_from_foundational_receipts(
        self,
        evidence: &WorthUiSteadyFrameFoundationalEvidence,
    ) -> Result<WorthUiSteadyFrameReportPlan, WorthUiSteadyFrameCounterDenial> {
        let profile = self.foundational_profile()?;
        let mut foundational_boundaries = Vec::with_capacity(evidence.receipt_count());
        for receipt in evidence.evidence() {
            let attached_receipt = attach_counter_backed_performance_receipt(
                FoundationalPerformanceAttachmentTargetKind::BoundaryReport,
                receipt.counter_backed_receipt().clone(),
            )
            .map_err(|_| report_planning_denial())?;
            let plan = plan_performance_report(FoundationalPerformanceReportRequest {
                source: attached_receipt,
                profile,
                include_layout_intent: false,
                include_contract_names: self.requests_report_assembly(),
                include_counter_specs: self.requests_report_assembly(),
                include_counter_rows: self.requests_report_assembly(),
                include_supporting_evidence_rows: self.requests_support_expansion(),
                include_budget_decisions: false,
                include_denied_work: false,
                include_widened_work: false,
            });
            foundational_boundaries.push(plan.materialization_boundary());
        }
        let materialization_boundary =
            strongest_materialization_boundary(&foundational_boundaries).into();
        Ok(WorthUiSteadyFrameReportPlan {
            materialization_boundary,
            source_receipt_count: evidence.receipt_count(),
            foundational_boundaries,
        })
    }

    fn requests_report_assembly(self) -> bool {
        matches!(
            self.materialization_boundary,
            WorthUiFrameReportMaterializationBoundary::ReportAssembly
                | WorthUiFrameReportMaterializationBoundary::SupportExpansion
        )
    }

    fn requests_support_expansion(self) -> bool {
        self.materialization_boundary == WorthUiFrameReportMaterializationBoundary::SupportExpansion
    }

    fn foundational_profile(
        self,
    ) -> Result<FoundationalProfileSet, WorthUiSteadyFrameCounterDenial> {
        let (diagnostic_richness, support_posture, certification_posture) =
            match self.materialization_boundary {
                WorthUiFrameReportMaterializationBoundary::ClaimInspectionOnly => (
                    DiagnosticRichnessProfile::OperationalMinimal,
                    SupportPostureProfile::InternalOnly,
                    CertificationPostureProfile::Uncertified,
                ),
                WorthUiFrameReportMaterializationBoundary::ReportAssembly => (
                    DiagnosticRichnessProfile::Standard,
                    SupportPostureProfile::InternalOnly,
                    CertificationPostureProfile::Uncertified,
                ),
                WorthUiFrameReportMaterializationBoundary::SupportExpansion => (
                    DiagnosticRichnessProfile::Standard,
                    SupportPostureProfile::SupportReady,
                    CertificationPostureProfile::EvidenceBacked,
                ),
            };

        profiles()
            .set()
            .diagnostic_richness(diagnostic_richness)
            .support_posture(support_posture)
            .compatibility_posture(CompatibilityPostureProfile::NativeOnly)
            .admission_readiness(AdmissionReadinessProfile::Admitted)
            .retention_delivery(RetentionDeliveryProfile::Retained)
            .certification_posture(certification_posture)
            .compose()
            .map_err(|_| report_planning_denial())
    }
}

impl WorthUiSteadyFrameReportPlan {
    pub fn materialization_boundary(&self) -> WorthUiFrameReportMaterializationBoundary {
        self.materialization_boundary
    }

    pub fn source_receipt_count(&self) -> usize {
        self.source_receipt_count
    }

    pub fn foundational_boundaries(
        &self,
    ) -> &[FoundationalPerformanceReportMaterializationBoundary] {
        &self.foundational_boundaries
    }
}

impl From<WorthUiFrameReportMaterializationBoundary>
    for FoundationalPerformanceReportMaterializationBoundary
{
    fn from(value: WorthUiFrameReportMaterializationBoundary) -> Self {
        match value {
            WorthUiFrameReportMaterializationBoundary::ClaimInspectionOnly => {
                Self::ClaimInspectionOnly
            }
            WorthUiFrameReportMaterializationBoundary::ReportAssembly => Self::ReportAssembly,
            WorthUiFrameReportMaterializationBoundary::SupportExpansion => Self::SupportExpansion,
        }
    }
}

impl From<FoundationalPerformanceReportMaterializationBoundary>
    for WorthUiFrameReportMaterializationBoundary
{
    fn from(value: FoundationalPerformanceReportMaterializationBoundary) -> Self {
        match value {
            FoundationalPerformanceReportMaterializationBoundary::ClaimInspectionOnly => {
                Self::ClaimInspectionOnly
            }
            FoundationalPerformanceReportMaterializationBoundary::ReportAssembly => {
                Self::ReportAssembly
            }
            FoundationalPerformanceReportMaterializationBoundary::SupportExpansion => {
                Self::SupportExpansion
            }
        }
    }
}

fn report_planning_denial() -> WorthUiSteadyFrameCounterDenial {
    WorthUiSteadyFrameCounterDenial::new(
        WorthUiSteadyFrameCounterDenialReason::FoundationalReportPlanning,
    )
}

fn strongest_materialization_boundary(
    boundaries: &[FoundationalPerformanceReportMaterializationBoundary],
) -> FoundationalPerformanceReportMaterializationBoundary {
    if boundaries.contains(&FoundationalPerformanceReportMaterializationBoundary::SupportExpansion)
    {
        FoundationalPerformanceReportMaterializationBoundary::SupportExpansion
    } else if boundaries
        .contains(&FoundationalPerformanceReportMaterializationBoundary::ReportAssembly)
    {
        FoundationalPerformanceReportMaterializationBoundary::ReportAssembly
    } else {
        FoundationalPerformanceReportMaterializationBoundary::ClaimInspectionOnly
    }
}
