use crate::workload_composition::planner_owned_routing::WorthWorkloadOrdinaryConsumerCutover;
use crate::workload_composition::{
    WorthTouchedGraphConflictAdmittedPublicProofInput, WorthTouchedGraphConflictDeletionCloseout,
    WorthTouchedGraphConflictSelectedRoutePacket,
    WorthTouchedGraphConflictSourceFirewallReport,
};
#[cfg(test)]
use crate::workload_composition::WorthTouchedGraphConflictResidueChain;

use super::types::{
    WorthTouchedGraphConflictPublicCloseoutError, WorthTouchedGraphConflictPublicCloseoutErrorKind,
};

pub(crate) struct WorthTouchedGraphConflictPublicProofAssemblyInputParts<'a> {
    deletion_closeout: &'a WorthTouchedGraphConflictDeletionCloseout,
    source_firewall_report: &'a WorthTouchedGraphConflictSourceFirewallReport,
}

pub(crate) struct CurrentWorthTouchedGraphConflictPublicProofAssemblyComponents {
    cutover: WorthWorkloadOrdinaryConsumerCutover,
    deletion_closeout: WorthTouchedGraphConflictDeletionCloseout,
    source_firewall_report: WorthTouchedGraphConflictSourceFirewallReport,
    selected_route_packet: WorthTouchedGraphConflictSelectedRoutePacket,
    admitted_public_proof_input: WorthTouchedGraphConflictAdmittedPublicProofInput,
}

impl<'a> WorthTouchedGraphConflictPublicProofAssemblyInputParts<'a> {
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

impl CurrentWorthTouchedGraphConflictPublicProofAssemblyComponents {
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

    #[cfg(test)]
    pub(crate) fn residue_chain(&self) -> WorthTouchedGraphConflictResidueChain {
        WorthTouchedGraphConflictResidueChain::from_current_live_surfaces(self.cutover.rows())
    }

    pub(crate) fn input(
        &self,
    ) -> Result<
        WorthTouchedGraphConflictPublicProofAssemblyInputParts<'_>,
        WorthTouchedGraphConflictPublicCloseoutError,
    > {
        WorthTouchedGraphConflictPublicProofAssemblyInputParts::new(
            &self.deletion_closeout,
            &self.source_firewall_report,
        )
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
