use std::collections::{BTreeMap, BTreeSet};

use crate::workload_platform::planar_boolean_edge_splitting::{
    interval_parameter_admission::AdmittedIntervalSplitCandidate,
    interval_parameter_admission::PlanarBooleanAdmittedIntervalSplitCandidateSet,
    point_split_posture::PlanarBooleanPointSplitPosture,
    point_split_posture::PlanarBooleanPointSplitPostureSet,
    point_split_posture::PosturedPointSplitCandidate,
};

use super::counters::PlanarBooleanRawEdgeSplitScheduleCounters;
use super::denial::{
    PlanarBooleanRawEdgeSplitScheduleDenial, PlanarBooleanRawEdgeSplitScheduleDenialKind,
};
use super::identity::{raw_entry_identity, raw_schedule_identity, raw_schedule_set_identity};
use super::schedule::{
    PlanarBooleanRawEdgeSplitSchedule, PlanarBooleanRawEdgeSplitScheduleEntry,
    PlanarBooleanRawEdgeSplitScheduleEntryKind, PlanarBooleanRawEdgeSplitScheduleSet,
    PlanarBooleanRawIntervalAuthority, PlanarBooleanRawPointEndpointAuthority,
};

impl PlanarBooleanRawEdgeSplitScheduleSet {
    pub fn assemble_from_admitted_candidates(
        point_postures: &PlanarBooleanPointSplitPostureSet,
        intervals: &PlanarBooleanAdmittedIntervalSplitCandidateSet,
    ) -> Result<Self, PlanarBooleanRawEdgeSplitScheduleDenial> {
        validate_candidate_set_lineage(point_postures, intervals)?;
        let mut grouped: BTreeMap<
            SourceEdgeScheduleKey,
            Vec<PlanarBooleanRawEdgeSplitScheduleEntry>,
        > = BTreeMap::new();
        let mut counter_build = CounterBuild::default();
        let mut source_event_groups = BTreeSet::new();
        insert_point_candidates_into_split_schedule(
            point_postures,
            &mut grouped,
            &mut counter_build,
            &mut source_event_groups,
        );
        insert_interval_candidates_into_split_schedule(
            intervals,
            &mut grouped,
            &mut counter_build,
            &mut source_event_groups,
        );
        let schedules = build_source_edge_schedules(grouped)?;
        let counters = counter_build.finish(schedules.len(), source_event_groups.len());
        let set_identity = raw_schedule_set_identity(
            point_postures.posture_set_identity(),
            intervals.interval_candidate_set_identity(),
            &schedules,
        );
        Ok(Self::new(
            set_identity,
            point_postures.posture_set_identity().to_string(),
            intervals.interval_candidate_set_identity().to_string(),
            schedules,
            counters,
        ))
    }
}

fn insert_point_candidates_into_split_schedule(
    point_postures: &PlanarBooleanPointSplitPostureSet,
    grouped: &mut BTreeMap<SourceEdgeScheduleKey, Vec<PlanarBooleanRawEdgeSplitScheduleEntry>>,
    counter_build: &mut CounterBuild,
    source_event_groups: &mut BTreeSet<String>,
) {
    for postured in point_postures.postured_candidates() {
        let entry = raw_point_schedule_entry(postured);
        source_event_groups.insert(format!("point:{}", entry.event_identity()));
        counter_build.record_point(postured.posture());
        insert_raw_entry_into_source_schedule(grouped, entry);
    }
}

fn insert_interval_candidates_into_split_schedule(
    intervals: &PlanarBooleanAdmittedIntervalSplitCandidateSet,
    grouped: &mut BTreeMap<SourceEdgeScheduleKey, Vec<PlanarBooleanRawEdgeSplitScheduleEntry>>,
    counter_build: &mut CounterBuild,
    source_event_groups: &mut BTreeSet<String>,
) {
    for admitted in intervals.admitted_candidates() {
        let entry = raw_interval_schedule_entry(admitted);
        source_event_groups.insert(format!("interval:{}", entry.event_identity()));
        counter_build.interval_entries += 1;
        insert_raw_entry_into_source_schedule(grouped, entry);
    }
}

