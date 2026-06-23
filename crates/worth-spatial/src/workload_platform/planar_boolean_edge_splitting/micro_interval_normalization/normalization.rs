use crate::workload_platform::planar_boolean_edge_splitting::endpoint_boundary_normalization::{
    PlanarBooleanEndpointBoundaryNormalizedSplitSchedule,
    PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
};

use super::action::PlanarBooleanMicroIntervalPolicy;
use super::counters::PlanarBooleanIntervalSubdivisionNormalizationCounters;
use super::denial::{
    PlanarBooleanIntervalSubdivisionNormalizationDenial,
    PlanarBooleanIntervalSubdivisionNormalizationDenialKind,
};
use super::identity::{
    interval_subdivision_schedule_identity, interval_subdivision_schedule_set_identity,
};
use super::span_grouping::IntervalSubdivisionGrouping;
use super::subdivision_row::{
    PlanarBooleanIntervalSubdivisionNormalizedSchedule,
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
};

impl PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet {
    pub fn normalize_overlap_interval_subdivisions(
        &self,
        policy: PlanarBooleanMicroIntervalPolicy,
    ) -> Result<
        PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
        PlanarBooleanIntervalSubdivisionNormalizationDenial,
    > {
        let mut schedules = Vec::with_capacity(self.schedules().len());
        let mut counters = CounterBuild::default();
        for schedule in self.schedules() {
            schedules.push(normalize_schedule(schedule, policy, &mut counters)?);
        }
        let schedule_set_identity =
            interval_subdivision_schedule_set_identity(self.schedule_set_identity(), &schedules);
        Ok(PlanarBooleanIntervalSubdivisionNormalizedScheduleSet::new(
            schedule_set_identity,
            self.schedule_set_identity().to_string(),
            schedules,
            counters.finish(self.schedules().len()),
        ))
    }
}

fn normalize_schedule(
    schedule: &PlanarBooleanEndpointBoundaryNormalizedSplitSchedule,
    policy: PlanarBooleanMicroIntervalPolicy,
    counters: &mut CounterBuild,
) -> Result<
    PlanarBooleanIntervalSubdivisionNormalizedSchedule,
    PlanarBooleanIntervalSubdivisionNormalizationDenial,
> {
    reject_foreign_retained_interval_rows(schedule)?;
    let grouping = IntervalSubdivisionGrouping::from_retained_entries(
        schedule.schedule_identity(),
        schedule.retained_interval_entries(),
        policy,
    )?;
    counters.record_schedule(schedule, &grouping);
    let interval_subdivisions = grouping.into_rows();
    let schedule_identity = interval_subdivision_schedule_identity(
        schedule.schedule_identity(),
        &interval_subdivisions,
    );
    Ok(PlanarBooleanIntervalSubdivisionNormalizedSchedule::new(
        schedule_identity,
        schedule.schedule_identity().to_string(),
        schedule.source_edge_identity().to_string(),
        schedule.carrier_identity().to_string(),
        schedule.fragment_cuts().to_vec(),
        schedule.endpoint_contact_decisions().to_vec(),
        interval_subdivisions,
    ))
}

fn reject_foreign_retained_interval_rows(
    schedule: &PlanarBooleanEndpointBoundaryNormalizedSplitSchedule,
) -> Result<(), PlanarBooleanIntervalSubdivisionNormalizationDenial> {
    for entry in schedule.retained_interval_entries() {
        if entry.source_edge_identity() != schedule.source_edge_identity()
            || entry.carrier_identity() != schedule.carrier_identity()
        {
            return Err(PlanarBooleanIntervalSubdivisionNormalizationDenial::new(
                PlanarBooleanIntervalSubdivisionNormalizationDenialKind::ForeignEndpointBoundarySchedule,
                entry.entry_identity(),
                "retained interval row must belong to the endpoint-boundary schedule source edge and carrier",
            ));
        }
    }
    Ok(())
}

#[derive(Default)]
struct CounterBuild {
    retained_interval_rows_inspected: usize,
    normalized_interval_subdivisions: usize,
    redundant_interval_rows_collapsed: usize,
    micro_intervals_admitted: usize,
    micro_intervals_policy_required: usize,
    opposite_sense_rows_preserved: usize,
    fragment_point_cuts_retained: usize,
    endpoint_contact_decisions_retained: usize,
}

impl CounterBuild {
    fn record_schedule(
        &mut self,
        schedule: &PlanarBooleanEndpointBoundaryNormalizedSplitSchedule,
        grouping: &IntervalSubdivisionGrouping,
    ) {
        self.retained_interval_rows_inspected += grouping.inspected_rows();
        self.normalized_interval_subdivisions += grouping.rows().len();
        self.redundant_interval_rows_collapsed += grouping.redundant_rows_collapsed();
        self.micro_intervals_admitted += grouping.micro_intervals_admitted();
        self.micro_intervals_policy_required += grouping.micro_intervals_policy_required();
        self.opposite_sense_rows_preserved += grouping.opposite_sense_rows_preserved();
        self.fragment_point_cuts_retained += schedule.fragment_cuts().len();
        self.endpoint_contact_decisions_retained += schedule.endpoint_contact_decisions().len();
    }

    fn finish(
        self,
        normalized_schedules: usize,
    ) -> PlanarBooleanIntervalSubdivisionNormalizationCounters {
        PlanarBooleanIntervalSubdivisionNormalizationCounters::new(
            normalized_schedules,
            self.retained_interval_rows_inspected,
            self.normalized_interval_subdivisions,
            self.redundant_interval_rows_collapsed,
            self.micro_intervals_admitted,
            self.micro_intervals_policy_required,
            self.opposite_sense_rows_preserved,
            self.fragment_point_cuts_retained,
            self.endpoint_contact_decisions_retained,
        )
    }
}
