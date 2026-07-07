use super::edge_splitting_support::recovered_carriers_for;
use super::metaboss_support::MetabossEventExtractionSubject;
use std::collections::BTreeSet;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanAdmittedIntervalSplitCandidateSet, PlanarBooleanPointSplitPostureSet,
    PlanarBooleanRawEdgeSplitScheduleEntryKind, PlanarBooleanRawEdgeSplitScheduleSet,
    PlanarBooleanSplitEventParticipationIndex,
};

pub(crate) fn assert_raw_edge_split_schedule_matches_metaboss(
    subject: &MetabossEventExtractionSubject,
) {
    let proof = build_raw_edge_split_schedule_for_metaboss(subject);
    assert_raw_counters_match_inputs(&proof.raw, &proof.postures, &proof.admitted_intervals);
    assert_raw_entries_retain_candidates(&proof.raw, &proof.postures, &proof.admitted_intervals);
    assert!(proof
        .raw
        .schedules()
        .iter()
        .all(|schedule| schedule
            .entries()
            .iter()
            .all(
                |entry| entry.source_edge_identity() == schedule.source_edge_identity()
                    && entry.carrier_identity() == schedule.carrier_identity()
            )));
}

pub(crate) fn build_raw_edge_split_schedule_for_metaboss(
    subject: &MetabossEventExtractionSubject,
) -> RawEdgeSplitScheduleMetabossProof {
    let recovered = recovered_carriers_for(subject);
    let index = PlanarBooleanSplitEventParticipationIndex::from_recovered_carriers(
        &recovered,
        subject.ledger(),
    )
    .expect("split participation index should consume recovered source-edge carriers");
    let point_candidates = index
        .extract_point_split_candidates()
        .expect("metaboss point candidates should extract from the participation index");
    let admitted_points = point_candidates
        .admit_parameter_domain()
        .expect("metaboss point candidates should admit in domain");
    let postures = admitted_points
        .classify_point_split_postures()
        .expect("metaboss point postures should classify");
    let interval_candidates = index
        .extract_interval_split_candidates()
        .expect("metaboss interval candidates should extract from the participation index");
    let admitted_intervals = interval_candidates
        .admit_parameter_domain()
        .expect("metaboss interval candidates should admit in domain");

    assert_eq!(
        postures.participation_index_identity(),
        admitted_intervals.participation_index_identity()
    );
    let raw = PlanarBooleanRawEdgeSplitScheduleSet::assemble_from_admitted_candidates(
        &postures,
        &admitted_intervals,
    )
    .expect("raw schedules should assemble from same-index candidate products");

    assert_eq!(
        raw.point_posture_set_identity(),
        postures.posture_set_identity()
    );
    assert_eq!(
        raw.interval_candidate_set_identity(),
        admitted_intervals.interval_candidate_set_identity()
    );
    RawEdgeSplitScheduleMetabossProof {
        raw,
        postures,
        admitted_intervals,
    }
}

pub(crate) struct RawEdgeSplitScheduleMetabossProof {
    pub(crate) raw: PlanarBooleanRawEdgeSplitScheduleSet,
    postures: PlanarBooleanPointSplitPostureSet,
    admitted_intervals: PlanarBooleanAdmittedIntervalSplitCandidateSet,
}

fn assert_raw_counters_match_inputs(
    raw: &PlanarBooleanRawEdgeSplitScheduleSet,
    postures: &PlanarBooleanPointSplitPostureSet,
    intervals: &PlanarBooleanAdmittedIntervalSplitCandidateSet,
) {
    let point_counters = postures.counters();
    let raw_counters = raw.counters();
    assert_eq!(
        raw_counters.point_entries(),
        postures.postured_candidates().len()
    );
    assert_eq!(
        raw_counters.interval_entries(),
        intervals.admitted_candidates().len()
    );
    assert_eq!(
        raw_counters.t_junction_entries(),
        point_counters.t_junction_promotions()
    );
    assert_eq!(
        raw_counters.shared_endpoint_noop_entries(),
        point_counters.shared_endpoint_noops()
    );
    assert_eq!(
        raw_counters.endpoint_noop_entries(),
        point_counters.endpoint_noops()
    );
    assert_eq!(
        raw_counters.source_event_groups(),
        distinct_source_event_identities(postures, intervals).len()
    );
    assert_eq!(
        raw_counters.source_edge_schedules(),
        distinct_source_edge_carrier_keys(raw).len()
    );
}

fn assert_raw_entries_retain_candidates(
    raw: &PlanarBooleanRawEdgeSplitScheduleSet,
    postures: &PlanarBooleanPointSplitPostureSet,
    intervals: &PlanarBooleanAdmittedIntervalSplitCandidateSet,
) {
    let mut expected = Vec::new();
    for postured in postures.postured_candidates() {
        expected.push(postured.postured_candidate_identity().to_string());
    }
    for admitted in intervals.admitted_candidates() {
        expected.push(admitted.candidate().candidate_identity().to_string());
    }
    expected.sort();
    let mut observed = raw
        .schedules()
        .iter()
        .flat_map(|schedule| schedule.entries())
        .map(|entry| {
            assert!(matches!(
                entry.kind(),
                PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(_)
                    | PlanarBooleanRawEdgeSplitScheduleEntryKind::Interval
            ));
            entry.candidate_identity().to_string()
        })
        .collect::<Vec<_>>();
    observed.sort();
    assert_eq!(observed, expected);
}

fn distinct_source_event_identities(
    postures: &PlanarBooleanPointSplitPostureSet,
    intervals: &PlanarBooleanAdmittedIntervalSplitCandidateSet,
) -> BTreeSet<String> {
    let mut identities = BTreeSet::new();
    for postured in postures.postured_candidates() {
        identities.insert(format!(
            "point:{}",
            postured
                .admitted_candidate()
                .candidate()
                .point_event_identity()
        ));
    }
    for interval in intervals.admitted_candidates() {
        identities.insert(format!(
            "interval:{}",
            interval.candidate().interval_event_identity()
        ));
    }
    identities
}

fn distinct_source_edge_carrier_keys(
    raw: &PlanarBooleanRawEdgeSplitScheduleSet,
) -> BTreeSet<(String, String)> {
    raw.schedules()
        .iter()
        .map(|schedule| {
            (
                schedule.source_edge_identity().to_string(),
                schedule.carrier_identity().to_string(),
            )
        })
        .collect()
}

const _: fn(&MetabossEventExtractionSubject) = assert_raw_edge_split_schedule_matches_metaboss;
