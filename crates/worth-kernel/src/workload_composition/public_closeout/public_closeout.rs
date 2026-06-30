use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::milestone_fourteen_seed::WorthTouchedGraphConflictMilestoneFourteenSeed;
use super::proof_chain::WorthTouchedGraphConflictProofChain;
use super::residue_chain::WorthTouchedGraphConflictResidueChain;
use crate::workload_composition::worth_workload::{
    current_replay_undo_boundary_route_authority, current_worth_workload_ordinary_consumer_cutover,
    WorthWorkloadOrdinaryConsumerCutover,
};
use crate::workload_composition::{
    current_kernel_compiled_product_consumer_dependency_matrix,
    current_worth_touched_graph_conflict_deletion_closeout,
    current_worth_touched_graph_conflict_source_firewall_report, BatchAdmissionExecutionReceipt,
    KernelCompiledProductConsumerDependencyError, KernelCompiledProductConsumerDependencyMatrix,
    WorthTouchedGraphConflictDeletionCloseout, WorthTouchedGraphConflictSourceFirewallReport,
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
    batch_execution_receipt: &'a BatchAdmissionExecutionReceipt,
    deletion_closeout: &'a WorthTouchedGraphConflictDeletionCloseout,
    source_firewall_report: &'a WorthTouchedGraphConflictSourceFirewallReport,
}

pub(crate) struct CurrentWorthTouchedGraphConflictPublicCloseoutComponents {
    cutover: WorthWorkloadOrdinaryConsumerCutover,
    deletion_closeout: WorthTouchedGraphConflictDeletionCloseout,
    source_firewall_report: WorthTouchedGraphConflictSourceFirewallReport,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictPublicCloseout {
    proof_chain: WorthTouchedGraphConflictProofChain,
    residue_chain: WorthTouchedGraphConflictResidueChain,
    source_firewall_digest: String,
    deletion_closeout_digest: String,
    milestone_fourteen_seed: WorthTouchedGraphConflictMilestoneFourteenSeed,
    closeout_digest: String,
}

impl<'a> WorthTouchedGraphConflictPublicCloseoutInput<'a> {
    pub(crate) fn new(
        batch_execution_receipt: &'a BatchAdmissionExecutionReceipt,
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
            batch_execution_receipt,
            deletion_closeout,
            source_firewall_report,
        })
    }

    pub(crate) const fn batch_execution_receipt(&self) -> &'a BatchAdmissionExecutionReceipt {
        self.batch_execution_receipt
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

impl WorthTouchedGraphConflictPublicCloseout {
    pub fn current() -> Result<Self, WorthTouchedGraphConflictPublicCloseoutError> {
        current_worth_touched_graph_conflict_public_closeout()
    }

    pub fn proof_chain(&self) -> &WorthTouchedGraphConflictProofChain {
        &self.proof_chain
    }

    pub fn residue_chain(&self) -> &WorthTouchedGraphConflictResidueChain {
        &self.residue_chain
    }

    pub fn source_firewall_digest(&self) -> &str {
        &self.source_firewall_digest
    }

    pub fn deletion_closeout_digest(&self) -> &str {
        &self.deletion_closeout_digest
    }

