use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::operation_authority_chain::{
    mint_operation_phase_proof, operation_phase_basis, WorthQueryCompletedWorkflowPhase,
    WorthQueryOperationPhaseProof,
};
use crate::domain_installation::operation_identity_basis::{
    canonical_indexed_operation_material, canonical_operation_material, graph_call_kind_material,
    operation_result_state_material, workflow_warning_material,
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
            return TransitionOutcome::Stale(WorthQueryWorkflowCompletionDenial::from_run(
                WorthQueryWorkflowCompletionDenialKind::StaleInstallationGeneration,
                &self,
            ));
        }
        if self.completed.len() != self.graph.stages().len() {
            return TransitionOutcome::Denied(WorthQueryWorkflowCompletionDenial::from_run(
                WorthQueryWorkflowCompletionDenialKind::IncompleteStages,
                &self,
            ));
        }
        let mut run = self;
        for receipt in run.receipts.iter_mut().rev() {
            receipt.retire_artifact_output();
        }
        run.artifact_registry.close_released();
        let mut trace = mint_completed_trace(run);
        match crate::domain_installation::dependency_impact::compile_workflow_semantic_aspect_dependencies(&trace) {
            Ok(dependency_closure) => trace.dependency_closure = Some(dependency_closure),
            Err(denial) => {
                return TransitionOutcome::Denied(WorthQueryWorkflowCompletionDenial::from_trace(
                    WorthQueryWorkflowCompletionDenialKind::DependencyCompilation(denial),
                    &trace,
                ));
            }
        }
        match crate::domain_installation::operation_lineage::bind_execution_lineage(trace) {
            Ok(trace) => TransitionOutcome::Success(trace),
            Err((trace, _)) => {
                TransitionOutcome::Denied(WorthQueryWorkflowCompletionDenial::from_trace(
                    WorthQueryWorkflowCompletionDenialKind::LineageEvidence,
                    &trace,
                ))
            }
        }
    }
}

fn mint_completed_trace<D, O, F, L: BasisOperationLane>(
    run: WorthQueryWorkflowRun<D, O, F, L>,
) -> WorthQueryCompletedWorkflowTrace<D, O, F, L> {
    let mut receipt_identities = run
        .receipts
        .iter()
        .map(|receipt| (receipt.stage_identity(), receipt.identity()))
        .collect::<Vec<_>>();
    receipt_identities.sort();
    let operation_conditional = canonical_indexed_operation_material(
        "workflow.operation.conditional",
        run.operation_conditional_provenance()
            .iter()
            .map(super::workflow_conditional_trace::conditional_trace_operational_material),
    );
    let identity = hash_parts(&[
        "worth_query_completed_workflow_trace_v1".into(),
        format!("run:{}", run.identity),
        format!("operation_conditional:{operation_conditional}"),
        format!(
            "receipts:{}",
            receipt_identities
                .iter()
                .map(|(_, identity)| *identity)
                .collect::<Vec<_>>()
                .join(",")
        ),
    ]);
    let semantic_identity = semantic_trace_identity(&run);
    let phase_proof = mint_operation_phase_proof(
        identity.clone(),
        Some(run.authority_proof.proof.payload().identity()),
        operation_phase_basis(&run.authority_proof.proof).clone(),
    );
    WorthQueryCompletedWorkflowTrace {
        run,
        identity,
        semantic_identity,
        phase_proof,
        lineage: None,
        dependency_closure: None,
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
        format!(
            "operation_conditional:{}",
            canonical_indexed_operation_material(
                "workflow.operation.conditional",
                run.operation_conditional_provenance()
                    .iter()
                    .map(super::workflow_conditional_trace::conditional_trace_semantic_material),
            )
        ),
        format!("stages:{}", semantic_parts.join("|")),
    ])
}

