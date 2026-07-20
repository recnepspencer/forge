use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::operation_authority_chain::{
    mint_operation_phase_proof, operation_phase_basis,
};
use crate::identity::hash_parts;

use super::{
    WorthQueryBoundGraphExecutionReceipt, WorthQueryWorkflowEffectEvidence,
    WorthQueryWorkflowInvariantOutcome, WorthQueryWorkflowPrimaryReadEvidence,
    WorthQueryWorkflowRun, WorthQueryWorkflowRunCounters, WorthQueryWorkflowStageReceipt,
    WorthQueryWorkflowStageWarning, WorthQueryWorkflowValue,
};

pub(super) struct WorthQueryAdmittedWorkflowStageEvidence {
    pub(super) output: WorthQueryWorkflowValue,
    pub(super) result_state: Option<crate::domain_installation::WorthQueryOperationResultState>,
    pub(super) warnings: Vec<WorthQueryWorkflowStageWarning>,
    pub(super) graph_receipts: Vec<WorthQueryBoundGraphExecutionReceipt>,
    pub(super) primary_read_evidence: Vec<WorthQueryWorkflowPrimaryReadEvidence>,
    pub(super) effect_evidence: Vec<WorthQueryWorkflowEffectEvidence>,
    pub(super) invariant_outcomes: Vec<WorthQueryWorkflowInvariantOutcome>,
    pub(super) counters: WorthQueryWorkflowRunCounters,
    pub(super) execution_snapshot: crate::memory_workspace::WorthQuerySnapshotIdentity,
    pub(super) conditional: Vec<crate::domain_installation::WorthQueryConditionalProvenance>,
}

impl<D, O, F, L: BasisOperationLane> WorthQueryWorkflowRun<D, O, F, L> {
    pub(super) fn retain_admitted_stage(
        &mut self,
        stage: &worth_query_installation::facade::WorthQueryPortableWorkflowStage,
        predecessor_receipt_identities: Vec<String>,
        evidence: WorthQueryAdmittedWorkflowStageEvidence,
    ) {
        let stage_identity = stage.identity();
        let identity = hash_parts(&[
            "worth_query_workflow_stage_receipt_v1".into(),
            format!("run:{}", self.identity),
            format!("stage:{stage_identity}"),
            format!("predecessors:{}", predecessor_receipt_identities.join(",")),
            format!("output:{}", evidence.output.semantic_part()),
            format!("result_state:{:?}", evidence.result_state),
            format!("warnings:{}", warning_semantics(&evidence)),
            format!("graph_evidence:{}", graph_semantics(&evidence)),
            format!("primary_reads:{}", read_semantics(&evidence)),
            format!("effects:{}", effect_semantics(&evidence)),
            format!("invariants:{}", invariant_semantics(&evidence)),
            format!(
                "parallel_admission:{}",
                self.active_parallel_admission
                    .as_ref()
                    .map(|receipt| receipt.identity())
                    .unwrap_or("not-required")
            ),
            format!("counters:{:?}", evidence.counters),
            format!(
                "conditional:{}",
                evidence
                    .conditional
                    .iter()
                    .map(|item| item.signal_identity())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        ]);
        let predecessor_authority_proofs = stage
            .predecessors()
            .iter()
            .map(|predecessor| {
                let receipt = self
                    .receipt_index
                    .get(predecessor)
                    .map(|index| &self.receipts[*index])
                    .expect("admitted predecessor retains its Query authority proof");
                std::sync::Arc::clone(&receipt.stage_authority_proof)
            })
            .collect::<Vec<_>>();
        debug_assert!(predecessor_authority_proofs
            .iter()
            .zip(&predecessor_receipt_identities)
            .all(|(proof, identity)| {
                proof.proof.payload().identity() == identity
                    && std::sync::Arc::ptr_eq(&proof.run_authority, &self.authority_proof)
            }));
        let stage_authority_proof =
            std::sync::Arc::new(super::WorthQueryWorkflowStageAuthorityProof {
                stage_identity: stage_identity.into(),
                run_authority: std::sync::Arc::clone(&self.authority_proof),
                proof: mint_operation_phase_proof(
                    identity.clone(),
                    Some(self.authority_proof.proof.payload().identity()),
                    operation_phase_basis(&self.authority_proof.proof).clone(),
                ),
            });
        let receipt_index = self.receipts.len();
        self.receipts.push(WorthQueryWorkflowStageReceipt {
            identity,
            run_identity: self.identity.clone(),
            binding_identity: self.bound.binding_identity().into(),
            stage_identity: stage_identity.into(),
            predecessor_stage_identities: stage.predecessors().to_vec(),
            predecessor_receipt_identities,
            output: evidence.output,
            result_state: evidence.result_state,
            warnings: evidence.warnings,
            graph_receipts: evidence.graph_receipts,
            primary_read_evidence: evidence.primary_read_evidence,
            effect_evidence: evidence.effect_evidence,
            invariant_outcomes: evidence.invariant_outcomes,
            parallel_admission: self.active_parallel_admission.clone(),
            counters: evidence.counters,
            authority_proof: std::sync::Arc::clone(&self.authority_proof),
            stage_authority_proof,
            predecessor_authority_proofs,
            execution_snapshot: evidence.execution_snapshot,
            conditional: evidence.conditional,
        });
        self.receipt_index
            .insert(stage_identity.into(), receipt_index);
        self.completed.insert(stage_identity.into());
    }
}

fn warning_semantics(evidence: &WorthQueryAdmittedWorkflowStageEvidence) -> String {
    evidence
        .warnings
        .iter()
        .map(|warning| format!("{warning:?}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn graph_semantics(evidence: &WorthQueryAdmittedWorkflowStageEvidence) -> String {
    evidence
        .graph_receipts
        .iter()
        .map(|receipt| {
            format!(
                "{}:{:?}:{}:{}",
                receipt.role(),
                receipt.kind(),
                receipt.evidence_identity(),
                receipt
                    .projection()
                    .map(|projection| projection.receipt().result_digest())
                    .unwrap_or("not-projected")
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn read_semantics(evidence: &WorthQueryAdmittedWorkflowStageEvidence) -> String {
    evidence
        .primary_read_evidence
        .iter()
        .map(|read| {
            format!(
                "{}:{}:{}",
                read.role(),
                read.read_receipt().read_graph_digest(),
                read.read_receipt().result_digest(),
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn effect_semantics(evidence: &WorthQueryAdmittedWorkflowStageEvidence) -> String {
    evidence
        .effect_evidence
        .iter()
        .map(|effect| format!("{}:{}", effect.family().as_str(), effect.receipt_identity()))
        .collect::<Vec<_>>()
        .join(",")
}

fn invariant_semantics(evidence: &WorthQueryAdmittedWorkflowStageEvidence) -> String {
    evidence
        .invariant_outcomes
        .iter()
        .map(|outcome| {
            format!(
                "{}:{}:{}",
                outcome.invariant_role(),
                outcome.installed_invariant_identity(),
                outcome.effect_receipt_identities().join("+")
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}
