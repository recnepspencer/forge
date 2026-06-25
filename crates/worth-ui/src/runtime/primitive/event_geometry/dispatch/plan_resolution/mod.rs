mod activation_posture;
mod candidate_selection;
mod outcome_resolution;
mod region_hit_count;

pub(super) use activation_posture::{primary_activation_bubbles, region_activation_is_eligible};
pub(super) use candidate_selection::{candidate_for_region, CandidateSelectionMode};
pub(super) use outcome_resolution::dispatch_outcome;
pub(super) use region_hit_count::candidates_hit_count;
