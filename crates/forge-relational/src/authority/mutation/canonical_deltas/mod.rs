mod aspect_trace_projection;
mod changed_authoritative_patch;
mod data;
mod engine;
mod lifecycle_transition_evidence;
mod materialized_state;
mod patch_authority;
mod patch_fragments;

#[cfg(test)]
mod engine_tests;
#[cfg(test)]
mod patch_fragments_tests;

pub(crate) use data::{CanonicalDeltaError, CanonicalRecordAspectDelta};
pub(crate) use engine::canonical_delta_for_mutation;
pub(crate) use patch_fragments::{
    authoritative_patch_with_delta_supplements, FoundationalPatchFragment,
};
