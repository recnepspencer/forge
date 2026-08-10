use crate::harness::certification::{ParityAnchor, RejectionCertificationRow};
use crate::harness::live_certification::{
    LiveCertificationBundle, LiveCertificationRow, LiveHostileExpectation, LivePerturbationClass,
    LiveRejectionBundle, LiveRejectionRow,
};

pub(in crate::harness::region_live_certification) fn canonical_row(
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

pub(in crate::harness::region_live_certification) fn rejection_row(
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
