use super::architecture_alignment_report::WorthTouchedGraphConflictArchitectureAlignmentReport;
use super::milestone_fifteen_seed::WorthTouchedGraphConflictMilestoneFifteenSeed;
use super::proof_chain::WorthTouchedGraphConflictProofChain;
use super::residue_chain::WorthTouchedGraphConflictResidueChain;
use crate::workload_composition::worth_workload::WorthWorkloadOrdinaryConsumerCutover;
use crate::workload_composition::{
    WorthTouchedGraphConflictAdmittedPublicProofInput, WorthTouchedGraphConflictDeletionCloseout,
    WorthTouchedGraphConflictSelectedRoutePacket, WorthTouchedGraphConflictSourceFirewallReport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTouchedGraphConflictPublicCloseoutErrorKind {
    CurrentProofUnavailable,
    SourceFirewallViolation,
    MismatchedFirewallProof,
    OrdinaryConsumerDependencyStillOpen,
    IncompleteProofChain,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictPublicCloseoutError {
    kind: WorthTouchedGraphConflictPublicCloseoutErrorKind,
    detail: String,
}

pub(crate) struct WorthTouchedGraphConflictPublicCloseoutInput<'a> {
    deletion_closeout: &'a WorthTouchedGraphConflictDeletionCloseout,
    source_firewall_report: &'a WorthTouchedGraphConflictSourceFirewallReport,
}

pub(crate) struct CurrentWorthTouchedGraphConflictPublicCloseoutComponents {
    cutover: WorthWorkloadOrdinaryConsumerCutover,
    deletion_closeout: WorthTouchedGraphConflictDeletionCloseout,
    source_firewall_report: WorthTouchedGraphConflictSourceFirewallReport,
    selected_route_packet: WorthTouchedGraphConflictSelectedRoutePacket,
    admitted_public_proof_input: WorthTouchedGraphConflictAdmittedPublicProofInput,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictPublicCloseout {
    pub(crate) proof_chain: WorthTouchedGraphConflictProofChain,
    pub(crate) residue_chain: WorthTouchedGraphConflictResidueChain,
    pub(crate) architecture_alignment_report: WorthTouchedGraphConflictArchitectureAlignmentReport,
    pub(crate) source_firewall_digest: String,
    pub(crate) deletion_closeout_digest: String,
    pub(crate) milestone_fifteen_seed: WorthTouchedGraphConflictMilestoneFifteenSeed,
    pub(crate) closeout_digest: String,
}

impl<'a> WorthTouchedGraphConflictPublicCloseoutInput<'a> {
    pub(crate) fn new(
        deletion_closeout: &'a WorthTouchedGraphConflictDeletionCloseout,
        source_firewall_report: &'a WorthTouchedGraphConflictSourceFirewallReport,
    ) -> Result<Self, WorthTouchedGraphConflictPublicCloseoutError> {
        if deletion_closeout.source_firewall_report_digest()
            != source_firewall_report.report_digest()
        {
            return Err(WorthTouchedGraphConflictPublicCloseoutError::new(
                WorthTouchedGraphConflictPublicCloseoutErrorKind::MismatchedFirewallProof,
                "public closeout requires one deletion closeout and source firewall report from the same proof chain",
            ));
        }
        Ok(Self {
            deletion_closeout,
            source_firewall_report,
        })
    }

    pub(crate) const fn deletion_closeout(&self) -> &'a WorthTouchedGraphConflictDeletionCloseout {
        self.deletion_closeout
    }

    pub(crate) const fn source_firewall_report(
        &self,
    ) -> &'a WorthTouchedGraphConflictSourceFirewallReport {
        self.source_firewall_report
    }
}

impl CurrentWorthTouchedGraphConflictPublicCloseoutComponents {
    pub(crate) fn new(
        cutover: WorthWorkloadOrdinaryConsumerCutover,
        deletion_closeout: WorthTouchedGraphConflictDeletionCloseout,
        source_firewall_report: WorthTouchedGraphConflictSourceFirewallReport,
        selected_route_packet: WorthTouchedGraphConflictSelectedRoutePacket,
        admitted_public_proof_input: WorthTouchedGraphConflictAdmittedPublicProofInput,
    ) -> Self {
        Self {
            cutover,
            deletion_closeout,
            source_firewall_report,
            selected_route_packet,
            admitted_public_proof_input,
        }
    }

    pub(crate) fn cutover(&self) -> &WorthWorkloadOrdinaryConsumerCutover {
        &self.cutover
    }

    pub(crate) fn input(
        &self,
    ) -> Result<
        WorthTouchedGraphConflictPublicCloseoutInput<'_>,
        WorthTouchedGraphConflictPublicCloseoutError,
    > {
        WorthTouchedGraphConflictPublicCloseoutInput::new(
            &self.deletion_closeout,
            &self.source_firewall_report,
        )
    }

    pub(crate) fn residue_chain(&self) -> WorthTouchedGraphConflictResidueChain {
        WorthTouchedGraphConflictResidueChain::from_cutover_rows(self.cutover.rows())
    }

    pub(crate) fn selected_route_packet(&self) -> &WorthTouchedGraphConflictSelectedRoutePacket {
        &self.selected_route_packet
    }

    pub(crate) fn admitted_public_proof_input(
        &self,
    ) -> &WorthTouchedGraphConflictAdmittedPublicProofInput {
        &self.admitted_public_proof_input
    }
}

impl WorthTouchedGraphConflictPublicCloseout {
    pub fn current() -> Result<Self, WorthTouchedGraphConflictPublicCloseoutError> {
        super::public_closeout::current_worth_touched_graph_conflict_public_closeout()
    }

    pub fn proof_chain(&self) -> &WorthTouchedGraphConflictProofChain {
        &self.proof_chain
    }

    pub fn residue_chain(&self) -> &WorthTouchedGraphConflictResidueChain {
        &self.residue_chain
    }

    pub fn architecture_alignment_report(
        &self,
    ) -> &WorthTouchedGraphConflictArchitectureAlignmentReport {
        &self.architecture_alignment_report
    }

    pub fn source_firewall_digest(&self) -> &str {
        &self.source_firewall_digest
    }

    pub fn deletion_closeout_digest(&self) -> &str {
        &self.deletion_closeout_digest
    }

    pub fn milestone_fifteen_seed(&self) -> &WorthTouchedGraphConflictMilestoneFifteenSeed {
        &self.milestone_fifteen_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}

impl WorthTouchedGraphConflictPublicCloseoutError {
    pub(crate) fn new(
        kind: WorthTouchedGraphConflictPublicCloseoutErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> WorthTouchedGraphConflictPublicCloseoutErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