fn stage_semantic_part(receipt: &WorthQueryWorkflowStageReceipt) -> String {
    canonical_operation_material(vec![
        ("stage.identity", receipt.stage_identity.clone()),
        (
            "stage.predecessors",
            canonical_indexed_operation_material(
                "stage.predecessor",
                receipt.predecessor_stage_identities.iter().cloned(),
            ),
        ),
        (
            "stage.result_state",
            operation_result_state_material(receipt.result_state).into(),
        ),
        (
            "stage.output",
            crate::domain_installation::operation_identity_basis::workflow_semantic_value_material(
                &receipt.output_semantics,
            ),
        ),
        (
            "stage.warnings",
            canonical_indexed_operation_material(
                "stage.warning",
                receipt.warnings.iter().map(workflow_warning_material),
            ),
        ),
        (
            "stage.domain_evidence",
            receipt
                .domain_evidence()
                .map(super::WorthQueryAdmittedDomainEvidence::replay_meaning)
                .map(|meaning| meaning.semantic_material())
                .unwrap_or_else(|| "not-required".into()),
        ),
        (
            "stage.graph",
            canonical_indexed_operation_material(
                "stage.graph.receipt",
                receipt.graph_receipts.iter().map(|graph| {
                    canonical_operation_material(vec![
                        ("graph.role", graph.role().into()),
                        ("graph.kind", graph_call_kind_material(graph.kind()).into()),
                        ("graph.evidence", graph.evidence_identity().into()),
                        (
                            "graph.projection",
                            graph
                                .projection()
                                .map(|projection| projection.receipt().result_digest())
                                .unwrap_or("not-projected")
                                .into(),
                        ),
                    ])
                }),
            ),
        ),
        (
            "stage.reads",
            canonical_indexed_operation_material(
                "stage.read",
                receipt.primary_read_evidence.iter().map(|read| {
                    canonical_operation_material(vec![
                        ("read.role", read.role().into()),
                        ("read.result", read.read_receipt().result_digest().into()),
                    ])
                }),
            ),
        ),
        (
            "stage.effects",
            canonical_indexed_operation_material(
                "stage.effect",
                receipt.effect_evidence.iter().map(|effect| {
                    canonical_operation_material(vec![
                        ("effect.family", effect.family().as_str().into()),
                        ("effect.receipt", effect.receipt_identity().into()),
                    ])
                }),
            ),
        ),
        (
            "stage.invariants",
            canonical_indexed_operation_material(
                "stage.invariant",
                receipt.invariant_outcomes.iter().map(|outcome| {
                    canonical_operation_material(vec![
                        ("invariant.role", outcome.invariant_role().into()),
                        (
                            "invariant.installed",
                            outcome.installed_invariant_identity().into(),
                        ),
                    ])
                }),
            ),
        ),
    ])
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowCompletionDenialKind {
    StaleInstallationGeneration,
    IncompleteStages,
    LineageEvidence,
    DependencyCompilation(
        crate::domain_installation::WorthQuerySemanticAspectDependencyCompilationDenial,
    ),
}

#[derive(Debug)]
pub struct WorthQueryWorkflowCompletionDenial {
    kind: WorthQueryWorkflowCompletionDenialKind,
    executed_effects: Vec<super::WorthQueryWorkflowEffectEvidence>,
    counters: WorthQueryWorkflowRunCounters,
}

impl WorthQueryWorkflowCompletionDenial {
    fn from_run<D, O, F, L: BasisOperationLane>(
        kind: WorthQueryWorkflowCompletionDenialKind,
        run: &WorthQueryWorkflowRun<D, O, F, L>,
    ) -> Self {
        Self {
            kind,
            executed_effects: run
                .receipts
                .iter()
                .flat_map(|receipt| receipt.effect_evidence().iter().cloned())
                .collect(),
            counters: run.counters,
        }
    }

    fn from_trace<D, O, F, L: BasisOperationLane>(
        kind: WorthQueryWorkflowCompletionDenialKind,
        trace: &WorthQueryCompletedWorkflowTrace<D, O, F, L>,
    ) -> Self {
        Self::from_run(kind, &trace.run)
    }

    pub const fn kind(&self) -> WorthQueryWorkflowCompletionDenialKind {
        self.kind
    }

    pub fn executed_effects(&self) -> &[super::WorthQueryWorkflowEffectEvidence] {
        &self.executed_effects
    }

    pub const fn counters(&self) -> WorthQueryWorkflowRunCounters {
        self.counters
    }
}

pub struct WorthQueryCompletedWorkflowTrace<D, O, F, L: BasisOperationLane> {
    pub(super) run: WorthQueryWorkflowRun<D, O, F, L>,
    pub(super) identity: String,
    semantic_identity: String,
    pub(super) phase_proof: WorthQueryOperationPhaseProof<WorthQueryCompletedWorkflowPhase>,
    pub(crate) lineage: Option<crate::domain_installation::WorthQueryTraceLineageReport>,
    dependency_closure:
        Option<crate::domain_installation::WorthQueryCompiledSemanticAspectDependencyClosure>,
}

impl<D, O, F, L: BasisOperationLane> WorthQueryCompletedWorkflowTrace<D, O, F, L> {
    pub(crate) fn bound(
        &self,
    ) -> &crate::domain_installation::WorthQueryBoundDomainOperation<D, O, F, L> {
        &self.run.bound
    }
    pub(crate) fn phase_proof(
        &self,
    ) -> &WorthQueryOperationPhaseProof<WorthQueryCompletedWorkflowPhase> {
        &self.phase_proof
    }
    pub(crate) fn workflow_run_identity(&self) -> &str {
        self.run.identity()
    }
    pub(crate) fn installed_workflow_read(
        &self,
    ) -> Option<&crate::ordinary::read::WorthQueryReadDeclaration> {
        self.run.executor.installed_read.as_ref()
    }
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
    pub fn operation_conditional_provenance(
        &self,
    ) -> &[crate::domain_installation::WorthQueryConditionalProvenance] {
        self.run.operation_conditional_provenance()
    }
    pub fn counters(&self) -> WorthQueryWorkflowRunCounters {
        self.run.counters
    }
    pub fn lineage_report(
        &self,
    ) -> Option<&crate::domain_installation::WorthQueryTraceLineageReport> {
        self.lineage.as_ref()
    }
    pub fn semantic_aspect_dependency_closure(
        &self,
    ) -> Option<&crate::domain_installation::WorthQueryCompiledSemanticAspectDependencyClosure>
    {
        self.dependency_closure.as_ref()
    }
    pub fn classify_authoritative_impact(
        &self,
        delivery: &worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
        conditional: &crate::domain_installation::WorthQueryConditionalProvenance,
    ) -> Result<
        crate::domain_installation::WorthQueryImpactDecision,
        crate::domain_installation::WorthQueryImpactAdmissionDenial,
    > {
        let closure = self.semantic_aspect_dependency_closure().ok_or_else(|| {
            crate::domain_installation::WorthQueryImpactAdmissionDenial::new(
                crate::domain_installation::WorthQueryImpactAdmissionDenialKind::DependencyClosureUnavailable,
                crate::domain_installation::WorthQueryImpactCounters::default(),
            )
        })?;
        crate::domain_installation::classify_owner_delivered_impact(closure, delivery, conditional)
    }
    pub(crate) fn refresh_semantic_identity_for_lineage(&mut self) {
        let Some(lineage) = &self.lineage else {
            return;
        };
        self.semantic_identity = hash_parts(&[
            "worth_query_workflow_semantic_trace_with_lineage_v1".into(),
            format!("workflow:{}", self.semantic_identity),
            format!("lineage:{}", lineage.semantic_part()),
        ]);
    }
}
