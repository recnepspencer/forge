use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::WorthQueryOperationGraphParticipation;

use super::workflow_stage_receipt::WorthQueryAdmittedWorkflowStageEvidence;
use super::{
    WorthQueryBoundGraphExecutionReceipt, WorthQueryWorkflowAdvanceDenial,
    WorthQueryWorkflowAdvanceDenialKind, WorthQueryWorkflowInvariantOutcome, WorthQueryWorkflowRun,
    WorthQueryWorkflowRunCounters, WorthQueryWorkflowSemanticValue,
    WorthQueryWorkflowStageMaterialParts,
};

pub(super) struct WorthQueryWorkflowStageValidationInput {
    pub(super) semantic_input: WorthQueryWorkflowSemanticValue,
    pub(super) material: WorthQueryWorkflowStageMaterialParts,
    pub(super) graph_receipts: Vec<WorthQueryBoundGraphExecutionReceipt>,
    pub(super) conditional: Vec<crate::domain_installation::WorthQueryConditionalProvenance>,
    pub(super) execution_snapshot: crate::memory_workspace::WorthQuerySnapshotIdentity,
    pub(super) effect_workflow_binding: crate::workflow::WorkflowContextBinding,
    pub(super) counters_before: WorthQueryWorkflowRunCounters,
    pub(super) resource_evidence: super::WorthQueryExecutionResourceAttemptEvidence,
}

impl WorthQueryWorkflowStageValidationInput {
    fn denial(
        &self,
        counters: WorthQueryWorkflowRunCounters,
        kind: WorthQueryWorkflowAdvanceDenialKind,
    ) -> WorthQueryWorkflowAdvanceDenial {
        WorthQueryWorkflowAdvanceDenial::with_executed_effects(
            kind,
            counters,
            self.material.executed_effects.clone(),
        )
        .with_graph_receipts(self.graph_receipts.clone())
    }
}

