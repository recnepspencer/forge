use worth_relational::facade::runtime::RelationalRuntime;
use worth_relational::facade::transactions::MergeExecutionOutcome;

pub use super::execution_artifacts::{
    EffectExecutionAuthority, EffectExecutionDeferred, EffectExecutionDenial,
    EffectExecutionDenialKind, EffectExecutionSettlementDeferred, EffectExecutionStop,
    ExecutedEffectAuthorityArtifact, ExecutedEffectPlan,
};
use super::execution_bridge::execute_lowered_writeback;
use super::execution_relational_scalar::execute_lowered_mutation;
use super::lowering::{LoweredEffectExecutionArtifact, LoweredEffectExecutionPlan};
use super::receipt::EffectExecutionReceipt;
use super::RelationalEffectExecutionFailure;
impl LoweredEffectExecutionPlan {
    pub fn execute_with(
        self,
        authority: EffectExecutionAuthority<'_>,
    ) -> Result<ExecutedEffectPlan, EffectExecutionStop> {
        execute_lowered_effect_plan(self, authority)
    }

    pub fn execute_receipt_with(
        self,
        authority: EffectExecutionAuthority<'_>,
    ) -> Result<EffectExecutionReceipt, EffectExecutionStop> {
        self.execute_with(authority)
            .map(|executed| executed.receipt())
    }
}

pub fn execute_lowered_effect_plan(
    lowered: LoweredEffectExecutionPlan,
    mut authority: EffectExecutionAuthority<'_>,
) -> Result<ExecutedEffectPlan, EffectExecutionStop> {
    execute_lowered_effect_plan_with_authority(lowered, &mut authority)
}

pub(crate) fn execute_lowered_effect_plan_with_authority(
    lowered: LoweredEffectExecutionPlan,
    authority: &mut EffectExecutionAuthority<'_>,
) -> Result<ExecutedEffectPlan, EffectExecutionStop> {
    match lowered.artifact() {
        LoweredEffectExecutionArtifact::Mutation(declaration) => {
            if !authority.has_relational_authority() && authority.has_bridge_authority() {
                return Err(EffectExecutionStop::Denied(EffectExecutionDenial::new(
                    &lowered,
                    EffectExecutionDenialKind::AuthorityOverrideRejected,
                    "lowered relational mutation execution rejected bridge host override; the admitted lowered plan requires relational authority",
                )));
            }
            let runtime = authority.relational_runtime().ok_or_else(|| {
                EffectExecutionStop::Denied(EffectExecutionDenial::new(
                    &lowered,
                    EffectExecutionDenialKind::MissingRelationalAuthority,
                    "lowered relational mutation execution requires a relational runtime authority",
                ))
            })?;
            let commit = execute_lowered_mutation(runtime, declaration)
                .map_err(|failure| lower_relational_stop(&lowered, failure))?;
            Ok(ExecutedEffectPlan::new(
                lowered,
                ExecutedEffectAuthorityArtifact::Mutation(commit),
                1,
            ))
        }
        LoweredEffectExecutionArtifact::Merge(declaration) => {
            if !authority.has_relational_authority() && authority.has_bridge_authority() {
                return Err(EffectExecutionStop::Denied(EffectExecutionDenial::new(
                    &lowered,
                    EffectExecutionDenialKind::AuthorityOverrideRejected,
                    "lowered relational merge execution rejected bridge host override; the admitted lowered plan requires relational authority",
                )));
            }
            let runtime = authority.relational_runtime().ok_or_else(|| {
                EffectExecutionStop::Denied(EffectExecutionDenial::new(
                    &lowered,
                    EffectExecutionDenialKind::MissingRelationalAuthority,
                    "lowered relational merge execution requires a relational runtime authority",
                ))
            })?;
            let outcome = execute_lowered_merge(runtime, declaration)
                .map_err(|failure| lower_relational_stop(&lowered, failure))?;
            Ok(ExecutedEffectPlan::new(
                lowered,
                ExecutedEffectAuthorityArtifact::Merge(outcome),
                1,
            ))
        }
        LoweredEffectExecutionArtifact::Writeback(declaration) => {
            if !authority.has_bridge_authority() && authority.has_relational_authority() {
                return Err(EffectExecutionStop::Denied(EffectExecutionDenial::new(
                    &lowered,
                    EffectExecutionDenialKind::AuthorityOverrideRejected,
                    "lowered query writeback execution rejected relational host override; the admitted lowered plan requires bridge authority",
                )));
            }
            let runtime = authority.bridge_runtime().ok_or_else(|| {
                EffectExecutionStop::Denied(EffectExecutionDenial::new(
                    &lowered,
                    EffectExecutionDenialKind::MissingBridgeAuthority,
                    "lowered query writeback execution requires a runtime bridge authority",
                ))
            })?;
            let execution =
                execute_lowered_writeback(runtime, declaration).map_err(|(kind, message)| {
                    EffectExecutionStop::Denied(EffectExecutionDenial::new(&lowered, kind, message))
                })?;
            Ok(ExecutedEffectPlan::new(
                lowered,
                ExecutedEffectAuthorityArtifact::Writeback { execution },
                1,
            ))
        }
    }
}

fn lower_relational_stop(
    lowered: &LoweredEffectExecutionPlan,
    failure: RelationalEffectExecutionFailure,
) -> EffectExecutionStop {
    match failure {
        RelationalEffectExecutionFailure::Deferred { message } => {
            EffectExecutionStop::Deferred(EffectExecutionDeferred::new(lowered, message))
        }
        RelationalEffectExecutionFailure::Denied { kind, message } => {
            EffectExecutionStop::Denied(EffectExecutionDenial::new(lowered, kind, message))
        }
        RelationalEffectExecutionFailure::SettlementDeferred(deferred) => {
            let (message, settlement) = deferred.into_parts();
            EffectExecutionStop::SettlementDeferred(EffectExecutionSettlementDeferred::new(
                lowered, message, settlement,
            ))
        }
    }
}

pub(crate) fn execute_lowered_merge(
    runtime: &mut RelationalRuntime,
    declaration: &crate::workflow::LoweredMergeWorkflowDeclaration,
) -> Result<MergeExecutionOutcome, RelationalEffectExecutionFailure> {
    let prepared = runtime
        .bind_merge_execution_request(declaration.merge_request().clone())
        .map_err(|error| {
            (
                EffectExecutionDenialKind::MergePreparationFailed,
                format!("{error:?}"),
            )
        })
        .and_then(|bound| {
            runtime.prepare_merge_execution(bound).map_err(|error| {
                lower_runtime_error(error, EffectExecutionDenialKind::MergePreparationFailed)
            })
        })?;
    runtime.execute_prepared_merge(prepared).map_err(|error| {
        let settlement = match &error {
            worth_relational::facade::merge::MergeExecutionError::Commit(error) => {
                error.deferred_settlement().cloned()
            }
            _ => None,
        };
        let (kind, message) =
            lower_runtime_error(error, EffectExecutionDenialKind::MergeExecutionFailed);
        RelationalEffectExecutionFailure::from_publication_failure(kind, message, settlement)
    })
}

pub(super) fn lower_runtime_error(
    error: impl std::fmt::Debug,
    kind: EffectExecutionDenialKind,
) -> (EffectExecutionDenialKind, String) {
    (kind, format!("{error:?}"))
}
