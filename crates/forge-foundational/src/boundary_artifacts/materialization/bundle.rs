use std::marker::PhantomData;

use super::bundle_types::{
    FoundationalBoundaryBundleMaterializationCost, FoundationalBoundaryBundleMaterializationDenial,
    FoundationalBoundaryBundlePlanningDenial, FoundationalBoundaryMaterializationBundle,
    FoundationalReceiptBundlePlan, FoundationalSummaryBundlePlan, ReceiptAbsent, ReceiptPresent,
    ReportAbsent, ReportPresent, SummaryAbsent, SummaryPresent,
};
use super::model::{
    FoundationalBoundaryMaterializationDecisionRow, FoundationalBoundaryMaterializationPlan,
};
use super::vocabulary::{
    FoundationalBoundaryDecisionCause, FoundationalBoundaryDecisionSubject,
    FoundationalBoundaryMaterializationSeam,
};
use crate::boundary_artifacts::{
    FoundationalBoundaryArtifactCategory, FoundationalBoundaryArtifactSurface,
    FoundationalBoundaryReportSurface,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryMaterializationBundlePlan<
    Primary,
    ReportRow = (),
    SummaryState = SummaryAbsent,
    ReportState = ReportAbsent,
    ReceiptState = ReceiptAbsent,
> {
    primary: FoundationalBoundaryMaterializationPlan<FoundationalBoundaryArtifactSurface<Primary>>,
    summary: Option<FoundationalSummaryBundlePlan>,
    report: Option<
        FoundationalBoundaryMaterializationPlan<FoundationalBoundaryReportSurface<ReportRow>>,
    >,
    receipt: Option<FoundationalReceiptBundlePlan>,
    membership_rows: Vec<FoundationalBoundaryMaterializationDecisionRow>,
    marker: PhantomData<(SummaryState, ReportState, ReceiptState)>,
}

impl<Primary> FoundationalBoundaryMaterializationBundlePlan<Primary> {
    fn new(
        primary: FoundationalBoundaryMaterializationPlan<
            FoundationalBoundaryArtifactSurface<Primary>,
        >,
    ) -> Self {
        Self {
            primary,
            summary: None,
            report: None,
            receipt: None,
            membership_rows: vec![],
            marker: PhantomData,
        }
    }
}

impl<Primary, ReportRow, SummaryState, ReportState, ReceiptState>
    FoundationalBoundaryMaterializationBundlePlan<
        Primary,
        ReportRow,
        SummaryState,
        ReportState,
        ReceiptState,
    >
{
    pub fn primary(
        &self,
    ) -> &FoundationalBoundaryMaterializationPlan<FoundationalBoundaryArtifactSurface<Primary>>
    {
        &self.primary
    }

    pub const fn source(
        &self,
    ) -> crate::boundary_artifacts::FoundationalBoundaryMaterializationSource {
        self.primary.source()
    }

    pub const fn seam(&self) -> FoundationalBoundaryMaterializationSeam {
        self.primary.seam()
    }

    pub const fn profile(&self) -> &crate::profiles::MaterializedFoundationalProfileSet {
        self.primary.profile()
    }

    pub fn summary(&self) -> Option<&FoundationalSummaryBundlePlan> {
        self.summary.as_ref()
    }

    pub fn report(
        &self,
    ) -> Option<
        &FoundationalBoundaryMaterializationPlan<FoundationalBoundaryReportSurface<ReportRow>>,
    > {
        self.report.as_ref()
    }

    pub fn receipt(&self) -> Option<&FoundationalReceiptBundlePlan> {
        self.receipt.as_ref()
    }

    pub fn membership_decision_rows(&self) -> &[FoundationalBoundaryMaterializationDecisionRow] {
        &self.membership_rows
    }

    pub fn cost(&self) -> FoundationalBoundaryBundleMaterializationCost {
        let mut attachment_count = self.primary.cost().attachment_count();
        let mut included_attachment_count = self.primary.cost().included_attachment_count();
        let mut decision_row_count =
            self.primary.cost().decision_row_count() + self.membership_rows.len() as u32;
        let mut member_count = 1;

        if let Some(summary) = &self.summary {
            attachment_count += summary.cost().attachment_count();
            included_attachment_count += summary.cost().included_attachment_count();
            decision_row_count += summary.cost().decision_row_count();
            member_count += 1;
        }
        if let Some(report) = &self.report {
            attachment_count += report.cost().attachment_count();
            included_attachment_count += report.cost().included_attachment_count();
            decision_row_count += report.cost().decision_row_count();
            member_count += 1;
        }
        if let Some(receipt) = &self.receipt {
            attachment_count += receipt.cost().attachment_count();
            included_attachment_count += receipt.cost().included_attachment_count();
            decision_row_count += receipt.cost().decision_row_count();
            member_count += 1;
        }

        FoundationalBoundaryBundleMaterializationCost::new(
            member_count,
            attachment_count,
            included_attachment_count,
            decision_row_count,
        )
    }

    pub fn materialize(
        self,
    ) -> Result<
        FoundationalBoundaryMaterializationBundle<Primary, ReportRow>,
        FoundationalBoundaryBundleMaterializationDenial,
    > {
        let cost = self.cost();
        let primary = self
            .primary
            .materialize()
            .map_err(FoundationalBoundaryBundleMaterializationDenial::Primary)?;
        let summary = self
            .summary
            .map(|summary| {
                summary
                    .materialize()
                    .map_err(FoundationalBoundaryBundleMaterializationDenial::Summary)
            })
            .transpose()?;
        let report = self
            .report
            .map(|report| {
                report
                    .materialize()
                    .map_err(FoundationalBoundaryBundleMaterializationDenial::Report)
            })
            .transpose()?;
        let receipt = self
            .receipt
            .map(|receipt| {
                receipt
                    .materialize()
                    .map_err(FoundationalBoundaryBundleMaterializationDenial::Receipt)
            })
            .transpose()?;

        Ok(FoundationalBoundaryMaterializationBundle {
            primary,
            summary,
            report,
            receipt,
            membership_rows: self.membership_rows,
            cost,
        })
    }
}

impl<Primary, ReportRow, ReportState, ReceiptState>
    FoundationalBoundaryMaterializationBundlePlan<
        Primary,
        ReportRow,
        SummaryAbsent,
        ReportState,
        ReceiptState,
    >
{
    pub fn with_summary(
        mut self,
        summary: FoundationalSummaryBundlePlan,
    ) -> Result<
        FoundationalBoundaryMaterializationBundlePlan<
            Primary,
            ReportRow,
            SummaryPresent,
            ReportState,
            ReceiptState,
        >,
        FoundationalBoundaryBundlePlanningDenial,
    > {
        validate_bundle_member(
            &self.primary,
            &summary,
            FoundationalBoundaryArtifactCategory::Summary,
        )?;
        self.membership_rows.push(bundle_membership_row(
            FoundationalBoundaryArtifactCategory::Summary,
            summary.seam(),
        ));
        self.summary = Some(summary);
        Ok(FoundationalBoundaryMaterializationBundlePlan {
            primary: self.primary,
            summary: self.summary,
            report: self.report,
            receipt: self.receipt,
            membership_rows: self.membership_rows,
            marker: PhantomData,
        })
    }
}

impl<Primary, SummaryState, ReceiptState>
    FoundationalBoundaryMaterializationBundlePlan<
        Primary,
        (),
        SummaryState,
        ReportAbsent,
        ReceiptState,
    >
{
    pub fn with_report<NextReportRow>(
        mut self,
        report: FoundationalBoundaryMaterializationPlan<
            FoundationalBoundaryReportSurface<NextReportRow>,
        >,
    ) -> Result<
        FoundationalBoundaryMaterializationBundlePlan<
            Primary,
            NextReportRow,
            SummaryState,
            ReportPresent,
            ReceiptState,
        >,
        FoundationalBoundaryBundlePlanningDenial,
    > {
        validate_bundle_member(
            &self.primary,
            &report,
            FoundationalBoundaryArtifactCategory::Report,
        )?;
        self.membership_rows.push(bundle_membership_row(
            FoundationalBoundaryArtifactCategory::Report,
            report.seam(),
        ));
        Ok(FoundationalBoundaryMaterializationBundlePlan {
            primary: self.primary,
            summary: self.summary,
            report: Some(report),
            receipt: self.receipt,
            membership_rows: self.membership_rows,
            marker: PhantomData,
        })
    }
}

impl<Primary, ReportRow, SummaryState, ReportState>
    FoundationalBoundaryMaterializationBundlePlan<
        Primary,
        ReportRow,
        SummaryState,
        ReportState,
        ReceiptAbsent,
    >
{
    pub fn with_receipt(
        mut self,
        receipt: FoundationalReceiptBundlePlan,
    ) -> Result<
        FoundationalBoundaryMaterializationBundlePlan<
            Primary,
            ReportRow,
            SummaryState,
            ReportState,
            ReceiptPresent,
        >,
        FoundationalBoundaryBundlePlanningDenial,
    > {
        validate_bundle_member(
            &self.primary,
            &receipt,
            FoundationalBoundaryArtifactCategory::Receipt,
        )?;
        self.membership_rows.push(bundle_membership_row(
            FoundationalBoundaryArtifactCategory::Receipt,
            receipt.seam(),
        ));
        self.receipt = Some(receipt);
        Ok(FoundationalBoundaryMaterializationBundlePlan {
            primary: self.primary,
            summary: self.summary,
            report: self.report,
            receipt: self.receipt,
            membership_rows: self.membership_rows,
            marker: PhantomData,
        })
    }
}

pub fn plan_artifact_boundary_bundle<Primary>(
    primary: FoundationalBoundaryMaterializationPlan<FoundationalBoundaryArtifactSurface<Primary>>,
) -> FoundationalBoundaryMaterializationBundlePlan<Primary> {
    FoundationalBoundaryMaterializationBundlePlan::new(primary)
}

fn validate_bundle_member<Primary, Surface>(
    primary: &FoundationalBoundaryMaterializationPlan<FoundationalBoundaryArtifactSurface<Primary>>,
    member: &FoundationalBoundaryMaterializationPlan<Surface>,
    member_category: FoundationalBoundaryArtifactCategory,
) -> Result<(), FoundationalBoundaryBundlePlanningDenial> {
    if primary.seam() != member.seam() {
        return Err(match member_category {
            FoundationalBoundaryArtifactCategory::Summary => {
                FoundationalBoundaryBundlePlanningDenial::SummarySeamMismatch
            }
            FoundationalBoundaryArtifactCategory::Report => {
                FoundationalBoundaryBundlePlanningDenial::ReportSeamMismatch
            }
            FoundationalBoundaryArtifactCategory::Receipt => {
                FoundationalBoundaryBundlePlanningDenial::ReceiptSeamMismatch
            }
            FoundationalBoundaryArtifactCategory::Artifact => {
                unreachable!("primary is the artifact")
            }
        });
    }

    if primary.source() != member.source() {
        return Err(match member_category {
            FoundationalBoundaryArtifactCategory::Summary => {
                FoundationalBoundaryBundlePlanningDenial::SummarySourceMismatch
            }
            FoundationalBoundaryArtifactCategory::Report => {
                FoundationalBoundaryBundlePlanningDenial::ReportSourceMismatch
            }
            FoundationalBoundaryArtifactCategory::Receipt => {
                FoundationalBoundaryBundlePlanningDenial::ReceiptSourceMismatch
            }
            FoundationalBoundaryArtifactCategory::Artifact => {
                unreachable!("primary is the artifact")
            }
        });
    }

    if primary.profile() != member.profile() {
        return Err(match member_category {
            FoundationalBoundaryArtifactCategory::Summary => {
                FoundationalBoundaryBundlePlanningDenial::SummaryProfileMismatch
            }
            FoundationalBoundaryArtifactCategory::Report => {
                FoundationalBoundaryBundlePlanningDenial::ReportProfileMismatch
            }
            FoundationalBoundaryArtifactCategory::Receipt => {
                FoundationalBoundaryBundlePlanningDenial::ReceiptProfileMismatch
            }
            FoundationalBoundaryArtifactCategory::Artifact => {
                unreachable!("primary is the artifact")
            }
        });
    }

    Ok(())
}

fn bundle_membership_row(
    category: FoundationalBoundaryArtifactCategory,
    seam: FoundationalBoundaryMaterializationSeam,
) -> FoundationalBoundaryMaterializationDecisionRow {
    FoundationalBoundaryMaterializationDecisionRow::new(
        Some(category),
        FoundationalBoundaryDecisionSubject::BundleMembership,
        FoundationalBoundaryDecisionCause::RequestedAsAdmitted,
        seam,
        None,
    )
}
