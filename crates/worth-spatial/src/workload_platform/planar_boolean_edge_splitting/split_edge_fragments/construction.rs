use crate::workload_platform::planar_boolean_edge_splitting::canonical_parameter::canonical_parameter_bits;
use crate::workload_platform::planar_boolean_edge_splitting::micro_interval_normalization::{
    PlanarBooleanIntervalSubdivisionNormalizedSchedule,
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
};
use crate::workload_platform::planar_boolean_edge_splitting::split_vertex_identity::{
    PlanarBooleanSplitVertexIdentitySchedule, PlanarBooleanSplitVertexIdentitySet,
};
use crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense;

use super::boundary_partition::{boundary_partition_for_schedule, FragmentBoundary};
use super::counters::PlanarBooleanSplitEdgeFragmentCounters;
use super::denial::{
    PlanarBooleanSplitEdgeFragmentDenial, PlanarBooleanSplitEdgeFragmentDenialKind,
};
use super::fragment_row::PlanarBooleanSplitEdgeFragment;
use super::fragment_set::{
    PlanarBooleanSplitEdgeFragmentSchedule, PlanarBooleanSplitEdgeFragmentSet,
};
use super::identity::{
    split_edge_fragment_identity, split_edge_fragment_schedule_identity,
    split_edge_fragment_set_identity,
};
use super::interval_membership::FragmentIntervalMembership;
use super::validation::{
    reject_foreign_schedule_rows, reject_fragment_coverage_gaps, reject_mismatched_vertex_set,
    reject_mixed_schedule_set_basis, require_vertex_schedule, vertex_schedule_index,
};

impl PlanarBooleanIntervalSubdivisionNormalizedScheduleSet {
    pub fn build_split_edge_fragments(
        &self,
        split_vertices: &PlanarBooleanSplitVertexIdentitySet,
    ) -> Result<PlanarBooleanSplitEdgeFragmentSet, PlanarBooleanSplitEdgeFragmentDenial> {
        reject_mismatched_vertex_set(self.schedule_set_identity(), split_vertices)?;
        reject_mixed_schedule_set_basis(self, split_vertices)?;
        let vertex_index = vertex_schedule_index(split_vertices);
        let fallback_basis = fallback_basis_for_schedule_set(self, split_vertices);
        let mut schedules = Vec::with_capacity(self.schedules().len());
        let mut counters = CounterBuild::default();
        for schedule in self.schedules() {
            let vertex_schedule = require_vertex_schedule(schedule, &vertex_index)?;
            schedules.push(build_schedule_fragments(
                schedule,
                vertex_schedule,
                fallback_basis.as_ref(),
                &mut counters,
            )?);
        }
        let fragment_set_identity = split_edge_fragment_set_identity(
            self.schedule_set_identity(),
            split_vertices.split_vertex_identity_set_identity(),
            &schedules,
        );
        Ok(PlanarBooleanSplitEdgeFragmentSet::new(
            fragment_set_identity,
            self.schedule_set_identity().to_string(),
            split_vertices
                .split_vertex_identity_set_identity()
                .to_string(),
            schedules,
            counters.finish(self.schedules().len()),
        ))
    }
}

fn build_schedule_fragments(
    schedule: &PlanarBooleanIntervalSubdivisionNormalizedSchedule,
    vertex_schedule: &PlanarBooleanSplitVertexIdentitySchedule,
    fallback_basis: Option<&(String, String)>,
    counters: &mut CounterBuild,
) -> Result<PlanarBooleanSplitEdgeFragmentSchedule, PlanarBooleanSplitEdgeFragmentDenial> {
    reject_foreign_schedule_rows(schedule)?;
    let boundaries = boundary_partition_for_schedule(
        schedule,
        vertex_schedule,
        fallback_basis.map(|basis| (basis.0.as_str(), basis.1.as_str())),
    )?;
    counters.split_vertices_consumed += vertex_schedule.vertices().len();
    counters.original_endpoint_boundaries_synthesized += 2;
    counters.endpoint_noop_boundaries_skipped += schedule.endpoint_contact_decisions().len();
    let fragments = fragments_from_boundaries(schedule, &boundaries, counters)?;
    reject_fragment_coverage_gaps(schedule.schedule_identity(), &fragments)?;
    counters.source_edges_covered += 1;
    let schedule_identity = split_edge_fragment_schedule_identity(
        schedule.schedule_identity(),
        vertex_schedule.schedule_identity(),
        &fragments,
    );
    Ok(PlanarBooleanSplitEdgeFragmentSchedule::new(
        schedule_identity,
        schedule.schedule_identity().to_string(),
        vertex_schedule.schedule_identity().to_string(),
        schedule.source_edge_identity().to_string(),
        schedule.carrier_identity().to_string(),
        fragments,
    ))
}

fn fallback_basis_for_schedule_set(
    schedule_set: &PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    split_vertices: &PlanarBooleanSplitVertexIdentitySet,
) -> Option<(String, String)> {
    split_vertices
        .vertices()
        .next()
        .map(|vertex| {
            (
                vertex.local_frame_identity().to_string(),
                vertex.precision_basis_identity().to_string(),
            )
        })
        .or_else(|| {
            schedule_set.schedules().iter().find_map(|schedule| {
                schedule
                    .interval_subdivisions()
                    .first()
                    .map(|subdivision| {
                        (
                            subdivision.local_frame_identity().to_string(),
                            subdivision.precision_basis_identity().to_string(),
                        )
                    })
                    .or_else(|| {
                        schedule.fragment_cuts().first().map(|cut| {
                            (
                                cut.local_frame_identity().to_string(),
                                cut.precision_basis_identity().to_string(),
                            )
                        })
                    })
            })
        })
}

