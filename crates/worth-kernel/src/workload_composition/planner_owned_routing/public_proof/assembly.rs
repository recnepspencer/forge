use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::architecture_alignment::build_architecture_alignment_report;
use super::assembly_input::WorthTouchedGraphConflictPublicProofAssemblyInput;
use super::assembly_types::WorthTouchedGraphConflictPublicProofAssemblyInputParts;
use super::milestone_fifteen_seed::WorthTouchedGraphConflictMilestoneFifteenSeed;
use super::proof_chain::WorthTouchedGraphConflictProofChain;
use super::types::{
    WorthTouchedGraphConflictPublicCloseout, WorthTouchedGraphConflictPublicCloseoutError,
    WorthTouchedGraphConflictPublicCloseoutErrorKind,
};
use crate::workload_composition::performance_trace::trace_scope;
use crate::workload_composition::planner_owned_routing::WorthWorkloadOrdinaryConsumerCutover;
use crate::workload_composition::{
    WorthTouchedGraphConflictAdmittedPublicProofInput, WorthTouchedGraphConflictSelectedRoutePacket,
};

pub(crate) fn assemble_public_closeout_from_parts(
    input: WorthTouchedGraphConflictPublicProofAssemblyInputParts<'_>,
    cutover: &WorthWorkloadOrdinaryConsumerCutover,
    selected_route_packet: &WorthTouchedGraphConflictSelectedRoutePacket,
    admitted_public_proof_input: &WorthTouchedGraphConflictAdmittedPublicProofInput,
) -> Result<WorthTouchedGraphConflictPublicCloseout, WorthTouchedGraphConflictPublicCloseoutError> {
    trace_scope("assemble_public_closeout_from_parts", || {
        let assembly_input = trace_scope("public_closeout_assembly_input", || {
            WorthTouchedGraphConflictPublicProofAssemblyInput::new(
                input,
                cutover,
                selected_route_packet,
                admitted_public_proof_input,
            )
        })?;
        let residue_chain = trace_scope("public_closeout_residue_chain", || {
            assembly_input.residue_chain()
        });
        let input = assembly_input.closeout_input();
        if input.source_firewall_report().violation_count() != 0 {
            return Err(WorthTouchedGraphConflictPublicCloseoutError::new(
                WorthTouchedGraphConflictPublicCloseoutErrorKind::SourceFirewallViolation,
                "public closeout requires a clean touched-graph conflict source firewall",
            ));
        }
        if residue_chain.ordinary_dependency_count() != 0 {
            return Err(WorthTouchedGraphConflictPublicCloseoutError::new(
                WorthTouchedGraphConflictPublicCloseoutErrorKind::OrdinaryConsumerDependencyStillOpen,
                "public closeout cannot publish while a covered ordinary consumer still requires a second ontology for conflict, serialization, sameness, reuse, or semantic validity",
            ));
        }
        let proof_chain = trace_scope("public_closeout_proof_chain", || {
            WorthTouchedGraphConflictProofChain::from_selected_route_packet(
                assembly_input.selected_route_packet(),
            )
        });
        if proof_chain.selected_conflict_plan_digests().is_empty()
            || proof_chain.overlap_identity_digests().is_empty()
            || proof_chain.locality_footprint_digests().is_empty()
            || proof_chain.independence_proof_digests().is_empty()
            || proof_chain
                .evidence_lookup_public_closeout_digest()
                .is_empty()
            || proof_chain
                .evidence_lookup_query_boundary_support_digest()
                .is_empty()
            || proof_chain
                .topology_query_backed_consumer_cutover_digest()
                .is_empty()
        {
            return Err(WorthTouchedGraphConflictPublicCloseoutError::new(
                WorthTouchedGraphConflictPublicCloseoutErrorKind::IncompleteProofChain,
                "public closeout requires selected conflict plans, overlap identity, locality proof, independence proof, and planner-owned evidence lookup route proof markers to survive into the execution receipt chain",
            ));
        }
        let replay_undo_cutover_witness_count = assembly_input
            .cutover()
            .replay_undo_selected_plan_witness_count();
        if replay_undo_cutover_witness_count
            != proof_chain.replay_undo_boundary_proof_digests().len()
            || replay_undo_cutover_witness_count
                != proof_chain.transaction_packet_identities().len()
            || replay_undo_cutover_witness_count != proof_chain.replay_scope_identities().len()
            || replay_undo_cutover_witness_count != proof_chain.undo_scope_identities().len()
        {
            return Err(WorthTouchedGraphConflictPublicCloseoutError::new(
                WorthTouchedGraphConflictPublicCloseoutErrorKind::IncompleteProofChain,
                "public closeout requires replay/undo admitted-boundary proof identities to survive from cutover into the published chain",
            ));
        }
        let architecture_alignment_report =
            trace_scope("public_closeout_architecture_alignment_report", || {
                build_architecture_alignment_report(
                    input.deletion_closeout(),
                    &residue_chain,
                    assembly_input.selected_route_packet(),
                )
            })?;
        let source_firewall_digest = input.source_firewall_report().report_digest().to_string();
        let deletion_closeout_digest = input.deletion_closeout().closeout_digest().to_string();
        let milestone_fifteen_seed = trace_scope("public_closeout_milestone_fifteen_seed", || {
            WorthTouchedGraphConflictMilestoneFifteenSeed::from_selected_route_packet(
                assembly_input.selected_route_packet(),
                residue_chain.residue_digest(),
                &source_firewall_digest,
                assembly_input.admitted_public_proof_input().clone(),
            )
        })?;
        let closeout_digest = trace_scope("public_closeout_digest", || {
            truth_digest_parts(
                TruthDigestScope::ArtifactIdentity,
                &[
                    "worth-kernel:touched-graph-conflict-public-closeout:v1".to_string(),
                    format!("proof-chain:{}", proof_chain.proof_chain_digest()),
                    format!("residue:{}", residue_chain.residue_digest()),
                    format!(
                        "architecture:{}",
                        architecture_alignment_report.report_digest()
                    ),
                    format!("firewall:{source_firewall_digest}"),
                    format!("deletion:{deletion_closeout_digest}"),
                    format!("seed:{}", milestone_fifteen_seed.seed_digest()),
                ],
            )
        });
        Ok(WorthTouchedGraphConflictPublicCloseout {
            proof_chain,
            residue_chain,
            architecture_alignment_report,
            source_firewall_digest,
            deletion_closeout_digest,
            milestone_fifteen_seed,
            closeout_digest,
        })
    })
}
