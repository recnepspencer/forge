use super::{
    WorthQueryBoundGraphExecutionReceipt, WorthQueryWorkflowEffectEvidence,
    WorthQueryWorkflowPredecessorReceipt, WorthQueryWorkflowStageEffectDenial,
    WorthQueryWorkflowStageExecutorFailure, WorthQueryWorkflowStageReceipt,
    WorthQueryWorkflowStageWorkspace,
};

pub struct WorthQueryWorkflowStageExecutionContext<'a> {
    operation_identity: &'a str,
    run_identity: &'a str,
    stage: &'a worth_query_installation::facade::WorthQueryPortableWorkflowStage,
    predecessor_receipts: Vec<WorthQueryWorkflowPredecessorReceipt<'a>>,
    effect_workflow_binding: crate::workflow::WorkflowContextBinding,
    basis: crate::basis_lifecycle::BasisFamily,
    installed_read: Option<&'a crate::ordinary::read::WorthQueryReadDeclaration>,
    operation_graph_reads:
        &'a [worth_query_installation::facade::WorthQueryOperationGraphReadRole],
    graph_receipts: &'a [WorthQueryBoundGraphExecutionReceipt],
    query_authority: crate::identity_authority::QueryCanonicalAuthority,
    identity_evolution_basis_identity: String,
}

pub(crate) struct WorthQueryWorkflowStageExecutionAuthority<'a> {
    pub(crate) effect_workflow_binding: crate::workflow::WorkflowContextBinding,
    pub(crate) basis: crate::basis_lifecycle::BasisFamily,
    pub(crate) installed_read: Option<&'a crate::ordinary::read::WorthQueryReadDeclaration>,
    pub(crate) operation_graph_reads:
        &'a [worth_query_installation::facade::WorthQueryOperationGraphReadRole],
    pub(crate) graph_receipts: &'a [WorthQueryBoundGraphExecutionReceipt],
    pub(crate) query_authority: crate::identity_authority::QueryCanonicalAuthority,
    pub(crate) identity_evolution_basis_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowStageLineageDenial {
    RuntimeMutationEvidenceRequired,
    AuthoritativeContinuityEvidenceRequired,
    IdentityEvolutionAdmission(crate::identity_evolution::IdentityEvolutionAdmissionError),
    IdentityEvolutionOutcomeMismatch,
}

