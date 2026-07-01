use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::architecture_alignment_report::build_architecture_alignment_report;
use super::milestone_fifteen_seed::WorthTouchedGraphConflictMilestoneFifteenSeed;
use super::proof_chain::WorthTouchedGraphConflictProofChain;
use super::public_closeout_types::{
    CurrentWorthTouchedGraphConflictPublicCloseoutComponents,
    WorthTouchedGraphConflictPublicCloseout, WorthTouchedGraphConflictPublicCloseoutError,
    WorthTouchedGraphConflictPublicCloseoutErrorKind, WorthTouchedGraphConflictPublicCloseoutInput,
};
use super::residue_chain::WorthTouchedGraphConflictResidueChain;
use crate::workload_composition::worth_workload::{
    current_worth_workload_ordinary_consumer_cutover, WorthWorkloadOrdinaryConsumerCutover,
};
use crate::workload_composition::{
    current_kernel_compiled_product_consumer_dependency_matrix,
    admit_worth_touched_graph_conflict_public_proof_input,
    current_worth_touched_graph_conflict_deletion_closeout,
    current_worth_touched_graph_conflict_source_firewall_report,
    KernelCompiledProductConsumerDependencyError, KernelCompiledProductConsumerDependencyMatrix,
};
#[cfg(test)]
use crate::workload_composition::compiled_product_consumer_cutover::{
    current_kernel_compiled_product_consumer_dependency_matrix_with_targets_loader,
    KernelCompiledProductConsumerCoverageTarget,
};

pub fn current_worth_touched_graph_conflict_public_closeout(
) -> Result<WorthTouchedGraphConflictPublicCloseout, WorthTouchedGraphConflictPublicCloseoutError> {
    let components = current_public_closeout_components()?;
    publish_from_parts(
        components.input()?,
        components.cutover(),
        components.residue_chain(),
        components.selected_route_packet(),
        components.admitted_public_proof_input(),
    )
}

pub fn current_worth_touched_graph_conflict_milestone_fifteen_seed() -> Result<
    WorthTouchedGraphConflictMilestoneFifteenSeed,
    WorthTouchedGraphConflictPublicCloseoutError,
> {
    Ok(current_worth_touched_graph_conflict_public_closeout()?
        .milestone_fifteen_seed()
        .clone())
}

pub(crate) fn publish_from_parts(
    input: WorthTouchedGraphConflictPublicCloseoutInput<'_>,
    cutover: &WorthWorkloadOrdinaryConsumerCutover,
    residue_chain: WorthTouchedGraphConflictResidueChain,
    selected_route_packet: &crate::workload_composition::WorthTouchedGraphConflictSelectedRoutePacket,
    admitted_public_proof_input: &crate::workload_composition::WorthTouchedGraphConflictAdmittedPublicProofInput,
) -> Result<WorthTouchedGraphConflictPublicCloseout, WorthTouchedGraphConflictPublicCloseoutError> {
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
    let proof_chain = WorthTouchedGraphConflictProofChain::from_selected_route_packet(
        selected_route_packet,
    );
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
            "public closeout requires selected conflict plans, overlap identity, locality proof, independence proof, and query-backed evidence lookup public closeout proof to survive into the execution receipt chain",
        ));
    }
    let replay_undo_cutover_witness_count = cutover.replay_undo_selected_plan_witness_count();
    if replay_undo_cutover_witness_count != proof_chain.replay_undo_boundary_proof_digests().len()
        || replay_undo_cutover_witness_count != proof_chain.transaction_packet_identities().len()
        || replay_undo_cutover_witness_count != proof_chain.replay_scope_identities().len()
        || replay_undo_cutover_witness_count != proof_chain.undo_scope_identities().len()
    {
        return Err(WorthTouchedGraphConflictPublicCloseoutError::new(
            WorthTouchedGraphConflictPublicCloseoutErrorKind::IncompleteProofChain,
            "public closeout requires replay/undo admitted-boundary proof identities to survive from cutover into the published chain",
        ));
    }
    let architecture_alignment_report =
        build_architecture_alignment_report(input.deletion_closeout(), &residue_chain)?;
    let source_firewall_digest = input.source_firewall_report().report_digest().to_string();
    let deletion_closeout_digest = input.deletion_closeout().closeout_digest().to_string();
    let milestone_fifteen_seed = WorthTouchedGraphConflictMilestoneFifteenSeed::from_selected_route_packet(
        selected_route_packet,
        residue_chain.residue_digest(),
        &source_firewall_digest,
        admitted_public_proof_input.clone(),
    )?;
    let closeout_digest = truth_digest_parts(
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
    );
    Ok(WorthTouchedGraphConflictPublicCloseout {
        proof_chain,
        residue_chain,
        architecture_alignment_report,
        source_firewall_digest,
        deletion_closeout_digest,
        milestone_fifteen_seed,
        closeout_digest,
    })
}