impl<D: 'static, O: 'static, F: 'static, L: BasisOperationLane> WorthQueryWorkflowRun<D, O, F, L> {
    pub(super) fn validate_stage_evidence(
        &mut self,
        stage: &worth_query_installation::facade::WorthQueryPortableWorkflowStage,
        mut input: WorthQueryWorkflowStageValidationInput,
    ) -> Result<WorthQueryAdmittedWorkflowStageEvidence, WorthQueryWorkflowAdvanceDenial> {
        self.validate_stage_lineage(stage, &input)?;
        self.validate_primary_reads(stage, &mut input)?;
        self.validate_effects(stage, &mut input)?;
        let invariant_outcomes = self.validate_invariants(stage, &input)?;
        self.validate_result_contracts(stage, &input)?;
        let output_semantics = input.material.output.semantic_value();
        let output_occurrence_identity =
            input.material.output.domain_evidence_occurrence_identity();
        let stage_counters = self.counters.delta_since(input.counters_before);
        self.validate_cost_contract(stage, stage_counters, &input)?;
        let evidence_contract = self
            .graph
            .artifact_contracts(stage.identity())
            .and_then(super::WorthQueryInstalledWorkflowArtifactContracts::evidence)
            .cloned();
        let domain_evidence =
            super::admit_domain_evidence(super::WorthQueryDomainEvidenceAdmissionInput {
                contract: evidence_contract.as_deref(),
                material: input.material.domain_evidence.take(),
                binding: super::WorthQueryDomainEvidenceBindingParts {
                    operation_identity: self.bound.definition().canonical_identity().to_owned(),
                    binding_identity: self.bound.binding_identity().to_owned(),
                    run_identity: Some(self.identity.clone()),
                    stage_identity: Some(stage.identity().to_owned()),
                    basis_identity: self.bound.basis().capability_digest().to_owned(),
                    execution_snapshot_identity: input
                        .execution_snapshot
                        .evidence_identity()
                        .as_str()
                        .to_owned(),
                    output_occurrence_identity,
                },
                ledger: Some(&mut self.domain_evidence_ledger),
            })
            .map_err(|denial| {
                input.denial(
                    self.counters,
                    WorthQueryWorkflowAdvanceDenialKind::DomainEvidence(denial.kind()),
                )
            })?;
        Ok(WorthQueryAdmittedWorkflowStageEvidence {
            input: input.semantic_input,
            output: input.material.output,
            output_semantics,
            result_state: input.material.result_state,
            warnings: input.material.warnings,
            graph_receipts: input.graph_receipts,
            primary_read_evidence: input.material.primary_graph_reads,
            effect_evidence: input.material.effects,
            invariant_outcomes,
            counters: stage_counters,
            execution_snapshot: input.execution_snapshot,
            conditional: input.conditional,
            lineage: input.material.lineage,
            domain_evidence,
            resource_evidence: input.resource_evidence,
        })
    }

    fn validate_stage_lineage(
        &self,
        stage: &worth_query_installation::facade::WorthQueryPortableWorkflowStage,
        input: &WorthQueryWorkflowStageValidationInput,
    ) -> Result<(), WorthQueryWorkflowAdvanceDenial> {
        let valid = super::workflow_lineage_validation::valid_stage_lineage(
            &input.material.lineage,
            self.bound.definition().semantics().lineage,
            &input.conditional,
            self.bound.definition().canonical_identity(),
            &self.identity,
            stage.identity(),
            &input.material.effects,
        );
        valid.then_some(()).ok_or_else(|| {
            input.denial(
                self.counters,
                WorthQueryWorkflowAdvanceDenialKind::LineageEvidence,
            )
        })
    }

    fn validate_primary_reads(
        &mut self,
        stage: &worth_query_installation::facade::WorthQueryPortableWorkflowStage,
        input: &mut WorthQueryWorkflowStageValidationInput,
    ) -> Result<(), WorthQueryWorkflowAdvanceDenial> {
        input
            .material
            .primary_graph_reads
            .sort_by(|left, right| left.role().cmp(right.role()));
        let mut expected = self
            .bound
            .definition()
            .semantics()
            .graph_reads
            .roles()
            .iter()
            .filter(|read| {
                stage.semantics().graph_read_roles.contains(&read.role)
                    && read.participation
                        == WorthQueryOperationGraphParticipation::PrimaryLogicalGraph
            })
            .map(|read| read.role.as_str())
            .collect::<Vec<_>>();
        expected.sort();
        let evidence_roles = input
            .material
            .primary_graph_reads
            .iter()
            .map(|evidence| evidence.role())
            .collect::<Vec<_>>();
        let valid = evidence_roles == expected
            && input.material.primary_graph_reads.iter().all(|evidence| {
                evidence.validates(
                    &self.bound.definition().semantics().canonical_query,
                    self.bound.basis().normalized().family(),
                    &input.execution_snapshot,
                    self.bound
                        .operation()
                        .domain_authority()
                        .runtime_authority(),
                )
            });
        if !valid {
            return Err(input.denial(
                self.counters,
                WorthQueryWorkflowAdvanceDenialKind::PrimaryReadEvidence,
            ));
        }
        self.counters.graph_read_contacts += input.material.primary_graph_reads.len();
        Ok(())
    }

    fn validate_effects(
        &mut self,
        stage: &worth_query_installation::facade::WorthQueryPortableWorkflowStage,
        input: &mut WorthQueryWorkflowStageValidationInput,
    ) -> Result<(), WorthQueryWorkflowAdvanceDenial> {
        input.material.effects.sort_by_key(|effect| effect.family());
        let mut expected = stage.semantics().effect_roles.clone();
        expected.sort();
        let evidence = input
            .material
            .effects
            .iter()
            .map(|effect| effect.family())
            .collect::<Vec<_>>();
        let valid = evidence == expected
            && input.material.effects.iter().all(|effect| {
                effect.binds_workflow(
                    &input.effect_workflow_binding,
                    self.bound.basis().normalized().family(),
                )
            });
        if !valid {
            return Err(input.denial(
                self.counters,
                WorthQueryWorkflowAdvanceDenialKind::EffectEvidence,
            ));
        }
        self.counters.effect_receipt_checks += input.material.effects.len();
        Ok(())
    }

    fn validate_invariants(
        &mut self,
        stage: &worth_query_installation::facade::WorthQueryPortableWorkflowStage,
        input: &WorthQueryWorkflowStageValidationInput,
    ) -> Result<Vec<WorthQueryWorkflowInvariantOutcome>, WorthQueryWorkflowAdvanceDenial> {
        if !stage.semantics().invariant_roles.is_empty() && input.material.effects.is_empty() {
            return Err(input.denial(
                self.counters,
                WorthQueryWorkflowAdvanceDenialKind::InvariantEvidence,
            ));
        }
        let outcomes = stage
            .semantics()
            .invariant_roles
            .iter()
            .map(|role| {
                let installed_identity = self
                    .bound
                    .operation()
                    .domain_authority()
                    .installed_invariant_identity(role)
                    .ok_or_else(|| {
                        input.denial(
                            self.counters,
                            WorthQueryWorkflowAdvanceDenialKind::InvariantEvidence,
                        )
                    })?;
                Ok(WorthQueryWorkflowInvariantOutcome::from_query_commits(
                    role,
                    installed_identity,
                    &input.material.effects,
                ))
            })
            .collect::<Result<Vec<_>, WorthQueryWorkflowAdvanceDenial>>()?;
        self.counters.invariant_checks += outcomes.len();
        Ok(outcomes)
    }

    fn validate_result_contracts(
        &mut self,
        stage: &worth_query_installation::facade::WorthQueryPortableWorkflowStage,
        input: &WorthQueryWorkflowStageValidationInput,
    ) -> Result<(), WorthQueryWorkflowAdvanceDenial> {
        self.counters.output_contract_checks += 1;
        if !input.material.output.satisfies(&stage.semantics().output) {
            return Err(input.denial(
                self.counters,
                WorthQueryWorkflowAdvanceDenialKind::OutputContract,
            ));
        }
        if let Err(denial) = self.validate_artifact_output(stage, &input.material.output) {
            return Err(input.denial(
                self.counters,
                WorthQueryWorkflowAdvanceDenialKind::ArtifactCarriage(denial),
            ));
        }
        self.counters.terminal_contract_checks += 1;
        let state = input.material.result_state;
        if stage.is_terminal() != state.is_some()
            || state.is_some_and(|state| !stage.semantics().terminal_result_states.contains(&state))
        {
            return Err(input.denial(
                self.counters,
                WorthQueryWorkflowAdvanceDenialKind::TerminalContract,
            ));
        }
        Ok(())
    }

    fn validate_artifact_output(
        &self,
        stage: &worth_query_installation::facade::WorthQueryPortableWorkflowStage,
        output: &super::WorthQueryWorkflowValue,
    ) -> Result<(), crate::domain_installation::WorthQueryArtifactDenial> {
        let worth_query_installation::facade::WorthQueryWorkflowValueContract::InstalledArtifact(_) =
            &stage.semantics().output
        else {
            return Ok(());
        };
        let super::WorthQueryWorkflowValue::InstalledArtifact(handle) = output else {
            return Err(crate::domain_installation::WorthQueryArtifactDenial::new(
                crate::domain_installation::WorthQueryArtifactDenialKind::StageMismatch,
                None,
                "artifact workflow output must be an owned managed handle",
            ));
        };
        let expected_contract = self
            .graph
            .artifact_contracts(stage.identity())
            .and_then(super::WorthQueryInstalledWorkflowArtifactContracts::output)
            .cloned()
            .ok_or_else(|| {
                crate::domain_installation::WorthQueryArtifactDenial::new(
                    crate::domain_installation::WorthQueryArtifactDenialKind::ArtifactContractNotInstalled,
                    None,
                    "workflow stage has no installed artifact output authority",
                )
            })?;
        let admission = crate::domain_installation::WorthQueryArtifactTransferAdmission::mint(
            super::WorthQueryArtifactTransferAdmissionParts {
                expected_contract,
                domain_authority: std::sync::Arc::clone(self.bound.operation().domain_authority()),
                operation_identity: self.bound.definition().canonical_identity().to_owned(),
                binding_identity: self.bound.binding_identity().to_owned(),
                run_identity: self.identity.clone(),
                predecessor_stage: stage.identity().to_owned(),
                consumer_stage: stage.identity().to_owned(),
                basis_identity: self.bound.basis().capability_digest().to_owned(),
            },
        );
        handle.validate_output(&admission)
    }

    fn validate_cost_contract(
        &self,
        stage: &worth_query_installation::facade::WorthQueryPortableWorkflowStage,
        stage_counters: WorthQueryWorkflowRunCounters,
        input: &WorthQueryWorkflowStageValidationInput,
    ) -> Result<(), WorthQueryWorkflowAdvanceDenial> {
        let mut declared = stage.semantics().cost_roles.clone();
        declared.sort();
        (declared == stage_counters.observed_cost_roles())
            .then_some(())
            .ok_or_else(|| {
                input.denial(
                    self.counters,
                    WorthQueryWorkflowAdvanceDenialKind::CostContract,
                )
            })
    }
}
