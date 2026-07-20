use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::operation_authority_chain::{
    mint_operation_phase_proof, operation_phase_basis, WorthQueryCompletedWorkflowPhase,
    WorthQueryOperationPhaseProof,
};
use crate::identity::hash_parts;

use super::{WorthQueryWorkflowRun, WorthQueryWorkflowRunCounters, WorthQueryWorkflowStageReceipt};
use worth_proof::TransitionOutcome;

impl<D: 'static, O: 'static, F: 'static, L: BasisOperationLane> WorthQueryWorkflowRun<D, O, F, L> {
    pub fn complete(
        self,
    ) -> TransitionOutcome<
        WorthQueryCompletedWorkflowTrace<D, O, F, L>,
        WorthQueryWorkflowCompletionDenial,
        std::convert::Infallible,
        WorthQueryWorkflowCompletionDenial,
        WorthQueryWorkflowCompletionDenial,
        WorthQueryWorkflowCompletionDenial,
    > {
        if !self.bound.installation_is_current() {
            return TransitionOutcome::Stale(
                WorthQueryWorkflowCompletionDenial::StaleInstallationGeneration,
            );
        }
        if self.completed.len() != self.graph.stages().len() {
            return TransitionOutcome::Denied(WorthQueryWorkflowCompletionDenial::IncompleteStages);
        }
        let mut receipt_identities = self
            .receipts
            .iter()
            .map(|receipt| (receipt.stage_identity(), receipt.identity()))
            .collect::<Vec<_>>();
        receipt_identities.sort();
        let identity = hash_parts(&[
            "worth_query_completed_workflow_trace_v1".into(),
            format!("run:{}", self.identity),
            format!(
                "receipts:{}",
                receipt_identities
                    .iter()
                    .map(|(_, identity)| *identity)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        ]);
        let semantic_identity = semantic_trace_identity(&self);
        let phase_proof = mint_operation_phase_proof(
            identity.clone(),
            Some(self.authority_proof.proof.payload().identity()),
            operation_phase_basis(&self.authority_proof.proof).clone(),
        );
        TransitionOutcome::Success(WorthQueryCompletedWorkflowTrace {
            run: self,
            identity,
            semantic_identity,
            phase_proof,
        })
    }
}

fn semantic_trace_identity<D, O, F, L: BasisOperationLane>(
    run: &WorthQueryWorkflowRun<D, O, F, L>,
) -> String {
    let mut semantic_parts = run
        .receipts
        .iter()
        .map(stage_semantic_part)
        .collect::<Vec<_>>();
    semantic_parts.sort();
    hash_parts(&[
        "worth_query_workflow_semantic_trace_v1".into(),
        format!("operation:{}", run.bound.definition().canonical_identity()),
        format!("stages:{}", semantic_parts.join("|")),
    ])
}

fn stage_semantic_part(receipt: &WorthQueryWorkflowStageReceipt) -> String {
    format!(
        "{}:{}:{:?}:{}:{}:{}:{}:{}:{}",
        receipt.stage_identity,
        receipt.predecessor_stage_identities.join(","),
        receipt.result_state,
        receipt.output.semantic_part(),
        receipt
            .warnings
            .iter()
            .map(|warning| format!("{warning:?}"))
            .collect::<Vec<_>>()
            .join(","),
        receipt
            .graph_receipts
            .iter()
            .map(|graph| {
                format!(
                    "{}:{:?}:{}",
                    graph.role(),
                    graph.kind(),
                    graph
                        .projection()
                        .map(|projection| projection.receipt().result_digest())
                        .unwrap_or("not-projected")
                )
            })
            .collect::<Vec<_>>()
            .join(","),
        receipt
            .primary_read_evidence
            .iter()
            .map(|read| format!("{}:{}", read.role(), read.read_receipt().result_digest()))
            .collect::<Vec<_>>()
            .join(","),
        receipt
            .effect_evidence
            .iter()
            .map(|effect| effect.family().as_str().to_string())
            .collect::<Vec<_>>()
            .join(","),
        receipt
            .invariant_outcomes
            .iter()
            .map(|outcome| format!(
                "{}:{}",
                outcome.invariant_role(),
                outcome.installed_invariant_identity()
            ))
            .collect::<Vec<_>>()
            .join(","),
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowCompletionDenial {
    StaleInstallationGeneration,
    IncompleteStages,
}

pub struct WorthQueryCompletedWorkflowTrace<D, O, F, L: BasisOperationLane> {
    pub(super) run: WorthQueryWorkflowRun<D, O, F, L>,
    pub(super) identity: String,
    semantic_identity: String,
    pub(super) phase_proof: WorthQueryOperationPhaseProof<WorthQueryCompletedWorkflowPhase>,
}

impl<D, O, F, L: BasisOperationLane> WorthQueryCompletedWorkflowTrace<D, O, F, L> {
    pub fn identity(&self) -> &str {
        debug_assert_eq!(self.phase_proof.payload().identity(), self.identity);
        &self.identity
    }
    pub fn semantic_identity(&self) -> &str {
        &self.semantic_identity
    }
    pub fn stage_receipts(&self) -> &[WorthQueryWorkflowStageReceipt] {
        &self.run.receipts
    }
    pub fn counters(&self) -> WorthQueryWorkflowRunCounters {
        self.run.counters
    }
}
