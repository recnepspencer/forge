mod classification;
mod evidence;
mod execution;
mod interleaving;
mod observation;
mod planning;

pub(crate) use evidence::{
    milestone_13_artifact_report, milestone_13_complexity_surface, milestone_13_counter_contract,
};
#[cfg(test)]
pub(crate) use execution::admit_inflight_cold_recall;
pub(crate) use execution::shared::{
    expected_verification_label, placement_family_for_artifact_key,
    recall_coalescing_key_for_artifact, recall_record_key,
};
pub(crate) use execution::{
    canonical_residency_manifest, cutover_tier_replica, execute_cold_recall,
    prepare_authoritative_tier_move, prepare_derived_tier_move, recover_tiering_state,
    retire_tier_replica, transfer_tier_replica, verify_tier_replica,
};
pub(crate) use interleaving::{
    observe_continuation_interleaving, observe_placement_read_interleaving,
    observe_stable_basis_interleaving, resolve_cold_recall_read_handle,
    resolve_resident_read_handle,
};
pub(crate) use observation::observe_working_set;
pub(crate) use planning::{
    plan_authoritative_tier_move, plan_broadened_recall, plan_cold_recall_lease,
    plan_derived_tier_move, plan_resident_read_lease, summarize_placement_demand,
};