fn insert_raw_entry_into_source_schedule(
    grouped: &mut BTreeMap<SourceEdgeScheduleKey, Vec<PlanarBooleanRawEdgeSplitScheduleEntry>>,
    entry: PlanarBooleanRawEdgeSplitScheduleEntry,
) {
    grouped
        .entry(SourceEdgeScheduleKey::from_entry(&entry))
        .or_default()
        .push(entry);
}

fn build_source_edge_schedules(
    grouped: BTreeMap<SourceEdgeScheduleKey, Vec<PlanarBooleanRawEdgeSplitScheduleEntry>>,
) -> Result<Vec<PlanarBooleanRawEdgeSplitSchedule>, PlanarBooleanRawEdgeSplitScheduleDenial> {
    let mut schedules = Vec::with_capacity(grouped.len());
    for (schedule_key, mut entries) in grouped {
        entries.sort_by(|left, right| left.entry_identity().cmp(right.entry_identity()));
        reject_mixed_source_edges(&schedule_key, &entries)?;
        let schedule_identity = raw_schedule_identity(
            &schedule_key.source_edge_identity,
            &schedule_key.carrier_identity,
            &entries,
        );
        schedules.push(PlanarBooleanRawEdgeSplitSchedule::new(
            schedule_identity,
            schedule_key.source_edge_identity,
            schedule_key.carrier_identity,
            entries,
        ));
    }
    Ok(schedules)
}

fn raw_point_schedule_entry(
    postured: &PosturedPointSplitCandidate,
) -> PlanarBooleanRawEdgeSplitScheduleEntry {
    let candidate = postured.admitted_candidate().candidate();
    PlanarBooleanRawEdgeSplitScheduleEntry::new(
        raw_entry_identity(
            candidate.source_edge_identity(),
            postured.postured_candidate_identity(),
        ),
        candidate.source_edge_identity().to_string(),
        candidate.carrier_identity().to_string(),
        postured.postured_candidate_identity().to_string(),
        candidate.point_event_identity().to_string(),
        Some(candidate.parameter_fact_identity().to_string()),
        candidate.parameter(),
        None,
        candidate
            .coordinate_fact()
            .local_frame_identity()
            .to_string(),
        candidate
            .coordinate_fact()
            .precision_basis_identity()
            .to_string(),
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(postured.posture()),
        candidate.segment_pair_identities().to_vec(),
        candidate.predicate_receipt_identities().to_vec(),
        candidate.event_group_identities().to_vec(),
        PlanarBooleanRawPointEndpointAuthority {
            exact_endpoint_source_identity: postured
                .admitted_candidate()
                .exact_endpoint_source_identity()
                .map(str::to_string),
            exact_projected_endpoint_fact_identity: postured
                .admitted_candidate()
                .exact_projected_endpoint_fact_identity()
                .map(str::to_string),
            shared_endpoint_source_identities: candidate
                .shared_endpoint_source_identities()
                .to_vec(),
            shared_endpoint_projection_fact_digests: candidate
                .shared_endpoint_projection_fact_digests()
                .to_vec(),
        },
        None,
    )
}

fn raw_interval_schedule_entry(
    admitted: &AdmittedIntervalSplitCandidate,
) -> PlanarBooleanRawEdgeSplitScheduleEntry {
    let candidate = admitted.candidate();
    PlanarBooleanRawEdgeSplitScheduleEntry::new(
        raw_entry_identity(
            candidate.source_edge_identity(),
            candidate.candidate_identity(),
        ),
        candidate.source_edge_identity().to_string(),
        candidate.carrier_identity().to_string(),
        candidate.candidate_identity().to_string(),
        candidate.interval_event_identity().to_string(),
        None,
        admitted.admitted_parameter_range()[0],
        Some(admitted.admitted_parameter_range()),
        candidate.local_frame_identity().to_string(),
        candidate.precision_basis_identity().to_string(),
        PlanarBooleanRawEdgeSplitScheduleEntryKind::Interval,
        Vec::new(),
        Vec::new(),
        candidate.event_group_identities().to_vec(),
        PlanarBooleanRawPointEndpointAuthority::default(),
        Some(PlanarBooleanRawIntervalAuthority::new(
            candidate.interval_event_kind(),
            candidate.source_interval_identity().to_string(),
            candidate.source_parameter_range(),
            candidate.source_sense(),
            candidate.normalized_interval_identity().to_string(),
            candidate.normalized_parameter_range(),
            candidate.participation_row_identity().to_string(),
        )),
    )
}