impl<'a> WorthQueryWorkflowStageExecutionContext<'a> {
    pub(crate) fn new(
        operation_identity: &'a str,
        run_identity: &'a str,
        stage: &'a worth_query_installation::facade::WorthQueryPortableWorkflowStage,
        predecessor_receipts: &'a [&'a WorthQueryWorkflowStageReceipt],
        authority: WorthQueryWorkflowStageExecutionAuthority<'a>,
    ) -> Self {
        Self {
            operation_identity,
            run_identity,
            stage,
            predecessor_receipts: predecessor_receipts
                .iter()
                .map(|receipt| WorthQueryWorkflowPredecessorReceipt::new(receipt))
                .collect(),
            effect_workflow_binding: authority.effect_workflow_binding,
            basis: authority.basis,
            installed_read: authority.installed_read,
            operation_graph_reads: authority.operation_graph_reads,
            graph_receipts: authority.graph_receipts,
            query_authority: authority.query_authority,
            identity_evolution_basis_identity: authority.identity_evolution_basis_identity,
        }
    }

    pub fn operation_identity(&self) -> &str {
        self.operation_identity
    }
    pub fn run_identity(&self) -> &str {
        self.run_identity
    }
    pub fn stage(&self) -> &worth_query_installation::facade::WorthQueryPortableWorkflowStage {
        self.stage
    }
    pub fn predecessor_receipts(&self) -> &[WorthQueryWorkflowPredecessorReceipt<'a>] {
        &self.predecessor_receipts
    }
    pub(crate) fn effect_workflow_binding(&self) -> &crate::workflow::WorkflowContextBinding {
        &self.effect_workflow_binding
    }

    pub fn execute_identity_evolution(
        &self,
        establishing_effect: &WorthQueryWorkflowEffectEvidence,
    ) -> Result<
        crate::identity_evolution::InstalledIdentityEvolutionOutcome,
        WorthQueryWorkflowStageLineageDenial,
    > {
        let mutation_receipt = establishing_effect
            .mutation_receipt()
            .ok_or(WorthQueryWorkflowStageLineageDenial::RuntimeMutationEvidenceRequired)?;
        let continuity = mutation_receipt.continuity_mutation_evidence();
        let (descriptor, lifecycle_target) = match mutation_receipt.mutation_family() {
            crate::runtime::WorthQueryMutationFamily::Insert => {
                let target = mutation_receipt.target_entity_identity().ok_or(
                    WorthQueryWorkflowStageLineageDenial::AuthoritativeContinuityEvidenceRequired,
                )?;
                (
                    crate::identity_evolution::LineageTraversalDescriptor::generated_identity(
                        target.evidence_identity().as_str().to_owned(),
                    ),
                    Some(target.clone()),
                )
            }
            crate::runtime::WorthQueryMutationFamily::Delete => {
                let target = mutation_receipt.target_entity_identity().ok_or(
                    WorthQueryWorkflowStageLineageDenial::AuthoritativeContinuityEvidenceRequired,
                )?;
                (
                    crate::identity_evolution::LineageTraversalDescriptor::retired_identity(
                        target.evidence_identity().as_str().to_owned(),
                    ),
                    Some(target.clone()),
                )
            }
            crate::runtime::WorthQueryMutationFamily::Update => (
                authoritative_continuity_descriptor(continuity.ok_or(
                    WorthQueryWorkflowStageLineageDenial::AuthoritativeContinuityEvidenceRequired,
                )?)?,
                None,
            ),
            crate::runtime::WorthQueryMutationFamily::Assertion => {
                return Err(
                    WorthQueryWorkflowStageLineageDenial::AuthoritativeContinuityEvidenceRequired,
                );
            }
        };
        let query =
            crate::identity_evolution::IdentityEvolutionQueryContext::installed_operation_lineage(
                &self.query_authority,
                self.operation_identity,
                &self.identity_evolution_basis_identity,
                descriptor,
            );
        let admitted = crate::identity_evolution::admit_identity_evolution_query(query)
            .map_err(WorthQueryWorkflowStageLineageDenial::IdentityEvolutionAdmission)?;
        let artifact =
            crate::identity_evolution::execute_admitted_identity_evolution_query(&admitted)
                .map_err(WorthQueryWorkflowStageLineageDenial::IdentityEvolutionAdmission)?;
        crate::identity_evolution::InstalledIdentityEvolutionOutcome::from_execution(
            artifact,
            continuity.cloned(),
            lifecycle_target,
            crate::identity_evolution::InstalledIdentityEvolutionBinding {
                operation_identity: self.operation_identity,
                run_identity: self.run_identity,
                stage_identity: self.stage.identity(),
                effect_receipt_identity: establishing_effect.receipt_identity().to_owned(),
                establishing_entity_targets: mutation_receipt
                    .deltas()
                    .iter()
                    .map(|delta| delta.entity_identity().clone())
                    .collect(),
            },
        )
        .ok_or(WorthQueryWorkflowStageLineageDenial::IdentityEvolutionOutcomeMismatch)
    }

    pub fn execute_identity_correspondence(
        &self,
        establishing_effect: &WorthQueryWorkflowEffectEvidence,
        observation: super::WorthQueryInstalledCorrespondenceObservation,
    ) -> Result<
        crate::identity_evolution::InstalledIdentityEvolutionOutcome,
        WorthQueryWorkflowStageLineageDenial,
    > {
        let mutation_receipt = establishing_effect
            .mutation_receipt()
            .ok_or(WorthQueryWorkflowStageLineageDenial::RuntimeMutationEvidenceRequired)?;
        let query =
            crate::identity_evolution::IdentityEvolutionQueryContext::installed_operation_correspondence(
                &self.query_authority,
                self.operation_identity,
                &self.identity_evolution_basis_identity,
                observation.into_engine_comparison(),
            );
        let admitted = crate::identity_evolution::admit_identity_evolution_query(query)
            .map_err(WorthQueryWorkflowStageLineageDenial::IdentityEvolutionAdmission)?;
        let artifact =
            crate::identity_evolution::execute_admitted_identity_evolution_query(&admitted)
                .map_err(WorthQueryWorkflowStageLineageDenial::IdentityEvolutionAdmission)?;
        crate::identity_evolution::InstalledIdentityEvolutionOutcome::from_execution(
            artifact,
            None,
            None,
            crate::identity_evolution::InstalledIdentityEvolutionBinding {
                operation_identity: self.operation_identity,
                run_identity: self.run_identity,
                stage_identity: self.stage.identity(),
                effect_receipt_identity: establishing_effect.receipt_identity().to_owned(),
                establishing_entity_targets: mutation_receipt
                    .deltas()
                    .iter()
                    .map(|delta| delta.entity_identity().clone())
                    .collect(),
            },
        )
        .ok_or(WorthQueryWorkflowStageLineageDenial::IdentityEvolutionOutcomeMismatch)
    }

    pub fn graph_projection(&self, role: &str) -> Option<&crate::runtime::WorthQueryReadResult> {
        if !self
            .stage
            .semantics()
            .graph_read_roles
            .iter()
            .any(|declared| declared == role)
        {
            return None;
        }
        self.graph_receipts
            .iter()
            .find(|receipt| {
                receipt.role() == role
                    && receipt.kind()
                        == crate::domain_installation::WorthQueryGraphProviderCallKind::Project
            })
            .and_then(WorthQueryBoundGraphExecutionReceipt::projection)
    }

    pub fn execute_mutation(
        &self,
        command: crate::runtime::WorthQueryWriteCommand,
        workspace: &mut WorthQueryWorkflowStageWorkspace<'_>,
    ) -> Result<WorthQueryWorkflowEffectEvidence, WorthQueryWorkflowStageEffectDenial> {
        if !self
            .stage
            .semantics()
            .effect_roles
            .contains(&worth_query_installation::facade::WorthQueryOperationEffectFamily::Mutation)
        {
            return Err(WorthQueryWorkflowStageEffectDenial::UndeclaredEffectFamily);
        }
        let execution = workspace
            .workspace
            .execute_ordinary_authoritative_mutation(command, false)
            .map_err(|error| WorthQueryWorkflowStageEffectDenial::Runtime(format!("{error:?}")))?;
        let evidence = WorthQueryWorkflowEffectEvidence::runtime_mutation(
            execution.into_receipt(),
            &self.effect_workflow_binding,
            self.basis,
        );
        workspace.executed_effects.push(evidence.clone());
        Ok(evidence)
    }

    pub fn execute_installed_read(
        &self,
        role: &str,
        workspace: &mut WorthQueryWorkflowStageWorkspace<'_>,
    ) -> Result<
        crate::ordinary::read::WorthQueryReadCompletion,
        WorthQueryWorkflowStageExecutorFailure,
    > {
        let role_is_admitted = self
            .stage
            .semantics()
            .graph_read_roles
            .iter()
            .any(|declared| declared == role)
            && self.operation_graph_reads.iter().any(|declared| {
                declared.role == role
                    && declared.participation
                        == worth_query_installation::facade::WorthQueryOperationGraphParticipation::PrimaryLogicalGraph
            });
        if !role_is_admitted {
            return Err(WorthQueryWorkflowStageExecutorFailure::new(
                crate::domain_installation::WorthQueryOperationFailureClass::Indeterminate,
                "workflow stage lacks the installed primary read role",
            ));
        }
        let declaration = self.installed_read.ok_or_else(|| {
            WorthQueryWorkflowStageExecutorFailure::new(
                crate::domain_installation::WorthQueryOperationFailureClass::Indeterminate,
                "workflow operation has no Query-installed read declaration",
            )
        })?;
        if workspace.installed_read_executions != 0 {
            return Err(WorthQueryWorkflowStageExecutorFailure::new(
                crate::domain_installation::WorthQueryOperationFailureClass::Indeterminate,
                "installed canonical read may execute only once per workflow stage",
            ));
        }
        workspace.installed_read_executions += 1;
        declaration
            .clone_for_installed_execution()
            .using(crate::ordinary::read::current())
            .run(workspace.workspace)
            .into_result()
            .map_err(|stop| {
                WorthQueryWorkflowStageExecutorFailure::new(
                    crate::domain_installation::WorthQueryOperationFailureClass::Dependency,
                    format!("{stop:?}"),
                )
            })
    }

    pub(crate) fn requires_primary_read(&self) -> bool {
        self.operation_graph_reads.iter().any(|declared| {
            self.stage
                .semantics()
                .graph_read_roles
                .contains(&declared.role)
                && declared.participation
                    == worth_query_installation::facade::WorthQueryOperationGraphParticipation::PrimaryLogicalGraph
        })
    }
}

