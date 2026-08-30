//! Independent Milestone 3.12 change and ordering models.
//!
//! These models intentionally import no production WORTH UI semantics. Later
//! phases compare production-issued evidence with these independently authored
//! expectations.

#[path = "observation_rebind/cost_capacity.rs"]
mod cost_capacity;
#[path = "observation_rebind/effect_recovery.rs"]
mod effect_recovery;
#[path = "observation_rebind/identity_model.rs"]
mod identity_model;
#[path = "observation_rebind/lifecycle_cleanup.rs"]
pub(crate) mod lifecycle_cleanup;
#[path = "observation_rebind/mixed_source.rs"]
mod mixed_source;
#[path = "observation_rebind/model.rs"]
mod model;
#[path = "observation_rebind/ordering_model.rs"]
mod ordering_model;
#[path = "observation_rebind/query_consequence.rs"]
mod query_consequence;
#[path = "observation_rebind/scroll_runtime.rs"]
mod scroll_runtime;
#[path = "observation_rebind/semantic_pixel_independence.rs"]
mod semantic_pixel_independence;
#[path = "observation_rebind/source_affinity.rs"]
mod source_affinity;
#[path = "observation_rebind/stale_plan.rs"]
mod stale_plan;
#[path = "observation_rebind/support.rs"]
pub(crate) mod support;
#[path = "observation_rebind/terminal_outcomes.rs"]
mod terminal_outcomes;