fn validate_candidate_set_lineage(
    point_postures: &PlanarBooleanPointSplitPostureSet,
    intervals: &PlanarBooleanAdmittedIntervalSplitCandidateSet,
) -> Result<(), PlanarBooleanRawEdgeSplitScheduleDenial> {
    if point_postures.participation_index_identity() == intervals.participation_index_identity() {
        return Ok(());
    }
    Err(PlanarBooleanRawEdgeSplitScheduleDenial::new(
        PlanarBooleanRawEdgeSplitScheduleDenialKind::ForeignCandidateSet,
        format!(
            "{}|{}",
            point_postures.participation_index_identity(),
            intervals.participation_index_identity()
        ),
        "raw split schedules must assemble point and interval candidates from the same participation index",
    ))
}

pub(super) fn reject_mixed_source_edges(
    expected_key: &SourceEdgeScheduleKey,
    entries: &[PlanarBooleanRawEdgeSplitScheduleEntry],
) -> Result<(), PlanarBooleanRawEdgeSplitScheduleDenial> {
    if entries.iter().all(|entry| {
        entry.source_edge_identity() == expected_key.source_edge_identity
            && entry.carrier_identity() == expected_key.carrier_identity
    }) {
        return Ok(());
    }
    Err(PlanarBooleanRawEdgeSplitScheduleDenial::new(
        PlanarBooleanRawEdgeSplitScheduleDenialKind::MixedSourceEdgeSchedule,
        format!(
            "{}|{}",
            expected_key.source_edge_identity, expected_key.carrier_identity
        ),
        "raw split schedule entries must all belong to the keyed source edge carrier",
    ))
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct SourceEdgeScheduleKey {
    source_edge_identity: String,
    carrier_identity: String,
}

impl SourceEdgeScheduleKey {
    pub(super) fn from_entry(entry: &PlanarBooleanRawEdgeSplitScheduleEntry) -> Self {
        Self {
            source_edge_identity: entry.source_edge_identity().to_string(),
            carrier_identity: entry.carrier_identity().to_string(),
        }
    }
}

#[derive(Default)]
struct CounterBuild {
    point_entries: usize,
    interval_entries: usize,
    t_junction_entries: usize,
    shared_endpoint_noop_entries: usize,
    endpoint_noop_entries: usize,
}

impl CounterBuild {
    fn record_point(&mut self, posture: PlanarBooleanPointSplitPosture) {
        self.point_entries += 1;
        if posture == PlanarBooleanPointSplitPosture::TJunctionPromotion {
            self.t_junction_entries += 1;
        }
        if posture == PlanarBooleanPointSplitPosture::SharedEndpoint {
            self.shared_endpoint_noop_entries += 1;
        }
        if posture == PlanarBooleanPointSplitPosture::EndpointNoOp {
            self.endpoint_noop_entries += 1;
        }
    }

    fn finish(
        self,
        source_edge_schedules: usize,
        source_event_groups: usize,
    ) -> PlanarBooleanRawEdgeSplitScheduleCounters {
        PlanarBooleanRawEdgeSplitScheduleCounters::new(
            source_edge_schedules,
            self.point_entries,
            self.interval_entries,
            self.t_junction_entries,
            self.shared_endpoint_noop_entries,
            self.endpoint_noop_entries,
            source_event_groups,
        )
    }
}
