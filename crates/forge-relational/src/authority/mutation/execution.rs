use crate::config::data::MutationConfig;
use crate::schema::data::RelationalSchemaRegistry;
use crate::storage::overlay::WorkingState;
use crate::symbols::data::StringInterner;
use crate::transactions::data::{AuthoritativeApplyPlan, CommitConflict};

use super::effect_assembly::assemble_effect;
use super::intents::dispatch_intent;
use super::{MutationEffect, MutationPreparationTelemetry, MutationWorkspace};

pub(crate) struct MutationApplyOutcome {
    pub(crate) effect: MutationEffect,
    pub(crate) preparation_telemetry: MutationPreparationTelemetry,
}

pub(crate) fn apply_plan_to_working_state(
    state: &mut WorkingState,
    apply_plan: &AuthoritativeApplyPlan,
    config: &MutationConfig,
    schema_registry: &RelationalSchemaRegistry,
    symbols: &mut StringInterner,
) -> Result<MutationApplyOutcome, CommitConflict> {
    let mut workspace = MutationWorkspace::new(
        state,
        symbols,
        config,
        schema_registry,
        apply_plan.version_id,
    );
    let mut effect = MutationEffect::default();

    for intent in &apply_plan.merged_intents {
        let child = dispatch_intent(intent, &mut workspace)?;
        effect.accumulate(assemble_effect(child, &mut workspace));
    }

    Ok(MutationApplyOutcome {
        effect,
        preparation_telemetry: workspace.preparation_telemetry(),
    })
}
