use crate::config::data::MutationConfig;
use crate::schema::data::RelationalSchemaRegistry;
use crate::storage::overlay::RelationalDraft;
use crate::symbols::data::StringInterner;
use crate::transactions::data::{AuthoritativeApplyPlan, CommitConflict};

use super::intents::dispatch_intent;
use super::record_changes::apply_adjacency_deltas;
use super::{MutationEffect, MutationWorkspace};

pub(crate) fn apply_plan_to_draft(
    draft: &mut RelationalDraft,
    apply_plan: &AuthoritativeApplyPlan,
    config: &MutationConfig,
    schema_registry: &RelationalSchemaRegistry,
    symbols: &mut StringInterner,
) -> Result<MutationEffect, CommitConflict> {
    let mut workspace = MutationWorkspace {
        draft,
        symbols,
        config,
        schema: schema_registry,
        version_id: apply_plan.version_id,
    };
    let mut effect = MutationEffect::default();

    for intent in &apply_plan.merged_intents {
        let child = dispatch_intent(intent, &mut workspace)?;
        apply_adjacency_deltas(workspace.draft, &child.adjacency_deltas);
        effect.accumulate(child);
    }

    Ok(effect)
}