    pub fn milestone_fourteen_seed(&self) -> &WorthTouchedGraphConflictMilestoneFourteenSeed {
        &self.milestone_fourteen_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}

pub fn current_worth_touched_graph_conflict_public_closeout(
) -> Result<WorthTouchedGraphConflictPublicCloseout, WorthTouchedGraphConflictPublicCloseoutError> {
    let components = current_public_closeout_components()?;
    publish_from_parts(
        components.input()?,
        components.cutover(),
        components.residue_chain(),
    )
}

pub fn current_worth_touched_graph_conflict_milestone_fourteen_seed() -> Result<
    WorthTouchedGraphConflictMilestoneFourteenSeed,
    WorthTouchedGraphConflictPublicCloseoutError,
> {
    Ok(current_worth_touched_graph_conflict_public_closeout()?
        .milestone_fourteen_seed()
        .clone())
}

impl WorthTouchedGraphConflictPublicCloseoutError {
    fn new(
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

pub(crate) fn publish_from_parts(
    input: WorthTouchedGraphConflictPublicCloseoutInput<'_>,
    cutover: &WorthWorkloadOrdinaryConsumerCutover,
    residue_chain: WorthTouchedGraphConflictResidueChain,
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
            "public closeout cannot publish while a covered ordinary consumer still depends on old conflict or serialization authority",
        ));
    }
    let proof_chain = WorthTouchedGraphConflictProofChain::from_cutover(cutover);
    if proof_chain.selected_conflict_plan_digests().is_empty()
        || proof_chain.overlap_identity_digests().is_empty()
        || proof_chain.locality_footprint_digests().is_empty()
        || proof_chain.independence_proof_digests().is_empty()
    {
        return Err(WorthTouchedGraphConflictPublicCloseoutError::new(
            WorthTouchedGraphConflictPublicCloseoutErrorKind::IncompleteProofChain,
            "public closeout requires selected conflict plans, overlap identity, locality proof, and independence proof to survive into the execution receipt chain",
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
    require_current_replay_undo_proof_chain(cutover, &proof_chain)?;
    let source_firewall_digest = input.source_firewall_report().report_digest().to_string();
    let deletion_closeout_digest = input.deletion_closeout().closeout_digest().to_string();
    let milestone_fourteen_seed =
        WorthTouchedGraphConflictMilestoneFourteenSeed::from_closeout_parts(
            &proof_chain,
            residue_chain.residue_digest(),
            &source_firewall_digest,
        );
    let closeout_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-kernel:touched-graph-conflict-public-closeout:v1".to_string(),
            format!("proof-chain:{}", proof_chain.proof_chain_digest()),
            format!("residue:{}", residue_chain.residue_digest()),
            format!("firewall:{source_firewall_digest}"),
            format!("deletion:{deletion_closeout_digest}"),
            format!("seed:{}", milestone_fourteen_seed.seed_digest()),
        ],
    );
    Ok(WorthTouchedGraphConflictPublicCloseout {
        proof_chain,
        residue_chain,
        source_firewall_digest,
        deletion_closeout_digest,
        milestone_fourteen_seed,
        closeout_digest,
    })
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
    Ok(CurrentWorthTouchedGraphConflictPublicCloseoutComponents {
        cutover,
        deletion_closeout,
        source_firewall_report,
    })
}

impl CurrentWorthTouchedGraphConflictPublicCloseoutComponents {
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
            self.cutover.batch_execution_receipt(),
            &self.deletion_closeout,
            &self.source_firewall_report,
        )
    }

    pub(crate) fn residue_chain(&self) -> WorthTouchedGraphConflictResidueChain {
        WorthTouchedGraphConflictResidueChain::from_cutover_rows(self.cutover.rows())
    }
}

fn require_current_replay_undo_proof_chain(
    cutover: &WorthWorkloadOrdinaryConsumerCutover,
    proof_chain: &WorthTouchedGraphConflictProofChain,
) -> Result<(), WorthTouchedGraphConflictPublicCloseoutError> {
    if cutover.replay_undo_selected_plan_witness_count() == 0 {
        return Ok(());
    }

    let current_route_authority =
        current_replay_undo_boundary_route_authority().map_err(|error| {
            WorthTouchedGraphConflictPublicCloseoutError::new(
                WorthTouchedGraphConflictPublicCloseoutErrorKind::CurrentProofUnavailable,
                format!("phase 13 replay/undo route authority did not assemble: {error:?}"),
            )
        })?;
    if proof_chain.replay_undo_boundary_proof_digests()
        != [current_route_authority.boundary_proof_digest().to_string()]
        || proof_chain.transaction_packet_identities()
            != [current_route_authority
                .transaction_packet_identity()
                .to_string()]
        || proof_chain.replay_scope_identities()
            != [current_route_authority.replay_scope_identity().to_string()]
        || proof_chain.undo_scope_identities()
            != [current_route_authority.undo_scope_identity().to_string()]
    {
        return Err(WorthTouchedGraphConflictPublicCloseoutError::new(
            WorthTouchedGraphConflictPublicCloseoutErrorKind::IncompleteProofChain,
            "public closeout requires current replay/undo admitted-boundary proof identities, not foreign proof joins",
        ));
    }
    Ok(())
}
