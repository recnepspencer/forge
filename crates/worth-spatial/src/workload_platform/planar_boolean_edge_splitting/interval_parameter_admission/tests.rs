use crate::workload_platform::planar_boolean_edge_splitting::interval_split_candidates::{
    PlanarBooleanIntervalSplitCandidate, PlanarBooleanIntervalSplitCandidateCounters,
    PlanarBooleanIntervalSplitCandidateInput, PlanarBooleanIntervalSplitCandidateSet,
};
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanIntervalEventKind, PlanarBooleanSourceIntervalSense,
};

use super::PlanarBooleanSplitIntervalAdmissionDenialKind;

#[test]
fn split_interval_parameter_domain_accepts_ordered_non_collapsed_ranges() {
    let admitted = candidate_set([0.2, 0.7], PlanarBooleanSourceIntervalSense::Forward)
        .admit_parameter_domain()
        .expect("ordered non-collapsed range should admit");

    assert_eq!(admitted.counters().inspected_interval_candidates(), 1);
    assert_eq!(admitted.counters().admitted_interval_candidates(), 1);
    assert_eq!(admitted.counters().collapsed_interval_denials(), 0);
    assert_eq!(admitted.counters().rejected_non_finite_intervals(), 0);
    assert_eq!(admitted.counters().rejected_out_of_domain_intervals(), 0);
    assert_eq!(
        admitted.counters().rejected_contradictory_sense_intervals(),
        0
    );
    assert_eq!(
        admitted.admitted_candidates()[0].admitted_parameter_range(),
        [0.2, 0.7]
    );
}

#[test]
fn split_interval_parameter_domain_rejects_collapsed_or_nan_ranges() {
    let collapsed = candidate_set([0.4, 0.4], PlanarBooleanSourceIntervalSense::Forward)
        .admit_parameter_domain()
        .expect_err("collapsed interval must deny");
    assert_eq!(
        collapsed.kind(),
        PlanarBooleanSplitIntervalAdmissionDenialKind::CollapsedInterval
    );
    assert_eq!(collapsed.rejected_collapsed_intervals(), 1);
    assert_eq!(collapsed.rejected_non_finite_intervals(), 0);

    let nan = candidate_set([0.2, f64::NAN], PlanarBooleanSourceIntervalSense::Forward)
        .admit_parameter_domain()
        .expect_err("NaN interval range must deny");
    assert_eq!(
        nan.kind(),
        PlanarBooleanSplitIntervalAdmissionDenialKind::NonFiniteRange
    );
    assert_eq!(nan.rejected_non_finite_intervals(), 1);
    assert_eq!(nan.rejected_collapsed_intervals(), 0);
}

#[test]
fn split_interval_parameter_domain_rejects_positive_and_negative_infinity() {
    let positive = candidate_set(
        [0.2, f64::INFINITY],
        PlanarBooleanSourceIntervalSense::Forward,
    )
    .admit_parameter_domain()
    .expect_err("positive infinity interval endpoint must deny");
    assert_eq!(
        positive.kind(),
        PlanarBooleanSplitIntervalAdmissionDenialKind::NonFiniteRange
    );
    assert_eq!(positive.rejected_non_finite_intervals(), 1);

    let negative = candidate_set(
        [f64::NEG_INFINITY, 0.2],
        PlanarBooleanSourceIntervalSense::Forward,
    )
    .admit_parameter_domain()
    .expect_err("negative infinity interval endpoint must deny");
    assert_eq!(
        negative.kind(),
        PlanarBooleanSplitIntervalAdmissionDenialKind::NonFiniteRange
    );
    assert_eq!(negative.rejected_non_finite_intervals(), 1);
}

#[test]
fn split_interval_parameter_domain_rejects_out_of_domain_without_clamping() {
    let below = candidate_set(
        [-f64::MIN_POSITIVE, 0.4],
        PlanarBooleanSourceIntervalSense::Forward,
    )
    .admit_parameter_domain()
    .expect_err("negative near-zero endpoint must deny instead of clamping");
    assert_eq!(
        below.kind(),
        PlanarBooleanSplitIntervalAdmissionDenialKind::OutOfDomainRange
    );
    assert_eq!(below.rejected_out_of_domain_intervals(), 1);

    let above = candidate_set(
        [0.4, 1.0 + f64::EPSILON],
        PlanarBooleanSourceIntervalSense::Forward,
    )
    .admit_parameter_domain()
    .expect_err("greater-than-one endpoint must deny instead of clamping");
    assert_eq!(
        above.kind(),
        PlanarBooleanSplitIntervalAdmissionDenialKind::OutOfDomainRange
    );
    assert_eq!(above.rejected_out_of_domain_intervals(), 1);
}

#[test]
fn split_interval_parameter_domain_rejects_contradictory_forward_and_reversed_sense() {
    let reversed_on_ascending =
        candidate_set([0.2, 0.7], PlanarBooleanSourceIntervalSense::Reversed)
            .admit_parameter_domain()
            .expect_err("reversed sense must match descending source range");
    assert_eq!(
        reversed_on_ascending.kind(),
        PlanarBooleanSplitIntervalAdmissionDenialKind::ContradictoryIntervalSense
    );
    assert_eq!(
        reversed_on_ascending.rejected_contradictory_sense_intervals(),
        1
    );

    let forward_on_descending =
        candidate_set([0.7, 0.2], PlanarBooleanSourceIntervalSense::Forward)
            .admit_parameter_domain()
            .expect_err("forward sense must match ascending source range");
    assert_eq!(
        forward_on_descending.kind(),
        PlanarBooleanSplitIntervalAdmissionDenialKind::ContradictoryIntervalSense
    );
    assert_eq!(
        forward_on_descending.rejected_contradictory_sense_intervals(),
        1
    );
}