fn fragments_from_boundaries(
    schedule: &PlanarBooleanIntervalSubdivisionNormalizedSchedule,
    boundaries: &[FragmentBoundary],
    counters: &mut CounterBuild,
) -> Result<Vec<PlanarBooleanSplitEdgeFragment>, PlanarBooleanSplitEdgeFragmentDenial> {
    let mut fragments = Vec::with_capacity(boundaries.len().saturating_sub(1));
    for pair in boundaries.windows(2) {
        let [start, end] = pair else {
            unreachable!("windows(2) always yields two boundaries")
        };
        if end.parameter <= start.parameter {
            counters.collapsed_fragments_rejected += 1;
            return Err(PlanarBooleanSplitEdgeFragmentDenial::new(
                PlanarBooleanSplitEdgeFragmentDenialKind::CollapsedSplitFragment,
                schedule.schedule_identity(),
                "split fragment construction cannot emit a zero-length or inverted fragment",
            ));
        }
        let membership = FragmentIntervalMembership::for_range(
            [start.parameter, end.parameter],
            schedule.interval_subdivisions(),
        );
        if membership.is_interval_attributed() {
            counters.interval_attributed_fragments += 1;
        }
        fragments.push(fragment_from_boundary_pair(
            schedule, start, end, membership,
        ));
    }
    counters.fragments_emitted += fragments.len();
    Ok(fragments)
}

fn fragment_from_boundary_pair(
    schedule: &PlanarBooleanIntervalSubdivisionNormalizedSchedule,
    start: &FragmentBoundary,
    end: &FragmentBoundary,
    membership: FragmentIntervalMembership,
) -> PlanarBooleanSplitEdgeFragment {
    let mut point_cut_identities = start.point_cut_identities.clone();
    point_cut_identities.extend(end.point_cut_identities.iter().cloned());
    canonicalize_strings(&mut point_cut_identities);
    let mut event_group_identities = start.event_group_identities.clone();
    event_group_identities.extend(end.event_group_identities.iter().cloned());
    event_group_identities.extend(membership.event_group_identities.iter().cloned());
    canonicalize_strings(&mut event_group_identities);
    let mut cause_provenance_identities = start.cause_provenance_identities.clone();
    cause_provenance_identities.extend(end.cause_provenance_identities.iter().cloned());
    cause_provenance_identities.extend(membership.provenance_identities.iter().cloned());
    canonicalize_strings(&mut cause_provenance_identities);
    let mut source_senses = membership.source_senses;
    if source_senses.is_empty() {
        source_senses.push(PlanarBooleanSourceIntervalSense::Forward);
    }
    let parameter_range_bits = [
        canonical_parameter_bits(start.parameter),
        canonical_parameter_bits(end.parameter),
    ];
    let mut identity_causes = point_cut_identities.clone();
    identity_causes.extend(membership.interval_subdivision_identities.iter().cloned());
    identity_causes.extend(membership.normalized_interval_identities.iter().cloned());
    identity_causes.extend(event_group_identities.iter().cloned());
    identity_causes.extend(cause_provenance_identities.iter().cloned());
    canonicalize_strings(&mut identity_causes);
    let fragment_identity = split_edge_fragment_identity(
        schedule.source_edge_identity(),
        schedule.carrier_identity(),
        &start.endpoint,
        &end.endpoint,
        parameter_range_bits,
        start.endpoint.local_frame_identity(),
        start.endpoint.precision_basis_identity(),
        &identity_causes,
    );
    PlanarBooleanSplitEdgeFragment::new(
        fragment_identity,
        schedule.source_edge_identity().to_string(),
        schedule.carrier_identity().to_string(),
        start.endpoint.clone(),
        end.endpoint.clone(),
        [start.parameter, end.parameter],
        parameter_range_bits,
        start.endpoint.local_frame_identity().to_string(),
        start.endpoint.precision_basis_identity().to_string(),
        source_senses,
        point_cut_identities,
        membership.interval_subdivision_identities,
        membership.normalized_interval_identities,
        event_group_identities,
        cause_provenance_identities,
    )
}

fn canonicalize_strings(values: &mut Vec<String>) {
    values.sort();
    values.dedup();
}

#[derive(Default)]
struct CounterBuild {
    source_edges_covered: usize,
    split_vertices_consumed: usize,
    original_endpoint_boundaries_synthesized: usize,
    fragments_emitted: usize,
    interval_attributed_fragments: usize,
    endpoint_noop_boundaries_skipped: usize,
    collapsed_fragments_rejected: usize,
    coverage_gaps_rejected: usize,
    foreign_schedule_rows_rejected: usize,
}

impl CounterBuild {
    fn finish(self, schedules_inspected: usize) -> PlanarBooleanSplitEdgeFragmentCounters {
        PlanarBooleanSplitEdgeFragmentCounters::new(
            schedules_inspected,
            self.source_edges_covered,
            self.split_vertices_consumed,
            self.original_endpoint_boundaries_synthesized,
            self.fragments_emitted,
            self.interval_attributed_fragments,
            self.endpoint_noop_boundaries_skipped,
            self.collapsed_fragments_rejected,
            self.coverage_gaps_rejected,
            self.foreign_schedule_rows_rejected,
        )
    }
}
