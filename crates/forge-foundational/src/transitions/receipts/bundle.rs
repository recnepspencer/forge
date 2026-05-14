use std::marker::PhantomData;

use forge_proof::AuthorityWitness;

use super::issuance::{
    derive_transition_report_row, issue_transition_receipt_from_committed,
    FoundationalCommitReceiptArtifact, FoundationalCommitReceiptIssuance,
};
use super::vocabulary::{
    build_summary_claim, FoundationalCommitId, FoundationalCommitReceiptIdentity,
    FoundationalCommitReceiptIssuanceDenial, FoundationalTransitionProvenanceRow,
};
use crate::boundary_artifacts::{
    claim_support_only_boundary_surface, FoundationalBoundaryReportSurface,
    FoundationalBoundarySummarySurface, FoundationalSupportOnlyBoundaryClaim,
};
use crate::transitions::FoundationalCommittedAuthorityArtifact;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalTransitionBundleMaterializationCost {
    member_count: u32,
    provenance_row_count: u32,
    attested_delta_count: u32,
}

impl FoundationalTransitionBundleMaterializationCost {
    const fn new(member_count: u32, provenance_row_count: u32, attested_delta_count: u32) -> Self {
        Self {
            member_count,
            provenance_row_count,
            attested_delta_count,
        }
    }

    pub const fn member_count(&self) -> u32 {
        self.member_count
    }

    pub const fn provenance_row_count(&self) -> u32 {
        self.provenance_row_count
    }

