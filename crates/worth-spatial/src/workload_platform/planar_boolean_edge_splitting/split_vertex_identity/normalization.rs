use crate::workload_platform::planar_boolean_edge_splitting::micro_interval_normalization::{
    PlanarBooleanIntervalSubdivisionNormalizedSchedule,
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
};

use super::coalescence::SplitVertexCoalescence;
use super::counters::PlanarBooleanSplitVertexIdentityCounters;
use super::denial::{
    PlanarBooleanSplitVertexIdentityDenial, PlanarBooleanSplitVertexIdentityDenialKind,
};
use super::identity::{split_vertex_schedule_identity, split_vertex_schedule_set_identity};
use super::input_rows::{SplitVertexInputKind, SplitVertexInputRow};
use super::vertex_set::{
    PlanarBooleanSplitVertexIdentitySchedule, PlanarBooleanSplitVertexIdentitySet,
};

impl PlanarBooleanIntervalSubdivisionNormalizedScheduleSet {
    pub fn mint_split_vertex_identities(
        &self,
    ) -> Result<PlanarBooleanSplitVertexIdentitySet, PlanarBooleanSplitVertexIdentityDenial> {
        let mut schedules = Vec::with_capacity(self.schedules().len());
        let mut counters = CounterBuild::default();
        for schedule in self.schedules() {
            schedules.push(mint_schedule_split_vertex_identities(
                schedule,
                &mut counters,
            )?);
        }
        let set_identity =
            split_vertex_schedule_set_identity(self.schedule_set_identity(), &schedules);
        Ok(PlanarBooleanSplitVertexIdentitySet::new(
            set_identity,
            self.schedule_set_identity().to_string(),
            schedules,
            counters.finish(self.schedules().len()),
        ))
    }
}

fn mint_schedule_split_vertex_identities(
    schedule: &PlanarBooleanIntervalSubdivisionNormalizedSchedule,
    counters: &mut CounterBuild,
) -> Result<PlanarBooleanSplitVertexIdentitySchedule, PlanarBooleanSplitVertexIdentityDenial> {
    reject_foreign_schedule_rows(schedule)?;
    let inputs = split_vertex_inputs_for_schedule(schedule)?;
    counters.point_cuts_inspected += schedule.fragment_cuts().len();
    counters.interval_endpoint_candidates_inspected +=
        schedule.interval_subdivisions().len().saturating_mul(2);
    counters.endpoint_contact_decisions_inspected += schedule.endpoint_contact_decisions().len();
    let coalescence = SplitVertexCoalescence::from_inputs(inputs)?;
    counters.split_vertices_minted += coalescence.vertices().len();
    counters.split_vertices_coalesced += coalescence.coalesced_vertices();
    counters.interval_point_endpoint_collisions_resolved += coalescence.interval_point_collisions();
    let (vertices, decisions) = coalescence.into_parts();
    let schedule_identity =
        split_vertex_schedule_identity(schedule.schedule_identity(), &vertices, &decisions);
    Ok(PlanarBooleanSplitVertexIdentitySchedule::new(
        schedule_identity,
        schedule.schedule_identity().to_string(),
        schedule.source_edge_identity().to_string(),
        schedule.carrier_identity().to_string(),
        vertices,
        decisions,
    ))
}

fn split_vertex_inputs_for_schedule(
    schedule: &PlanarBooleanIntervalSubdivisionNormalizedSchedule,
) -> Result<Vec<SplitVertexInputRow>, PlanarBooleanSplitVertexIdentityDenial> {
    let mut inputs = Vec::with_capacity(
        schedule.fragment_cuts().len() + schedule.interval_subdivisions().len().saturating_mul(2),
    );
    for cut in schedule.fragment_cuts() {
        inputs.push(SplitVertexInputRow::from_point_cut(cut)?);
    }
    for subdivision in schedule.interval_subdivisions() {
        inputs.push(SplitVertexInputRow::from_interval_endpoint(
            subdivision,
            SplitVertexInputKind::IntervalStart,
        )?);
        inputs.push(SplitVertexInputRow::from_interval_endpoint(
            subdivision,
            SplitVertexInputKind::IntervalEnd,
        )?);
    }
    Ok(inputs)
}

fn reject_foreign_schedule_rows(
    schedule: &PlanarBooleanIntervalSubdivisionNormalizedSchedule,
) -> Result<(), PlanarBooleanSplitVertexIdentityDenial> {
    for cut in schedule.fragment_cuts() {
        if cut.source_edge_identity() != schedule.source_edge_identity()
            || cut.carrier_identity() != schedule.carrier_identity()
        {
            return Err(PlanarBooleanSplitVertexIdentityDenial::new(
                PlanarBooleanSplitVertexIdentityDenialKind::ForeignIntervalSubdivisionSchedule,
                cut.cut_identity(),
                "split vertex point cuts must belong to their interval-subdivision schedule",
            ));
        }
    }
    for subdivision in schedule.interval_subdivisions() {
        if subdivision.source_edge_identity() != schedule.source_edge_identity()
            || subdivision.carrier_identity() != schedule.carrier_identity()
        {
            return Err(PlanarBooleanSplitVertexIdentityDenial::new(
                PlanarBooleanSplitVertexIdentityDenialKind::ForeignIntervalSubdivisionSchedule,
                subdivision.subdivision_identity(),
                "split vertex interval endpoints must belong to their interval-subdivision schedule",
            ));
        }
    }
    Ok(())
}

#[derive(Default)]
struct CounterBuild {
    point_cuts_inspected: usize,
    interval_endpoint_candidates_inspected: usize,
    endpoint_contact_decisions_inspected: usize,
    split_vertices_minted: usize,
    split_vertices_coalesced: usize,
    interval_point_endpoint_collisions_resolved: usize,
}

impl CounterBuild {
    fn finish(self, schedules_inspected: usize) -> PlanarBooleanSplitVertexIdentityCounters {
        PlanarBooleanSplitVertexIdentityCounters::new(
            schedules_inspected,
            self.point_cuts_inspected,
            self.interval_endpoint_candidates_inspected,
            self.endpoint_contact_decisions_inspected,
            self.split_vertices_minted,
            self.split_vertices_coalesced,
            0,
            self.interval_point_endpoint_collisions_resolved,
        )
    }
}