#[test]
fn split_interval_parameter_domain_preserves_source_sense_after_ordering() {
    let admitted = candidate_set([0.7, 0.2], PlanarBooleanSourceIntervalSense::Reversed)
        .admit_parameter_domain()
        .expect("reversed source interval should admit after ordering");
    let candidate = admitted.admitted_candidates()[0].candidate();

    assert_eq!(
        admitted.admitted_candidates()[0].admitted_parameter_range(),
        [0.2, 0.7]
    );
    assert_eq!(candidate.source_parameter_range(), [0.7, 0.2]);
    assert_eq!(
        candidate.source_sense(),
        PlanarBooleanSourceIntervalSense::Reversed
    );
}

#[test]
fn split_interval_parameter_domain_denies_mixed_candidate_sets_instead_of_filtering_bad_rows() {
    let candidates = multi_candidate_set(&[
        ([0.2, 0.6], PlanarBooleanSourceIntervalSense::Forward),
        (
            [0.3, 1.0 + f64::EPSILON],
            PlanarBooleanSourceIntervalSense::Forward,
        ),
        ([0.8, 0.4], PlanarBooleanSourceIntervalSense::Reversed),
    ]);
    let poisoned_candidate_identity = candidates.candidates()[1].candidate_identity().to_string();

    let denial = candidates
        .admit_parameter_domain()
        .expect_err("one poisoned interval must deny the whole admission product");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitIntervalAdmissionDenialKind::OutOfDomainRange
    );
    assert_eq!(denial.evidence_identity(), poisoned_candidate_identity);
    assert_eq!(denial.rejected_out_of_domain_intervals(), 1);
}

#[test]
fn split_interval_parameter_domain_preserves_candidate_set_identity_and_order() {
    let candidates = multi_candidate_set(&[
        ([0.8, 0.4], PlanarBooleanSourceIntervalSense::Reversed),
        ([0.2, 0.6], PlanarBooleanSourceIntervalSense::Forward),
    ]);
    let original_order = candidates
        .candidates()
        .iter()
        .map(|candidate| candidate.candidate_identity().to_string())
        .collect::<Vec<_>>();

    let admitted = candidates
        .admit_parameter_domain()
        .expect("valid intervals should admit in candidate order");
    let admitted_order = admitted
        .admitted_candidates()
        .iter()
        .map(|candidate| candidate.candidate().candidate_identity().to_string())
        .collect::<Vec<_>>();

    assert_eq!(
        admitted.interval_candidate_set_identity(),
        candidates.candidate_set_identity()
    );
    assert_eq!(admitted_order, original_order);
}

#[test]
fn split_interval_parameter_domain_treats_signed_zero_collapse_as_collapsed_interval() {
    let denial = candidate_set([0.0, -0.0], PlanarBooleanSourceIntervalSense::Forward)
        .admit_parameter_domain()
        .expect_err("signed-zero interval endpoints still collapse to a point");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitIntervalAdmissionDenialKind::CollapsedInterval
    );
    assert_eq!(denial.rejected_collapsed_intervals(), 1);
}

fn candidate_set(
    source_parameter_range: [f64; 2],
    source_sense: PlanarBooleanSourceIntervalSense,
) -> PlanarBooleanIntervalSplitCandidateSet {
    multi_candidate_set(&[(source_parameter_range, source_sense)])
}

fn multi_candidate_set(
    intervals: &[([f64; 2], PlanarBooleanSourceIntervalSense)],
) -> PlanarBooleanIntervalSplitCandidateSet {
    PlanarBooleanIntervalSplitCandidateSet::new(
        "test interval candidate set".to_string(),
        "test participation index".to_string(),
        intervals
            .iter()
            .enumerate()
            .map(|(offset, (source_parameter_range, source_sense))| {
                candidate(offset, *source_parameter_range, *source_sense)
            })
            .collect(),
        PlanarBooleanIntervalSplitCandidateCounters::default(),
    )
}

fn candidate(
    offset: usize,
    source_parameter_range: [f64; 2],
    source_sense: PlanarBooleanSourceIntervalSense,
) -> PlanarBooleanIntervalSplitCandidate {
    PlanarBooleanIntervalSplitCandidate::new(PlanarBooleanIntervalSplitCandidateInput {
        candidate_identity: format!("candidate:{offset}:{source_parameter_range:?}"),
        interval_event_identity: "interval event".to_string(),
        interval_event_kind: PlanarBooleanIntervalEventKind::PartialOverlap,
        carrier_identity: "carrier".to_string(),
        source_edge_identity: "source edge".to_string(),
        segment_identity: "segment".to_string(),
        source_interval_identity: "source interval".to_string(),
        source_parameter_range,
        source_sense,
        normalized_interval_identity: "normalized interval".to_string(),
        normalized_parameter_range: [0.2, 0.7],
        local_frame_identity: "local frame".to_string(),
        precision_basis_identity: "precision basis".to_string(),
        participation_row_identity: "participation row".to_string(),
        event_group_identities: vec!["event group".to_string()],
    })
}