    pub const fn attested_delta_count(&self) -> u32 {
        self.attested_delta_count
    }
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

pub struct FoundationalTransitionBundleBuilder<
    T,
    SummaryState = SummaryAbsent,
    ReportState = ReportAbsent,
    ReceiptState = ReceiptAbsent,
> {
    committed: FoundationalCommittedAuthorityArtifact<T>,
    include_summary: bool,
    include_report: bool,
    receipt: Option<(
        FoundationalCommitReceiptIdentity,
        FoundationalCommitId,
        AuthorityWitness<FoundationalCommitReceiptIssuance>,
    )>,
    marker: PhantomData<(SummaryState, ReportState, ReceiptState)>,
}

impl<T> FoundationalTransitionBundleBuilder<T> {
    pub(crate) fn new(committed: FoundationalCommittedAuthorityArtifact<T>) -> Self {
        Self {
            committed,
            include_summary: false,
            include_report: false,
            receipt: None,
            marker: PhantomData,
        }
    }
}

impl<T, ReportState, ReceiptState>
    FoundationalTransitionBundleBuilder<T, SummaryAbsent, ReportState, ReceiptState>
{
    pub fn with_summary(
        self,
    ) -> FoundationalTransitionBundleBuilder<T, SummaryPresent, ReportState, ReceiptState> {
        FoundationalTransitionBundleBuilder {
            committed: self.committed,
            include_summary: true,
            include_report: self.include_report,
            receipt: self.receipt,
            marker: PhantomData,
        }
    }
}

impl<T, SummaryState, ReceiptState>
    FoundationalTransitionBundleBuilder<T, SummaryState, ReportAbsent, ReceiptState>
{
    pub fn with_merge_report(
        self,
    ) -> FoundationalTransitionBundleBuilder<T, SummaryState, ReportPresent, ReceiptState> {
        FoundationalTransitionBundleBuilder {
            committed: self.committed,
            include_summary: self.include_summary,
            include_report: true,
            receipt: self.receipt,
            marker: PhantomData,
        }
    }
}

impl<T, SummaryState, ReportState>
    FoundationalTransitionBundleBuilder<T, SummaryState, ReportState, ReceiptAbsent>
{
    pub fn with_receipt(
        self,
        receipt_identity: FoundationalCommitReceiptIdentity,
        commit_id: FoundationalCommitId,
        authority: AuthorityWitness<FoundationalCommitReceiptIssuance>,
    ) -> FoundationalTransitionBundleBuilder<T, SummaryState, ReportState, ReceiptPresent> {
        FoundationalTransitionBundleBuilder {
            committed: self.committed,
            include_summary: self.include_summary,
            include_report: self.include_report,
            receipt: Some((receipt_identity, commit_id, authority)),
            marker: PhantomData,
        }
    }
}

pub struct FoundationalTransitionBundle<T> {
    primary: FoundationalCommittedAuthorityArtifact<T>,
    summary: Option<FoundationalSupportOnlyBoundaryClaim<FoundationalBoundarySummarySurface>>,
    merge_report: Option<
        FoundationalSupportOnlyBoundaryClaim<
            FoundationalBoundaryReportSurface<FoundationalTransitionProvenanceRow>,
        >,
    >,
    receipt: Option<FoundationalCommitReceiptArtifact>,
    cost: FoundationalTransitionBundleMaterializationCost,
}

impl<T, SummaryState, ReportState, ReceiptState>
    FoundationalTransitionBundleBuilder<T, SummaryState, ReportState, ReceiptState>
{
    pub fn materialize(
        self,
    ) -> Result<FoundationalTransitionBundle<T>, FoundationalCommitReceiptIssuanceDenial> {
        let summary = if self.include_summary {
            Some(build_summary_claim(
                self.committed.target_branch(),
                self.committed.transition_class(),
                self.committed.committed_delta_summary(),
            )?)
        } else {
            None
        };

        let receipt = self
            .receipt
            .map(|(receipt_identity, commit_id, authority)| {
                issue_transition_receipt_from_committed(
                    &self.committed,
                    receipt_identity,
                    commit_id,
                    authority,
                )
            })
            .transpose()?;

        let merge_report = if self.include_report {
            let row = if let Some(receipt) = &receipt {
                receipt.transition_provenance_rows()[0].clone()
            } else {
                derive_transition_report_row(&self.committed)
            };
            let surface = FoundationalBoundaryReportSurface::new(vec![row], 1)
                .map_err(FoundationalCommitReceiptIssuanceDenial::ReceiptSurface)?;
            Some(claim_support_only_boundary_surface(surface))
        } else {
            None
        };

        let member_count = 1
            + u32::from(summary.is_some())
            + u32::from(merge_report.is_some())
            + u32::from(receipt.is_some());
        let provenance_row_count = merge_report
            .as_ref()
            .map_or(0, |report| report.surface().rows().len() as u32)
            + receipt
                .as_ref()
                .map_or(0, |issued| issued.transition_provenance_rows().len() as u32);
        let attested_delta_count = self.committed.committed_delta_summary().delta_count() as u32;

        Ok(FoundationalTransitionBundle {
            primary: self.committed,
            summary,
            merge_report,
            receipt,
            cost: FoundationalTransitionBundleMaterializationCost::new(
                member_count,
                provenance_row_count,
                attested_delta_count,
            ),
        })
    }
}

impl<T> FoundationalTransitionBundle<T> {
    pub fn primary(&self) -> &FoundationalCommittedAuthorityArtifact<T> {
        &self.primary
    }

    pub fn summary(
        &self,
    ) -> Option<&FoundationalSupportOnlyBoundaryClaim<FoundationalBoundarySummarySurface>> {
        self.summary.as_ref()
    }

    pub fn merge_report(
        &self,
    ) -> Option<
        &FoundationalSupportOnlyBoundaryClaim<
            FoundationalBoundaryReportSurface<FoundationalTransitionProvenanceRow>,
        >,
    > {
        self.merge_report.as_ref()
    }

    pub fn receipt(&self) -> Option<&FoundationalCommitReceiptArtifact> {
        self.receipt.as_ref()
    }

    pub fn transition_class(&self) -> crate::transitions::FoundationalAuthorityTransitionClass {
        self.primary.transition_class()
    }

    pub fn strategy_identity(&self) -> &crate::transitions::FoundationalTransitionStrategyIdentity {
        self.primary.strategy_identity()
    }

    pub fn transition_basis_identity(
        &self,
    ) -> crate::transitions::FoundationalTransitionBasisIdentity {
        self.primary.transition_basis_identity()
    }

    pub const fn materialization_cost(&self) -> FoundationalTransitionBundleMaterializationCost {
        self.cost
    }
}
