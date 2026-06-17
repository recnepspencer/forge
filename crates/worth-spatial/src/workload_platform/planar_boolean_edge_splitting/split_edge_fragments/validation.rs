use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::workload_platform::planar_boolean_edge_splitting::micro_interval_normalization::{
    PlanarBooleanIntervalSubdivisionNormalizedSchedule,
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
};
use crate::workload_platform::planar_boolean_edge_splitting::split_vertex_identity::{
    PlanarBooleanSplitVertexIdentitySchedule, PlanarBooleanSplitVertexIdentitySet,
};

use super::denial::{
    PlanarBooleanSplitEdgeFragmentDenial, PlanarBooleanSplitEdgeFragmentDenialKind,
};
use super::fragment_row::PlanarBooleanSplitEdgeFragment;

pub(super) fn reject_mismatched_vertex_set(
    interval_subdivision_schedule_set_identity: &str,
    split_vertices: &PlanarBooleanSplitVertexIdentitySet,
) -> Result<(), PlanarBooleanSplitEdgeFragmentDenial> {
    if split_vertices.interval_subdivision_schedule_set_identity()
        == interval_subdivision_schedule_set_identity
    {
        return Ok(());
    }
    Err(PlanarBooleanSplitEdgeFragmentDenial::new(
        PlanarBooleanSplitEdgeFragmentDenialKind::MismatchedSplitVertexScheduleSet,
        split_vertices.split_vertex_identity_set_identity(),
        "split edge fragments must consume split vertices minted from the same interval-subdivision schedule set",
    ))
}

pub(super) fn vertex_schedule_index<'a>(
    split_vertices: &'a PlanarBooleanSplitVertexIdentitySet,
) -> BTreeMap<&'a str, &'a PlanarBooleanSplitVertexIdentitySchedule> {
    split_vertices
        .schedules()
        .iter()
        .map(|schedule| (schedule.interval_subdivision_schedule_identity(), schedule))
        .collect()
}

pub(super) fn require_vertex_schedule<'a>(
    schedule: &PlanarBooleanIntervalSubdivisionNormalizedSchedule,
    index: &BTreeMap<&str, &'a PlanarBooleanSplitVertexIdentitySchedule>,
) -> Result<&'a PlanarBooleanSplitVertexIdentitySchedule, PlanarBooleanSplitEdgeFragmentDenial> {
    let Some(vertex_schedule) = index.get(schedule.schedule_identity()).copied() else {
        return Err(PlanarBooleanSplitEdgeFragmentDenial::new(
            PlanarBooleanSplitEdgeFragmentDenialKind::MissingSplitVertexSchedule,
            schedule.schedule_identity(),
            "each interval-subdivision schedule requires its corresponding split-vertex schedule",
        ));
    };
    reject_foreign_vertex_schedule(schedule, vertex_schedule)?;
    Ok(vertex_schedule)
}

pub(super) fn reject_mixed_schedule_set_basis(
    schedule_set: &PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    split_vertices: &PlanarBooleanSplitVertexIdentitySet,
) -> Result<(), PlanarBooleanSplitEdgeFragmentDenial> {
    let mut bases = BTreeSet::new();
    for vertex in split_vertices.vertices() {
        bases.insert((
            vertex.local_frame_identity().to_string(),
            vertex.precision_basis_identity().to_string(),
        ));
    }
    for schedule in schedule_set.schedules() {
        for subdivision in schedule.interval_subdivisions() {
            bases.insert((
                subdivision.local_frame_identity().to_string(),
                subdivision.precision_basis_identity().to_string(),
            ));
        }
        for cut in schedule.fragment_cuts() {
            bases.insert((
                cut.local_frame_identity().to_string(),
                cut.precision_basis_identity().to_string(),
            ));
        }
    }
    if bases.len() <= 1 {
        return Ok(());
    }
    Err(PlanarBooleanSplitEdgeFragmentDenial::new(
        PlanarBooleanSplitEdgeFragmentDenialKind::AmbiguousFragmentBasis,
        schedule_set.schedule_set_identity(),
        "split fragment schedule set must not mix frame or precision bases before full-domain fragments are constructed",
    ))
}

