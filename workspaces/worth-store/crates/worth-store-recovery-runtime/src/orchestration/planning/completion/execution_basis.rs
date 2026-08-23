use crate::entry::{
    PhysicalRecoveryLimitDimension, PhysicalRecoveryLimitFailure, PhysicalRecoveryOutcome,
};
use crate::progression::{
    derive_execution_basis, ExecutionBasisDenial, RecoveryPublicationPlan, RecoveryQuiescencePlan,
    RecoveryStagingLayoutPlan,
};

use super::super::context::PlanningContext;
use super::super::resolved_basis::ResolvedPlanningBasis;

pub(super) struct ExecutionProducts {
    pub(super) staging: RecoveryStagingLayoutPlan,
    pub(super) publication: RecoveryPublicationPlan,
    pub(super) quiescence: RecoveryQuiescencePlan,
}

pub(super) fn derive(
    context: PlanningContext,
    basis: &ResolvedPlanningBasis,
) -> Result<(PlanningContext, ExecutionProducts), PhysicalRecoveryOutcome> {
    let (staging, publication, quiescence) = match derive_execution_basis(
        context.authority.media.store_identity(),
        &context.selection,
        &basis.sample,
        &basis.fates,
        &basis.redo,
        &basis.observed_pages.selected_source,
        context.limits.staging_bytes,
        context.limits.dirty_frames,
    ) {
        Ok(execution) => execution,
        Err(ExecutionBasisDenial::StagingBytes { observed }) => {
            let admitted = context.limits.staging_bytes;
            return Err(context.cost_denial_block(
                basis.planning_counters(),
                worth_store_recovery_physics::RecoveryPlanCostDenial::StagingBytes,
                PhysicalRecoveryLimitFailure {
                    dimension: PhysicalRecoveryLimitDimension::StagingBytes,
                    observed,
                    admitted,
                },
            ));
        }
        Err(ExecutionBasisDenial::DirtyFrames { observed }) => {
            let admitted = context.limits.dirty_frames;
            return Err(context.cost_denial_block(
                basis.planning_counters(),
                worth_store_recovery_physics::RecoveryPlanCostDenial::DirtyFrames,
                PhysicalRecoveryLimitFailure {
                    dimension: PhysicalRecoveryLimitDimension::DirtyFrames,
                    observed,
                    admitted,
                },
            ));
        }
        Err(ExecutionBasisDenial::Invalid) => {
            return Err(context.redo_block(basis.planning_counters(), None));
        }
    };
    Ok((
        context,
        ExecutionProducts {
            staging,
            publication,
            quiescence,
        },
    ))
}
