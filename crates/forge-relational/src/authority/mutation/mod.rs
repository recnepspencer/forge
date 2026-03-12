mod aspect_versions;
mod effect;
mod effect_assembly;
mod execution;
mod intents;
mod mutation_context;
mod outcomes;
mod patch_details;
mod record_changes;
mod stale_targets;
mod workspace;

pub(crate) use execution::apply_plan_to_working_state;
pub(crate) use record_changes::apply_adjacency_deltas;
pub(crate) use effect::{AdjacencyDelta, AdjacencyDeltaKind, MutationEffect};
pub(crate) use workspace::MutationWorkspace;
