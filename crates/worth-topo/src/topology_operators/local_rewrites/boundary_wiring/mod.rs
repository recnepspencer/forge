pub(super) mod adjacency_support;
mod composed_successor_program;
mod membership;
mod relation_update;
#[cfg(test)]
mod successor_admission;

#[cfg(test)]
pub(crate) use membership::supports_admitted_relation_create_program;
pub(crate) use relation_update::ResolvedLoopSuccessorRewire;
#[cfg(test)]
pub(crate) use successor_admission::supports_admitted_loop_successor_program;
