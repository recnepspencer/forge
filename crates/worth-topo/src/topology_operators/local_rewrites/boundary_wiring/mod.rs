pub(super) mod adjacency_support;
mod composed_successor_program;
mod membership;
mod relation_update;
mod successor_admission;
mod successor_support;

pub(crate) use composed_successor_program::supports_composed_loop_successor_program;
pub(crate) use membership::supports_admitted_relation_create_program;
pub(crate) use relation_update::ResolvedLoopSuccessorRewire;
pub(crate) use successor_admission::supports_admitted_loop_successor_program;