pub(super) fn reject_foreign_schedule_rows(
    schedule: &PlanarBooleanIntervalSubdivisionNormalizedSchedule,
) -> Result<(), PlanarBooleanSplitEdgeFragmentDenial> {
    for cut in schedule.fragment_cuts() {
        if cut.source_edge_identity() != schedule.source_edge_identity()
            || cut.carrier_identity() != schedule.carrier_identity()
        {
            return Err(PlanarBooleanSplitEdgeFragmentDenial::new(
                PlanarBooleanSplitEdgeFragmentDenialKind::ForeignIntervalSubdivisionSchedule,
                cut.cut_identity(),
                "fragment point cuts must belong to their interval-subdivision schedule",
            ));
        }
    }
    for subdivision in schedule.interval_subdivisions() {
        if subdivision.source_edge_identity() != schedule.source_edge_identity()
            || subdivision.carrier_identity() != schedule.carrier_identity()
        {
            return Err(PlanarBooleanSplitEdgeFragmentDenial::new(
                PlanarBooleanSplitEdgeFragmentDenialKind::ForeignIntervalSubdivisionSchedule,
                subdivision.subdivision_identity(),
                "fragment interval rows must belong to their interval-subdivision schedule",
            ));
        }
    }
    Ok(())
}

pub(super) fn reject_fragment_coverage_gaps(
    schedule_identity: &str,
    fragments: &[PlanarBooleanSplitEdgeFragment],
) -> Result<(), PlanarBooleanSplitEdgeFragmentDenial> {
    let Some(first) = fragments.first() else {
        return Err(PlanarBooleanSplitEdgeFragmentDenial::new(
            PlanarBooleanSplitEdgeFragmentDenialKind::GapInSourceEdgeCoverage,
            schedule_identity,
            "split fragment construction must emit at least one fragment per source edge",
        ));
    };
    if first.parameter_range()[0] != 0.0 {
        return Err(PlanarBooleanSplitEdgeFragmentDenial::new(
            PlanarBooleanSplitEdgeFragmentDenialKind::GapInSourceEdgeCoverage,
            first.fragment_identity(),
            "first split fragment must begin at source parameter zero",
        ));
    }
    let mut prior_end = first.parameter_range()[1];
    for fragment in fragments.iter().skip(1) {
        let start = fragment.parameter_range()[0];
        if start > prior_end {
            return Err(PlanarBooleanSplitEdgeFragmentDenial::new(
                PlanarBooleanSplitEdgeFragmentDenialKind::GapInSourceEdgeCoverage,
                fragment.fragment_identity(),
                "adjacent split fragments must not leave source parameter gaps",
            ));
        }
        if start < prior_end {
            return Err(PlanarBooleanSplitEdgeFragmentDenial::new(
                PlanarBooleanSplitEdgeFragmentDenialKind::OverlappingFragmentRange,
                fragment.fragment_identity(),
                "adjacent split fragments must not overlap in source parameter space",
            ));
        }
        prior_end = fragment.parameter_range()[1];
    }
    if prior_end != 1.0 {
        return Err(PlanarBooleanSplitEdgeFragmentDenial::new(
            PlanarBooleanSplitEdgeFragmentDenialKind::GapInSourceEdgeCoverage,
            schedule_identity,
            "last split fragment must end at source parameter one",
        ));
    }
    Ok(())
}

fn reject_foreign_vertex_schedule(
    schedule: &PlanarBooleanIntervalSubdivisionNormalizedSchedule,
    vertex_schedule: &PlanarBooleanSplitVertexIdentitySchedule,
) -> Result<(), PlanarBooleanSplitEdgeFragmentDenial> {
    if vertex_schedule.source_edge_identity() != schedule.source_edge_identity()
        || vertex_schedule.carrier_identity() != schedule.carrier_identity()
    {
        return Err(PlanarBooleanSplitEdgeFragmentDenial::new(
            PlanarBooleanSplitEdgeFragmentDenialKind::ForeignSplitVertexSchedule,
            vertex_schedule.schedule_identity(),
            "split vertex schedule must belong to the fragment source edge and carrier",
        ));
    }
    for vertex in vertex_schedule.vertices() {
        if vertex.source_edge_identity() != schedule.source_edge_identity()
            || vertex.carrier_identity() != schedule.carrier_identity()
        {
            return Err(PlanarBooleanSplitEdgeFragmentDenial::new(
                PlanarBooleanSplitEdgeFragmentDenialKind::ForeignSplitVertexSchedule,
                vertex.split_vertex_identity(),
                "split vertex row must belong to the fragment source edge and carrier",
            ));
        }
    }
    Ok(())
}
