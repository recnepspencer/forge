use crate::workload_composition::planner_owned_routing::public_proof::require_admitted_public_proof_input_matches_selected_route_packet;
use crate::workload_composition::worth_workload::WorthWorkloadOrdinaryConsumerCutover;
use crate::workload_composition::{
    WorthTouchedGraphConflictAdmittedPublicProofInput, WorthTouchedGraphConflictResidueChain,
    WorthTouchedGraphConflictSelectedRoutePacket,
};

use super::assembly_types::WorthTouchedGraphConflictPublicProofAssemblyInputParts;
use super::types::{
    WorthTouchedGraphConflictPublicCloseoutError, WorthTouchedGraphConflictPublicCloseoutErrorKind,
};

pub(crate) struct WorthTouchedGraphConflictPublicProofAssemblyInput<'a> {
    closeout_input: WorthTouchedGraphConflictPublicProofAssemblyInputParts<'a>,
    cutover: &'a WorthWorkloadOrdinaryConsumerCutover,
    selected_route_packet: &'a WorthTouchedGraphConflictSelectedRoutePacket,
    admitted_public_proof_input: &'a WorthTouchedGraphConflictAdmittedPublicProofInput,
}

impl<'a> WorthTouchedGraphConflictPublicProofAssemblyInput<'a> {
    pub(crate) fn new(
        closeout_input: WorthTouchedGraphConflictPublicProofAssemblyInputParts<'a>,
        cutover: &'a WorthWorkloadOrdinaryConsumerCutover,
        selected_route_packet: &'a WorthTouchedGraphConflictSelectedRoutePacket,
        admitted_public_proof_input: &'a WorthTouchedGraphConflictAdmittedPublicProofInput,
    ) -> Result<Self, WorthTouchedGraphConflictPublicCloseoutError> {
        require_admitted_public_proof_input_matches_selected_route_packet(
            selected_route_packet,
            admitted_public_proof_input,
        )?;
        if closeout_input.source_firewall_report().report_digest()
            != selected_route_packet.source_firewall_digest()
            || closeout_input.deletion_closeout().closeout_digest()
                != selected_route_packet.deletion_closeout_digest()
        {
            return Err(WorthTouchedGraphConflictPublicCloseoutError::new(
                WorthTouchedGraphConflictPublicCloseoutErrorKind::MismatchedFirewallProof,
                "public proof assembly requires selected-route packet, deletion closeout, and source firewall proof from the same planner-owned chain",
            ));
        }
        if cutover.batch_execution_receipt().execution_receipt_digest()
            != selected_route_packet.batch_execution_receipt_digest()
            || cutover.replay_undo_boundary_proof_digests()
                != selected_route_packet
                    .lower_proof_chain_inputs()
                    .replay_undo_boundary_proof_digests
            || cutover.transaction_packet_identities()
                != selected_route_packet
                    .lower_proof_chain_inputs()
                    .transaction_packet_identities
            || cutover.replay_scope_identities()
                != selected_route_packet
                    .lower_proof_chain_inputs()
                    .replay_scope_identities
            || cutover.undo_scope_identities()
                != selected_route_packet
                    .lower_proof_chain_inputs()
                    .undo_scope_identities
        {
            return Err(WorthTouchedGraphConflictPublicCloseoutError::new(
                WorthTouchedGraphConflictPublicCloseoutErrorKind::IncompleteProofChain,
                "public proof assembly requires selected-route packet and cutover proof to bind the same batch and replay/undo authority chain",
            ));
        }
        Ok(Self {
            closeout_input,
            cutover,
            selected_route_packet,
            admitted_public_proof_input,
        })
    }

    pub(crate) fn closeout_input(
        &self,
    ) -> &WorthTouchedGraphConflictPublicProofAssemblyInputParts<'a> {
        &self.closeout_input
    }

    pub(crate) fn cutover(&self) -> &WorthWorkloadOrdinaryConsumerCutover {
        self.cutover
    }

    pub(crate) fn residue_chain(&self) -> WorthTouchedGraphConflictResidueChain {
        WorthTouchedGraphConflictResidueChain::from_current_live_surfaces(self.cutover.rows())
    }

    pub(crate) fn selected_route_packet(&self) -> &WorthTouchedGraphConflictSelectedRoutePacket {
        self.selected_route_packet
    }

    pub(crate) fn admitted_public_proof_input(
        &self,
    ) -> &WorthTouchedGraphConflictAdmittedPublicProofInput {
        self.admitted_public_proof_input
    }
}
