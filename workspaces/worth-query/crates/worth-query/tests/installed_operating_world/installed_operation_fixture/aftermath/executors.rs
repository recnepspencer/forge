use worth_query::facade::{domain, foundation, runtime};

use super::{
    AftermathCandidate, AftermathFamily, AftermathOriginal, GeometryDomain, ProvisionalWorkflow,
};

#[derive(Clone, Copy)]
pub(super) struct OriginalExecutor;

#[derive(Clone, Copy)]
pub(super) struct CandidateExecutor {
    wrong_inverse_target: bool,
    fail_after_effect: bool,
}

impl CandidateExecutor {
    pub(super) const fn new(wrong_inverse_target: bool, fail_after_effect: bool) -> Self {
        Self {
            wrong_inverse_target,
            fail_after_effect,
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct ProvisionalExecutor;

impl
    domain::WorthQueryDomainWorkflowStageExecutor<
        GeometryDomain,
        AftermathOriginal,
        AftermathFamily,
    > for OriginalExecutor
{
    const LOWERING_FAMILY: &'static str = "aftermath-mutation-v1";
    const DETERMINISTIC: bool = true;
    const EXECUTION_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;
    const RESULT_WIDTH_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;

    fn execute_stage(
        &self,
        input: domain::WorthQueryWorkflowValue,
        context: &domain::WorthQueryWorkflowStageExecutionContext<'_>,
        workspace: &mut domain::WorthQueryWorkflowStageWorkspace<'_>,
    ) -> Result<
        domain::WorthQueryWorkflowStageMaterial,
        domain::WorthQueryWorkflowStageExecutorFailure,
    > {
        execute_mutation("aftermath-original", input, context, workspace)
    }
}

impl
    domain::WorthQueryDomainWorkflowStageExecutor<
        GeometryDomain,
        AftermathCandidate,
        AftermathFamily,
    > for CandidateExecutor
{
    const LOWERING_FAMILY: &'static str = "aftermath-mutation-v1";
    const DETERMINISTIC: bool = true;
    const EXECUTION_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;
    const RESULT_WIDTH_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;

    fn execute_stage(
        &self,
        input: domain::WorthQueryWorkflowValue,
        context: &domain::WorthQueryWorkflowStageExecutionContext<'_>,
        workspace: &mut domain::WorthQueryWorkflowStageWorkspace<'_>,
    ) -> Result<
        domain::WorthQueryWorkflowStageMaterial,
        domain::WorthQueryWorkflowStageExecutorFailure,
    > {
        if let domain::WorthQueryWorkflowValue::EntityIdentity(value) = &input {
            if let Some(identity) = exact_inverse_identity(value) {
                return execute_exact_inverse(identity, context, workspace);
            }
        }
        execute_mutation("aftermath-candidate", input, context, workspace)
    }

    fn prepare_aftermath_intent(
        &self,
        original: &domain::WorthQueryAftermathOriginalEvidence,
    ) -> Option<domain::WorthQueryNormalizedWorkflowIntent> {
        let value = match original.kind() {
            domain::WorthQueryAftermathKind::ExactInverse => {
                let parts = original
                    .effect_target(0)?
                    .relational_entity_record_parts()?;
                let local_slot = if self.wrong_inverse_target {
                    parts.local_slot().saturating_sub(1)
                } else {
                    parts.local_slot()
                };
                format!(
                    "exact:{}:{}:{}",
                    parts.partition_id(),
                    local_slot,
                    parts.generation()
                )
            }
            domain::WorthQueryAftermathKind::Compensation if self.fail_after_effect => {
                "fail-after-effect".into()
            }
            domain::WorthQueryAftermathKind::Compensation => "apply".into(),
        };
        domain::WorthQueryNormalizedWorkflowIntent::new(vec![
            domain::WorthQueryWorkflowIntentStage::new(
                "apply",
                domain::WorthQueryWorkflowIntentValue::EntityIdentity(value),
            ),
        ])
        .ok()
    }

    fn verify_aftermath_postcondition(
        &self,
        original: &domain::WorthQueryAftermathOriginalEvidence,
        candidate: &domain::WorthQueryWorkflowTraceSemantics,
    ) -> bool {
        let expected = match original.postcondition() {
            domain::WorthQueryAftermathPostcondition::ExactPriorTruth => {
                return exact_inverse_is_evidenced(original, candidate);
            }
            domain::WorthQueryAftermathPostcondition::BusinessPostcondition { identity }
                if identity == "original-obligation-settled" =>
            {
                "effect-applied"
            }
            _ => return false,
        };
        candidate.stages().iter().any(|stage| {
            !stage.effects().is_empty()
                && stage.output() == &domain::WorthQueryWorkflowSemanticValue::Text(expected.into())
        })
    }
}

fn exact_inverse_is_evidenced(
    original: &domain::WorthQueryAftermathOriginalEvidence,
    candidate: &domain::WorthQueryWorkflowTraceSemantics,
) -> bool {
    let Some(original_receipt) = original
        .effect(0)
        .and_then(domain::WorthQueryWorkflowEffectEvidence::mutation_receipt)
    else {
        return false;
    };
    let Some(candidate_mutation) = candidate
        .stages()
        .iter()
        .flat_map(domain::WorthQueryWorkflowStageTraceSemantics::effects)
        .find_map(|effect| effect.mutation())
    else {
        return false;
    };
    original.effect_count() == 1
        && original_receipt.target_entity_identity() == candidate_mutation.target_entity()
        && original_receipt
            .target_collection_identity()
            .map(runtime::WorthQueryMutationTargetCollectionIdentity::as_str)
            == candidate_mutation.target_collection()
        && original_receipt.deltas().len() == candidate_mutation.deltas().len()
        && original_receipt
            .deltas()
            .iter()
            .zip(candidate_mutation.deltas())
            .all(|(original, candidate)| {
                original.entity_identity() == candidate.entity_identity()
                    && original.target_collection_identity()
                        == candidate.target_collection_identity()
                    && matches!(
                        (original.kind(), candidate.kind()),
                        (
                            foundation::WorthQueryMutationKind::Created,
                            foundation::WorthQueryMutationKind::Deleted
                        )
                    )
            })
}

impl
    domain::WorthQueryDomainWorkflowStageExecutor<
        GeometryDomain,
        ProvisionalWorkflow,
        AftermathFamily,
    > for ProvisionalExecutor
{
    const LOWERING_FAMILY: &'static str = "provisional-workflow-v1";
    const DETERMINISTIC: bool = true;
    const EXECUTION_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;
    const RESULT_WIDTH_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;

    fn execute_stage(
        &self,
        _input: domain::WorthQueryWorkflowValue,
        _context: &domain::WorthQueryWorkflowStageExecutionContext<'_>,
        _workspace: &mut domain::WorthQueryWorkflowStageWorkspace<'_>,
    ) -> Result<
        domain::WorthQueryWorkflowStageMaterial,
        domain::WorthQueryWorkflowStageExecutorFailure,
    > {
        Ok(
            domain::WorthQueryWorkflowStageMaterial::new(domain::WorthQueryWorkflowValue::Text(
                "provisional-result".into(),
            ))
            .with_result_state(domain::WorthQueryOperationResultState::Ready),
        )
    }
}

fn execute_mutation(
    entity: &str,
    input: domain::WorthQueryWorkflowValue,
    context: &domain::WorthQueryWorkflowStageExecutionContext<'_>,
    workspace: &mut domain::WorthQueryWorkflowStageWorkspace<'_>,
) -> Result<domain::WorthQueryWorkflowStageMaterial, domain::WorthQueryWorkflowStageExecutorFailure>
{
    let command = runtime::WorthQueryAspectMutationBuilder::new()
        .aspect("identity.id", entity)
        .build_insert("Vertex")
        .map_err(|detail| {
            stage_failure(
                domain::WorthQueryOperationFailureClass::InvalidInput,
                detail,
            )
        })?;
    context
        .execute_mutation(command, workspace)
        .map_err(|denial| {
            stage_failure(domain::WorthQueryOperationFailureClass::Dependency, denial)
        })?;
    if matches!(input, domain::WorthQueryWorkflowValue::EntityIdentity(value) if value == "fail-after-effect")
    {
        return Err(domain::WorthQueryWorkflowStageExecutorFailure::new(
            domain::WorthQueryOperationFailureClass::Dependency,
            "declared aftermath failure after mutation",
        ));
    }
    Ok(stage_material("effect-applied"))
}

fn execute_exact_inverse(
    original: foundation::WorthQueryEntityIdentity,
    context: &domain::WorthQueryWorkflowStageExecutionContext<'_>,
    workspace: &mut domain::WorthQueryWorkflowStageWorkspace<'_>,
) -> Result<domain::WorthQueryWorkflowStageMaterial, domain::WorthQueryWorkflowStageExecutorFailure>
{
    let command = runtime::WorthQueryDeleteMutationBuilder::new()
        .target_collection("Vertex")
        .build_delete(original)
        .map_err(|detail| {
            stage_failure(
                domain::WorthQueryOperationFailureClass::InvalidInput,
                detail,
            )
        })?;
    context
        .execute_mutation(command, workspace)
        .map_err(|denial| {
            stage_failure(domain::WorthQueryOperationFailureClass::Dependency, denial)
        })?;
    Ok(stage_material("prior-truth-restored"))
}

fn exact_inverse_identity(value: &str) -> Option<foundation::WorthQueryEntityIdentity> {
    let mut fields = value.strip_prefix("exact:")?.split(':');
    let partition_id = fields.next()?.parse().ok()?;
    let local_slot = fields.next()?.parse().ok()?;
    let generation = fields.next()?.parse().ok()?;
    if fields.next().is_some() {
        return None;
    }
    Some(
        foundation::WorthQueryEntityIdentity::from_relational_record(
            foundation::RelationalBridgeRecordIdentityParts::entity(
                partition_id,
                local_slot,
                generation,
            ),
        ),
    )
}

fn stage_failure(
    class: domain::WorthQueryOperationFailureClass,
    detail: impl std::fmt::Debug,
) -> domain::WorthQueryWorkflowStageExecutorFailure {
    domain::WorthQueryWorkflowStageExecutorFailure::new(class, format!("{detail:?}"))
}

fn stage_material(value: &str) -> domain::WorthQueryWorkflowStageMaterial {
    domain::WorthQueryWorkflowStageMaterial::new(domain::WorthQueryWorkflowValue::Text(
        value.into(),
    ))
    .with_result_state(domain::WorthQueryOperationResultState::Ready)
}
