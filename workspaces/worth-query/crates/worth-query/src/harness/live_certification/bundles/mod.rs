mod assembly;
mod changes;
mod lane_bundles;
mod rejection_bundles;

pub(super) use lane_bundles::{
    bounded_materialization_patch_bundle, bounded_materialization_refresh_bundle,
    bounded_materialization_replay_bundle, bounded_materialization_replay_end_state_control_bundle,
    coalesced_delivery_bundle, detail_patch_bundle, detail_replay_bundle,
    detail_replay_end_state_control_bundle, detail_suppression_bundle,
    ordered_collection_patch_bundle, ordered_collection_replay_bundle,
    ordered_collection_replay_end_state_control_bundle, progress_advance_bundle,
};
pub(super) use rejection_bundles::{
    change_sequence_gap_rejection_bundle, forbidden_coalescing_rejection_bundle,
    forbidden_refresh_rejection_bundle, invalid_live_promotion_rejection_bundle,
    non_monotonic_sequence_rejection_bundle, raw_cdc_leakage_rejection_bundle,
    unsupported_live_family_rejection_bundle, unsupported_patch_family_rejection_bundle,
    width_overflow_rejection_bundle,
};

use super::super::certification::{ParityAnchor, RejectionCertificationRow};
use super::model::{
    LiveCertificationBundle, LiveCertificationRow, LiveHostileExpectation, LivePerturbationClass,
    LiveRejectionBundle, LiveRejectionRow,
};

pub(super) fn canonical_row(
    row_name: &'static str,
    perturbation_class: LivePerturbationClass,
    hostile_expectation: LiveHostileExpectation,
    control_lane: LiveCertificationBundle,
    hostile_lane: LiveCertificationBundle,
    parity_lane: LiveCertificationBundle,
) -> LiveCertificationRow {
    LiveCertificationRow {
        row_name,
        perturbation_class,
        hostile_expectation,
        parity_anchor: ParityAnchor::Control,
        control_lane,
        hostile_lane,
        parity_lane,
    }
}

pub(super) fn rejection_row(
    row_name: &'static str,
    perturbation_class: LivePerturbationClass,
    control_lane: LiveCertificationBundle,
    hostile_lane: LiveRejectionBundle,
    parity_lane: LiveCertificationBundle,
) -> RejectionCertificationRow<LivePerturbationClass, LiveCertificationBundle, LiveRejectionBundle>
{
    LiveRejectionRow {
        row_name,
        perturbation_class,
        control_lane,
        hostile_lane,
        parity_lane,
    }
}
