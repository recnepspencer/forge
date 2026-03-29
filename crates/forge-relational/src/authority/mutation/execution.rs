use crate::config::data::MutationConfig;
use crate::schema::data::{AspectPlanCatalog, RelationalSchemaRegistry};
use crate::storage::overlay::WorkingState;
use crate::symbols::data::StringInterner;
use crate::transactions::data::{AuthoritativeApplyPlan, CommitConflict};

use super::effect_assembly::assemble_effect;
use super::intents::dispatch_intent;
use super::{
    BranchLocalDeleteAllowance, MutationEffect, MutationPreparationTelemetry, MutationWorkspace,
};

pub(crate) struct MutationApplyOutcome {
    pub(crate) effect: MutationEffect,
    pub(crate) preparation_telemetry: MutationPreparationTelemetry,
}

pub(crate) fn apply_plan_to_working_state(
    state: &mut WorkingState,
    apply_plan: &AuthoritativeApplyPlan,
    config: &MutationConfig,
    schema_registry: &RelationalSchemaRegistry,
    aspect_plans: &AspectPlanCatalog,
    symbols: &mut StringInterner,
    branch_local_delete_allowance: BranchLocalDeleteAllowance,
) -> Result<MutationApplyOutcome, CommitConflict> {
    let mut workspace = MutationWorkspace::new(
        state,
        symbols,
        config,
        schema_registry,
        aspect_plans,
        apply_plan.version_id,
        branch_local_delete_allowance,
    );
    let mut effect = MutationEffect::default();

    for intent in &apply_plan.merged_intents {
        let child = dispatch_intent(intent, &mut workspace)?;
        effect.accumulate(assemble_effect(child, &mut workspace)?);
    }

    Ok(MutationApplyOutcome {
        effect,
        preparation_telemetry: workspace.preparation_telemetry(),
    })
}
