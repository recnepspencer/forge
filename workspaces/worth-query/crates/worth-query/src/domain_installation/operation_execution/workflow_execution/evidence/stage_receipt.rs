use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::operation_authority_chain::{
    mint_operation_phase_proof, operation_phase_basis,
};
use crate::domain_installation::operation_identity_basis::{
    canonical_indexed_operation_material, canonical_operation_material, graph_call_kind_material,
    lineage_outcome_material, operation_result_state_material, workflow_counter_material,
    workflow_semantic_value_material, workflow_warning_material,
};
use crate::identity::hash_parts;

use super::{
    WorthQueryBoundGraphExecutionReceipt, WorthQueryWorkflowEffectEvidence,
    WorthQueryWorkflowInvariantOutcome, WorthQueryWorkflowPrimaryReadEvidence,
    WorthQueryWorkflowRun, WorthQueryWorkflowRunCounters, WorthQueryWorkflowStageReceipt,
    WorthQueryWorkflowStageWarning, WorthQueryWorkflowValue,
};

pub(super) struct WorthQueryAdmittedWorkflowStageEvidence {
    pub(super) input: super::WorthQueryWorkflowSemanticValue,
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
    pub(super) lineage: Vec<crate::identity_evolution::InstalledIdentityEvolutionOutcome>,
}

impl<D, O, F, L: BasisOperationLane> WorthQueryWorkflowRun<D, O, F, L> {
    pub(super) fn retain_admitted_stage(
        &mut self,
        stage: &worth_query_installation::facade::WorthQueryPortableWorkflowStage,
        predecessor_receipt_identities: Vec<String>,
        evidence: WorthQueryAdmittedWorkflowStageEvidence,
    ) {
        let stage_identity = stage.identity();
        let identity_material = canonical_operation_material(vec![
            (
                "receipt.schema",
                "worth-query-workflow-stage-receipt-v2".into(),
            ),
            ("receipt.run", self.identity.clone()),
            ("receipt.stage", stage_identity.into()),
            (
                "receipt.predecessors",
                canonical_indexed_operation_material(
                    "receipt.predecessor",
                    predecessor_receipt_identities.iter().cloned(),
                ),
            ),
            (
                "receipt.input",
                workflow_semantic_value_material(&evidence.input),
            ),
            ("receipt.output", evidence.output.semantic_part()),
            (
                "receipt.result_state",
                operation_result_state_material(evidence.result_state).into(),
            ),
            ("receipt.warnings", warning_semantics(&evidence)),
            ("receipt.graph_evidence", graph_semantics(&evidence)),
            ("receipt.primary_reads", read_semantics(&evidence)),
            ("receipt.effects", effect_semantics(&evidence)),
            ("receipt.invariants", invariant_semantics(&evidence)),
            (
                "receipt.parallel_admission",
                self.active_parallel_admission
                    .as_ref()
                    .map(|receipt| receipt.identity())
                    .unwrap_or("not-required")
                    .into(),
            ),
            (
                "receipt.counters",
                workflow_counter_material(evidence.counters),
            ),
            (
                "receipt.conditional",
                canonical_indexed_operation_material(
                    "receipt.signal",
                    evidence
                        .conditional
                        .iter()
                        .map(|item| item.signal_identity().to_owned()),
                ),
            ),
            (
                "receipt.lineage",
                canonical_indexed_operation_material(
                    "receipt.lineage.outcome",
                    evidence.lineage.iter().map(lineage_outcome_material),
                ),
            ),
        ]);
        let identity = hash_parts(&[identity_material]);
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
            input: evidence.input,
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
            lineage: evidence.lineage,
        });
        self.receipt_index
            .insert(stage_identity.into(), receipt_index);
        self.completed.insert(stage_identity.into());
    }
}

fn warning_semantics(evidence: &WorthQueryAdmittedWorkflowStageEvidence) -> String {
    canonical_indexed_operation_material(
        "receipt.warning",
        evidence.warnings.iter().map(workflow_warning_material),
    )
}

fn graph_semantics(evidence: &WorthQueryAdmittedWorkflowStageEvidence) -> String {
    canonical_indexed_operation_material(
        "receipt.graph",
        evidence.graph_receipts.iter().map(|receipt| {
            canonical_operation_material(vec![
                ("graph.role", receipt.role().into()),
                (
                    "graph.kind",
                    graph_call_kind_material(receipt.kind()).into(),
                ),
                ("graph.evidence", receipt.evidence_identity().into()),
                (
                    "graph.projection",
                    receipt
                        .projection()
                        .map(|projection| projection.receipt().result_digest())
                        .unwrap_or("not-projected")
                        .into(),
                ),
            ])
        }),
    )
}

fn read_semantics(evidence: &WorthQueryAdmittedWorkflowStageEvidence) -> String {
    canonical_indexed_operation_material(
        "receipt.read",
        evidence.primary_read_evidence.iter().map(|read| {
            canonical_operation_material(vec![
                ("read.role", read.role().into()),
                ("read.graph", read.read_receipt().read_graph_digest().into()),
                ("read.result", read.read_receipt().result_digest().into()),
            ])
        }),
    )
}

fn effect_semantics(evidence: &WorthQueryAdmittedWorkflowStageEvidence) -> String {
    canonical_indexed_operation_material(
        "receipt.effect",
        evidence.effect_evidence.iter().map(|effect| {
            canonical_operation_material(vec![
                ("effect.family", effect.family().as_str().into()),
                ("effect.receipt", effect.receipt_identity().into()),
            ])
        }),
    )
}

fn invariant_semantics(evidence: &WorthQueryAdmittedWorkflowStageEvidence) -> String {
    canonical_indexed_operation_material(
        "receipt.invariant",
        evidence.invariant_outcomes.iter().map(|outcome| {
            canonical_operation_material(vec![
                ("invariant.role", outcome.invariant_role().into()),
                (
                    "invariant.installed",
                    outcome.installed_invariant_identity().into(),
                ),
                (
                    "invariant.effects",
                    canonical_indexed_operation_material(
                        "invariant.effect",
                        outcome.effect_receipt_identities().iter().cloned(),
                    ),
                ),
            ])
        }),
    )
}
