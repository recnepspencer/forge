use crate::workload_platform::planar_boolean_edge_splitting::canonical_parameter::canonical_parameter_bits;
use crate::workload_platform::planar_boolean_edge_splitting::micro_interval_normalization::PlanarBooleanIntervalSubdivisionNormalizedSchedule;
use crate::workload_platform::planar_boolean_edge_splitting::split_vertex_identity::{
    PlanarBooleanSplitVertexIdentityRow, PlanarBooleanSplitVertexIdentitySchedule,
};

use super::denial::{
    PlanarBooleanSplitEdgeFragmentDenial, PlanarBooleanSplitEdgeFragmentDenialKind,
};
use super::endpoint_ref::{
    PlanarBooleanSplitEdgeFragmentEndpointKind, PlanarBooleanSplitEdgeFragmentEndpointRef,
};

#[derive(Clone, Debug)]
pub(super) struct FragmentBoundary {
    pub(super) parameter: f64,
    pub(super) parameter_bits: u64,
    pub(super) endpoint: PlanarBooleanSplitEdgeFragmentEndpointRef,
    pub(super) point_cut_identities: Vec<String>,
    pub(super) cause_provenance_identities: Vec<String>,
    pub(super) event_group_identities: Vec<String>,
}

pub(super) fn boundary_partition_for_schedule(
    schedule: &PlanarBooleanIntervalSubdivisionNormalizedSchedule,
    vertex_schedule: &PlanarBooleanSplitVertexIdentitySchedule,
    fallback_basis: Option<(&str, &str)>,
) -> Result<Vec<FragmentBoundary>, PlanarBooleanSplitEdgeFragmentDenial> {
    let (local_frame_identity, precision_basis_identity) =
        schedule_basis(schedule, vertex_schedule, fallback_basis)?;
    let mut boundaries = Vec::with_capacity(vertex_schedule.vertices().len() + 2);
    boundaries.push(FragmentBoundary {
        parameter: 0.0,
        parameter_bits: canonical_parameter_bits(0.0),
        endpoint: PlanarBooleanSplitEdgeFragmentEndpointRef::original_source_start(
            schedule.source_edge_identity(),
            schedule.carrier_identity(),
            &local_frame_identity,
            &precision_basis_identity,
        ),
        point_cut_identities: Vec::new(),
        cause_provenance_identities: Vec::new(),
        event_group_identities: Vec::new(),
    });
    for vertex in vertex_schedule.vertices() {
        reject_non_finite_boundary(
            vertex.split_vertex_identity(),
            vertex.normalized_parameter(),
        )?;
        boundaries.push(boundary_from_vertex(vertex));
    }
    boundaries.push(FragmentBoundary {
        parameter: 1.0,
        parameter_bits: canonical_parameter_bits(1.0),
        endpoint: PlanarBooleanSplitEdgeFragmentEndpointRef::original_source_end(
            schedule.source_edge_identity(),
            schedule.carrier_identity(),
            &local_frame_identity,
            &precision_basis_identity,
        ),
        point_cut_identities: Vec::new(),
        cause_provenance_identities: Vec::new(),
        event_group_identities: Vec::new(),
    });
    boundaries.sort_by(|a, b| a.parameter.total_cmp(&b.parameter));
    reject_unordered_boundaries(schedule.schedule_identity(), &boundaries)?;
    deduplicate_boundaries(schedule.schedule_identity(), boundaries)
}

fn boundary_from_vertex(vertex: &PlanarBooleanSplitVertexIdentityRow) -> FragmentBoundary {
    FragmentBoundary {
        parameter: vertex.normalized_parameter(),
        parameter_bits: vertex.normalized_parameter_bits(),
        endpoint: PlanarBooleanSplitEdgeFragmentEndpointRef::split_vertex(
            vertex.split_vertex_identity(),
            vertex.source_edge_identity(),
            vertex.carrier_identity(),
            vertex.normalized_parameter_bits(),
            vertex.local_frame_identity(),
            vertex.precision_basis_identity(),
        ),
        point_cut_identities: vertex.point_cut_identities().to_vec(),
        cause_provenance_identities: vertex.coalescence_provenance().to_vec(),
        event_group_identities: vertex.event_group_identities().to_vec(),
    }
}

fn schedule_basis(
    schedule: &PlanarBooleanIntervalSubdivisionNormalizedSchedule,
    vertex_schedule: &PlanarBooleanSplitVertexIdentitySchedule,
    fallback_basis: Option<(&str, &str)>,
) -> Result<(String, String), PlanarBooleanSplitEdgeFragmentDenial> {
    if let Some(vertex) = vertex_schedule.vertices().first() {
        return reject_mixed_vertex_schedule_basis(vertex_schedule).map(|()| {
            (
                vertex.local_frame_identity().to_string(),
                vertex.precision_basis_identity().to_string(),
            )
        });
    }
    if let Some(subdivision) = schedule.interval_subdivisions().first() {
        return Ok((
            subdivision.local_frame_identity().to_string(),
            subdivision.precision_basis_identity().to_string(),
        ));
    }
    if let Some(cut) = schedule.fragment_cuts().first() {
        return Ok((
            cut.local_frame_identity().to_string(),
            cut.precision_basis_identity().to_string(),
        ));
    }
    if let Some((local_frame_identity, precision_basis_identity)) = fallback_basis {
        return Ok((
            local_frame_identity.to_string(),
            precision_basis_identity.to_string(),
        ));
    }
    Err(PlanarBooleanSplitEdgeFragmentDenial::new(
        PlanarBooleanSplitEdgeFragmentDenialKind::MissingFragmentProvenance,
        schedule.schedule_identity(),
        "split fragment construction requires a proof-bearing frame and precision basis",
    ))
}

