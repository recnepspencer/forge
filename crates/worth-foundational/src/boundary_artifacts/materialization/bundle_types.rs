use super::model::{
    FoundationalBoundaryMaterializationDecisionRow, FoundationalBoundaryMaterializationDenial,
    FoundationalBoundaryMaterializationPlan, FoundationalMaterializedBoundaryArtifact,
};
use crate::boundary_artifacts::{
    FoundationalBoundaryArtifactSurface, FoundationalBoundaryReceiptSurface,
    FoundationalBoundaryReportSurface, FoundationalBoundarySummarySurface,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryBundleMaterializationCost {
    member_count: u32,
    attachment_count: u32,
    included_attachment_count: u32,
    decision_row_count: u32,
}

impl FoundationalBoundaryBundleMaterializationCost {
    pub(crate) const fn new(
        member_count: u32,
        attachment_count: u32,
        included_attachment_count: u32,
        decision_row_count: u32,
    ) -> Self {
        Self {
            member_count,
            attachment_count,
            included_attachment_count,
            decision_row_count,
        }
    }

    pub const fn member_count(&self) -> u32 {
        self.member_count
    }

    pub const fn attachment_count(&self) -> u32 {
        self.attachment_count
    }

    pub const fn included_attachment_count(&self) -> u32 {
        self.included_attachment_count
    }

    pub const fn decision_row_count(&self) -> u32 {
        self.decision_row_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalBoundaryBundlePlanningDenial {
    SummarySeamMismatch,
    SummarySourceMismatch,
    SummaryProfileMismatch,
    ReportSeamMismatch,
    ReportSourceMismatch,
    ReportProfileMismatch,
    ReceiptSeamMismatch,
    ReceiptSourceMismatch,
    ReceiptProfileMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalBoundaryBundleMaterializationDenial {
    Primary(FoundationalBoundaryMaterializationDenial),
    Summary(FoundationalBoundaryMaterializationDenial),
    Report(FoundationalBoundaryMaterializationDenial),
    Receipt(FoundationalBoundaryMaterializationDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SummaryAbsent;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SummaryPresent;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReportAbsent;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReportPresent;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReceiptAbsent;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReceiptPresent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryMaterializationBundle<Primary, ReportRow = ()> {
    pub(crate) primary:
        FoundationalMaterializedBoundaryArtifact<FoundationalBoundaryArtifactSurface<Primary>>,
    pub(crate) summary:
        Option<FoundationalMaterializedBoundaryArtifact<FoundationalBoundarySummarySurface>>,
    pub(crate) report: Option<
        FoundationalMaterializedBoundaryArtifact<FoundationalBoundaryReportSurface<ReportRow>>,
    >,
    pub(crate) receipt:
        Option<FoundationalMaterializedBoundaryArtifact<FoundationalBoundaryReceiptSurface>>,
    pub(crate) membership_rows: Vec<FoundationalBoundaryMaterializationDecisionRow>,
    pub(crate) cost: FoundationalBoundaryBundleMaterializationCost,
}

impl<Primary, ReportRow> FoundationalBoundaryMaterializationBundle<Primary, ReportRow> {
    pub fn primary(
        &self,
    ) -> &FoundationalMaterializedBoundaryArtifact<FoundationalBoundaryArtifactSurface<Primary>>
    {
        &self.primary
    }

    pub const fn source(
        &self,
    ) -> crate::boundary_artifacts::FoundationalBoundaryMaterializationSource {
        self.primary.source()
    }

    pub const fn seam(&self) -> crate::boundary_artifacts::FoundationalBoundaryMaterializationSeam {
        self.primary.seam()
    }

    pub const fn profile(&self) -> &crate::profiles::MaterializedFoundationalProfileSet {
        self.primary.profile()
    }

    pub fn summary(
        &self,
    ) -> Option<&FoundationalMaterializedBoundaryArtifact<FoundationalBoundarySummarySurface>> {
        self.summary.as_ref()
    }

    pub fn report(
        &self,
    ) -> Option<
        &FoundationalMaterializedBoundaryArtifact<FoundationalBoundaryReportSurface<ReportRow>>,
    > {
        self.report.as_ref()
    }

    pub fn receipt(
        &self,
    ) -> Option<&FoundationalMaterializedBoundaryArtifact<FoundationalBoundaryReceiptSurface>> {
        self.receipt.as_ref()
    }

    pub fn membership_decision_rows(&self) -> &[FoundationalBoundaryMaterializationDecisionRow] {
        &self.membership_rows
    }

    pub const fn cost(&self) -> FoundationalBoundaryBundleMaterializationCost {
        self.cost
    }
}

pub(crate) type FoundationalSummaryBundlePlan =
    FoundationalBoundaryMaterializationPlan<FoundationalBoundarySummarySurface>;
pub(crate) type FoundationalReceiptBundlePlan =
    FoundationalBoundaryMaterializationPlan<FoundationalBoundaryReceiptSurface>;
