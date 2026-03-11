mod aspect_versions;
mod execution;
mod intents;
mod patch_details;
mod record_changes;
mod stale_targets;
mod types;

pub(crate) use execution::apply_plan_to_draft;
pub(crate) use types::{AdjacencyDelta, AdjacencyDeltaKind, MutationEffect, MutationWorkspace};