fn map_planner_owned_routing_error(
    error: crate::workload_composition::PlannerOwnedRoutingError,
) -> WorthTouchedGraphConflictPublicCloseoutError {
    WorthTouchedGraphConflictPublicCloseoutError::new(
        WorthTouchedGraphConflictPublicCloseoutErrorKind::CurrentProofUnavailable,
        error.detail(),
    )
}

pub(crate) fn current_public_closeout_components() -> Result<
    CurrentWorthTouchedGraphConflictPublicCloseoutComponents,
    WorthTouchedGraphConflictPublicCloseoutError,
> {
    current_public_closeout_components_with_matrix_loader(
        current_kernel_compiled_product_consumer_dependency_matrix,
    )
}

pub(crate) fn current_public_closeout_components_with_matrix_loader<F>(
    load_matrix: F,
) -> Result<
    CurrentWorthTouchedGraphConflictPublicCloseoutComponents,
    WorthTouchedGraphConflictPublicCloseoutError,
>
where
    F: FnOnce() -> Result<
        KernelCompiledProductConsumerDependencyMatrix,
        KernelCompiledProductConsumerDependencyError,
    >,
{
    let cutover = current_worth_workload_ordinary_consumer_cutover().map_err(|error| {
        WorthTouchedGraphConflictPublicCloseoutError::new(
            WorthTouchedGraphConflictPublicCloseoutErrorKind::CurrentProofUnavailable,
            format!("phase 13 ordinary-consumer cutover did not assemble: {error:?}"),
        )
    })?;
    load_matrix().map_err(|error| {
        WorthTouchedGraphConflictPublicCloseoutError::new(
            WorthTouchedGraphConflictPublicCloseoutErrorKind::CurrentProofUnavailable,
            format!("phase 14 kernel consumer dependency matrix did not assemble: {error:?}"),
        )
    })?;
    let deletion_closeout =
        current_worth_touched_graph_conflict_deletion_closeout().map_err(|error| {
            WorthTouchedGraphConflictPublicCloseoutError::new(
                WorthTouchedGraphConflictPublicCloseoutErrorKind::CurrentProofUnavailable,
                format!("phase 13 deletion closeout did not assemble: {error:?}"),
            )
        })?;
    let source_firewall_report = current_worth_touched_graph_conflict_source_firewall_report()
        .map_err(|error| {
            WorthTouchedGraphConflictPublicCloseoutError::new(
                WorthTouchedGraphConflictPublicCloseoutErrorKind::CurrentProofUnavailable,
                format!("phase 13 source firewall report did not assemble: {error:?}"),
            )
        })?;
    let selected_route_packet = crate::workload_composition::current_worth_touched_graph_conflict_selected_route_packet()
        .map_err(map_planner_owned_routing_error)?;
    let admitted_public_proof_input =
        admit_worth_touched_graph_conflict_public_proof_input(&selected_route_packet)
            .map_err(map_planner_owned_routing_error)?;
    Ok(
        CurrentWorthTouchedGraphConflictPublicCloseoutComponents::new(
            cutover,
            deletion_closeout,
            source_firewall_report,
            selected_route_packet,
            admitted_public_proof_input,
        ),
    )
}

#[cfg(test)]
pub(crate) fn current_public_closeout_components_with_matrix_targets_loader<F>(
    load_targets: F,
) -> Result<
    CurrentWorthTouchedGraphConflictPublicCloseoutComponents,
    WorthTouchedGraphConflictPublicCloseoutError,
>
where
    F: FnOnce() -> Result<
        Vec<KernelCompiledProductConsumerCoverageTarget>,
        KernelCompiledProductConsumerDependencyError,
    >,
{
    current_public_closeout_components_with_matrix_loader(|| {
        current_kernel_compiled_product_consumer_dependency_matrix_with_targets_loader(load_targets)
    })
}