fn authoritative_continuity_descriptor(
    continuity: &crate::runtime::WorthQueryContinuityMutationEvidence,
) -> Result<
    crate::identity_evolution::LineageTraversalDescriptor,
    WorthQueryWorkflowStageLineageDenial,
> {
    use crate::runtime::WorthQueryContinuityOutcomeClass as Outcome;

    let anchor = continuity
        .prior_authoritative_identity()
        .evidence_identity()
        .as_str()
        .to_owned();
    let successors = continuity
        .successor_authoritative_identities()
        .iter()
        .map(|identity| identity.evidence_identity().as_str().to_owned())
        .collect::<Vec<_>>();
    match continuity.outcome_class() {
        Outcome::ContinuesAsSingleSuccessor => Ok(
            crate::identity_evolution::LineageTraversalDescriptor::direct_successor_exact(
                anchor,
                successors[0].clone(),
            ),
        ),
        Outcome::ContinuesAsSplitSuccessors => Ok(
            crate::identity_evolution::LineageTraversalDescriptor::direct_split_successors_exact(
                anchor, successors,
            ),
        ),
        Outcome::ContinuesViaTruthLoweredCanonicalMergeSuccessor => Ok(
            crate::identity_evolution::LineageTraversalDescriptor::direct_merge_successor_exact(
                anchor,
                successors[0].clone(),
            ),
        ),
        Outcome::RejectedNoAuthoritativeSuccessor
        | Outcome::RejectedAmbiguousSuccessor
        | Outcome::RejectedUnsupportedContinuityClass
        | Outcome::RejectedHistoricalResolutionFailure => {
            Err(WorthQueryWorkflowStageLineageDenial::AuthoritativeContinuityEvidenceRequired)
        }
    }
}
