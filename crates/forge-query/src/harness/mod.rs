#[cfg(test)]
mod adapter;
#[cfg(test)]
mod admission;
#[cfg(test)]
mod aspect_api_finalization_certification;
#[cfg(test)]
mod binding;
#[cfg(test)]
mod certification;
#[cfg(test)]
mod collection_certification;
#[cfg(test)]
mod collection_matrix;
#[cfg(test)]
mod correspondence_history_certification;
#[cfg(test)]
pub(crate) mod fixtures;
#[cfg(test)]
mod frontier_certification;
#[cfg(test)]
mod frontier_planning;
#[cfg(test)]
mod historical_diff_certification;
#[cfg(test)]
mod identity_evolution_certification;
#[cfg(test)]
mod live_certification;
#[cfg(test)]
mod matrices;
#[cfg(test)]
mod milestone_eight_certification;
#[cfg(test)]
pub(crate) mod milestone_nine_certification;
#[cfg(test)]
pub(crate) mod milestone_nine_five_forbidden_fallback_closeout;
#[cfg(test)]
pub(crate) mod milestone_nine_five_hostile_matrix;
#[cfg(test)]
pub(crate) mod milestone_nine_six_identity_boundary_matrix;
#[cfg(test)]
pub(crate) mod milestone_nine_one_certification;
#[cfg(test)]
pub(crate) mod milestone_nine_three_certification;
#[cfg(test)]
pub(crate) mod milestone_nine_two_certification;
#[cfg(test)]
mod parity;
#[cfg(test)]
mod phase_layers;
#[cfg(test)]
mod planning;
#[cfg(test)]
mod planning_certification;
#[cfg(test)]
mod planning_matrix;
#[cfg(test)]
mod preview_certification;
#[cfg(test)]
mod profiles;
#[cfg(test)]
mod region_live_certification;
#[cfg(test)]
mod reporting;
#[cfg(test)]
mod runtime_api_stabilization;
#[cfg(test)]
mod semantics;
#[cfg(test)]
mod typed;
#[cfg(test)]
mod unified_facade_certification;
#[cfg(test)]
mod validation_cases;
#[cfg(test)]
mod validation_certification;
#[cfg(test)]
mod validation_matrix;
#[cfg(test)]
mod workflow_certification;

#[cfg(test)]
pub(crate) use preview_certification::MilestoneFivePointTwoPreviewCertificationAdapter;
#[cfg(test)]
pub(crate) use runtime_api_stabilization::RuntimeApiStabilizationAdapter;