fn reject_mixed_vertex_schedule_basis(
    vertex_schedule: &PlanarBooleanSplitVertexIdentitySchedule,
) -> Result<(), PlanarBooleanSplitEdgeFragmentDenial> {
    let Some(first) = vertex_schedule.vertices().first() else {
        return Ok(());
    };
    if vertex_schedule.vertices().iter().all(|vertex| {
        vertex.local_frame_identity() == first.local_frame_identity()
            && vertex.precision_basis_identity() == first.precision_basis_identity()
    }) {
        return Ok(());
    }
    Err(PlanarBooleanSplitEdgeFragmentDenial::new(
        PlanarBooleanSplitEdgeFragmentDenialKind::AmbiguousFragmentBasis,
        vertex_schedule.schedule_identity(),
        "split fragment boundaries require one frame and precision basis per source-edge schedule",
    ))
}

fn reject_non_finite_boundary(
    evidence_identity: &str,
    parameter: f64,
) -> Result<(), PlanarBooleanSplitEdgeFragmentDenial> {
    if parameter.is_finite() {
        return Ok(());
    }
    Err(PlanarBooleanSplitEdgeFragmentDenial::new(
        PlanarBooleanSplitEdgeFragmentDenialKind::NonFiniteFragmentBoundary,
        evidence_identity,
        "split fragment boundaries require finite source-edge parameters",
    ))
}

fn reject_unordered_boundaries(
    schedule_identity: &str,
    boundaries: &[FragmentBoundary],
) -> Result<(), PlanarBooleanSplitEdgeFragmentDenial> {
    if boundaries
        .first()
        .is_none_or(|boundary| boundary.parameter > 0.0)
        || boundaries
            .last()
            .is_none_or(|boundary| boundary.parameter < 1.0)
    {
        return Err(PlanarBooleanSplitEdgeFragmentDenial::new(
            PlanarBooleanSplitEdgeFragmentDenialKind::GapInSourceEdgeCoverage,
            schedule_identity,
            "split fragment boundaries must cover the full source-edge parameter domain",
        ));
    }
    if boundaries
        .iter()
        .any(|boundary| boundary.parameter < 0.0 || boundary.parameter > 1.0)
    {
        return Err(PlanarBooleanSplitEdgeFragmentDenial::new(
            PlanarBooleanSplitEdgeFragmentDenialKind::UnorderedFragmentBoundary,
            schedule_identity,
            "split fragment boundaries must remain inside the source-edge parameter domain",
        ));
    }
    Ok(())
}

fn deduplicate_boundaries(
    schedule_identity: &str,
    boundaries: Vec<FragmentBoundary>,
) -> Result<Vec<FragmentBoundary>, PlanarBooleanSplitEdgeFragmentDenial> {
    let mut deduped: Vec<FragmentBoundary> = Vec::with_capacity(boundaries.len());
    for boundary in boundaries {
        if let Some(existing) = deduped
            .iter_mut()
            .find(|existing| existing.parameter_bits == boundary.parameter_bits)
        {
            if existing.endpoint.endpoint_kind()
                == PlanarBooleanSplitEdgeFragmentEndpointKind::SplitVertex
                && boundary.endpoint.endpoint_kind()
                    == PlanarBooleanSplitEdgeFragmentEndpointKind::SplitVertex
            {
                return Err(PlanarBooleanSplitEdgeFragmentDenial::new(
                    PlanarBooleanSplitEdgeFragmentDenialKind::CollapsedSplitFragment,
                    schedule_identity,
                    "duplicate split-vertex boundaries would create a zero-length fragment",
                ));
            }
            if is_original_endpoint(boundary.endpoint.endpoint_kind()) {
                existing.endpoint = boundary.endpoint;
            }
            existing
                .point_cut_identities
                .extend(boundary.point_cut_identities);
            existing
                .cause_provenance_identities
                .extend(boundary.cause_provenance_identities);
            existing
                .event_group_identities
                .extend(boundary.event_group_identities);
            canonicalize_boundary(existing);
            continue;
        }
        deduped.push(boundary);
    }
    Ok(deduped)
}

fn canonicalize_boundary(boundary: &mut FragmentBoundary) {
    boundary.point_cut_identities.sort();
    boundary.point_cut_identities.dedup();
    boundary.cause_provenance_identities.sort();
    boundary.cause_provenance_identities.dedup();
    boundary.event_group_identities.sort();
    boundary.event_group_identities.dedup();
}

fn is_original_endpoint(kind: PlanarBooleanSplitEdgeFragmentEndpointKind) -> bool {
    matches!(
        kind,
        PlanarBooleanSplitEdgeFragmentEndpointKind::OriginalSourceStart
            | PlanarBooleanSplitEdgeFragmentEndpointKind::OriginalSourceEnd
    )
}
