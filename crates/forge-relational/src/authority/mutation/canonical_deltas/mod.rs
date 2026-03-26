mod data;
mod engine;
mod evidence;

pub(crate) use data::{CanonicalDeltaError, CanonicalRecordAspectDelta};
pub(crate) use engine::canonical_delta_for_mutation;
