use crate::workload_platform::planar_boolean_edge_splitting::canonical_schedule_ordering::{
    PlanarBooleanOrderedEdgeSplitSchedule, PlanarBooleanOrderedEdgeSplitScheduleSet,
};

use super::contradiction_basis::reject_contradictory_same_parameter_points;
use super::counters::PlanarBooleanNormalizedEdgeSplitScheduleCounters;
use super::denial::PlanarBooleanDuplicateSplitNormalizationDenial;
use super::grouping::DuplicateSplitGrouping;
use super::identity::{normalized_schedule_identity, normalized_schedule_set_identity};
use super::normalized_cut::{
    PlanarBooleanNormalizedEdgeSplitSchedule, PlanarBooleanNormalizedEdgeSplitScheduleSet,
    PlanarBooleanNormalizedSplitCut,
};
use super::normalized_cut_builder::normalized_cut_from_duplicate_point_entries;

impl PlanarBooleanOrderedEdgeSplitScheduleSet {
    pub fn collapse_duplicate_split_points(
        &self,
    ) -> Result<
        PlanarBooleanNormalizedEdgeSplitScheduleSet,
        PlanarBooleanDuplicateSplitNormalizationDenial,
    > {
        let mut schedules = Vec::with_capacity(self.schedules().len());
        let mut counters = CounterBuild::default();
        for ordered_schedule in self.schedules() {
            schedules.push(normalized_schedule_from_ordered_schedule(
                ordered_schedule,
                &mut counters,
            )?);
        }
        let set_identity =
            normalized_schedule_set_identity(self.schedule_set_identity(), &schedules);
        Ok(PlanarBooleanNormalizedEdgeSplitScheduleSet::new(
            set_identity,
            self.schedule_set_identity().to_string(),
            schedules,
            counters.finish(self.schedules().len()),
        ))
    }
}

fn normalized_schedule_from_ordered_schedule(
    ordered_schedule: &PlanarBooleanOrderedEdgeSplitSchedule,
    counters: &mut CounterBuild,
) -> Result<PlanarBooleanNormalizedEdgeSplitSchedule, PlanarBooleanDuplicateSplitNormalizationDenial>
{
    reject_contradictory_same_parameter_points(ordered_schedule.ordered_entries())?;
    let grouping = DuplicateSplitGrouping::from_ordered_schedule(ordered_schedule)?;
    counters.record_grouping(&grouping);
    let cuts = normalized_cuts_from_grouping(&grouping)?;
    counters.record_normalized_cuts(&cuts);
    let retained_interval_entries = grouping.retained_interval_entries().to_vec();
    let retained_interval_entry_identities = retained_interval_entries
        .iter()
        .map(|entry| entry.entry_identity().to_string())
        .collect::<Vec<_>>();
    let schedule_identity = normalized_schedule_identity(
        ordered_schedule.schedule_identity(),
        &cuts,
        &retained_interval_entry_identities,
    );
    Ok(PlanarBooleanNormalizedEdgeSplitSchedule::new(
        schedule_identity,
        ordered_schedule.schedule_identity().to_string(),
        ordered_schedule.source_edge_identity().to_string(),
        ordered_schedule.carrier_identity().to_string(),
        cuts,
        retained_interval_entries,
    ))
}

fn normalized_cuts_from_grouping(
    grouping: &DuplicateSplitGrouping<'_>,
) -> Result<Vec<PlanarBooleanNormalizedSplitCut>, PlanarBooleanDuplicateSplitNormalizationDenial> {
    let mut cuts = Vec::with_capacity(grouping.point_groups().len());
    for point_group in grouping.point_groups() {
        cuts.push(normalized_cut_from_duplicate_point_entries(point_group)?);
    }
    Ok(cuts)
}

#[derive(Default)]
struct CounterBuild {
    raw_point_cuts: usize,
    normalized_point_cuts: usize,
    duplicate_reports_collapsed: usize,
    provenance_rows_retained: usize,
    retained_interval_entries: usize,
}

impl CounterBuild {
    fn record_grouping(&mut self, grouping: &DuplicateSplitGrouping<'_>) {
        self.raw_point_cuts += grouping.raw_point_cuts();
        self.retained_interval_entries += grouping.retained_interval_entry_count();
        self.duplicate_reports_collapsed += grouping
            .point_groups()
            .iter()
            .map(|entries| entries.len().saturating_sub(1))
            .sum::<usize>();
    }

    fn record_normalized_cuts(&mut self, cuts: &[PlanarBooleanNormalizedSplitCut]) {
        self.normalized_point_cuts += cuts.len();
        self.provenance_rows_retained += cuts
            .iter()
            .map(|cut| cut.provenance_entry_identities().len())
            .sum::<usize>();
    }

    fn finish(
        self,
        normalized_schedules: usize,
    ) -> PlanarBooleanNormalizedEdgeSplitScheduleCounters {
        PlanarBooleanNormalizedEdgeSplitScheduleCounters::new(
            normalized_schedules,
            self.raw_point_cuts,
            self.normalized_point_cuts,
            self.duplicate_reports_collapsed,
            self.provenance_rows_retained,
            self.retained_interval_entries,
        )
    }
}
