use crate::facade::refs::{
    EmptySpatialWitnessCatalog, SpatialAxis, SpatialDirectionWitnessRef, SpatialFrameRef,
    SpatialPointWitnessRef,
};
use crate::witness_resolution::witness_resolution::{
    resolve_spatial_direction_witness_with_catalog, resolve_spatial_point_witness_with_catalog,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthSpatialPhaseEightPerformanceCounters {
    witness_resolution_request_count: usize,
    point_witness_request_count: usize,
    direction_witness_request_count: usize,
    resolved_witness_count: usize,
    denied_witness_count: usize,
    catalog_lookup_request_count: usize,
}

impl WorthSpatialPhaseEightPerformanceCounters {
    pub const fn witness_resolution_request_count(&self) -> usize {
        self.witness_resolution_request_count
    }

    pub const fn point_witness_request_count(&self) -> usize {
        self.point_witness_request_count
    }

    pub const fn direction_witness_request_count(&self) -> usize {
        self.direction_witness_request_count
    }

    pub const fn resolved_witness_count(&self) -> usize {
        self.resolved_witness_count
    }

    pub const fn denied_witness_count(&self) -> usize {
        self.denied_witness_count
    }

    pub const fn catalog_lookup_request_count(&self) -> usize {
        self.catalog_lookup_request_count
    }
}

pub fn current_spatial_phase_eight_performance_counters(
) -> WorthSpatialPhaseEightPerformanceCounters {
    let catalog = EmptySpatialWitnessCatalog;
    let point_requests = [
        (SpatialPointWitnessRef::world_point([1.0, 2.0, 3.0]), false),
        (
            SpatialPointWitnessRef::frame_origin(SpatialFrameRef::workplane(
                "phase-eight-frame",
                [4.0, 5.0, 6.0],
                [0.0, 0.0, 1.0],
            )),
            false,
        ),
        (
            SpatialPointWitnessRef::ambiguous_curve_point("phase-eight-curve"),
            false,
        ),
        (
            SpatialPointWitnessRef::surface_point("phase-eight-surface", 0.25, 0.5),
            true,
        ),
    ];
    let direction_requests = [
        (
            SpatialDirectionWitnessRef::world_direction([0.0, 1.0, 0.0]),
            false,
        ),
        (
            SpatialDirectionWitnessRef::frame_axis(
                SpatialFrameRef::workplane(
                    "phase-eight-direction-frame",
                    [0.0, 0.0, 0.0],
                    [1.0, 0.0, 0.0],
                ),
                SpatialAxis::W,
            ),
            false,
        ),
        (
            SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 0.0]),
            false,
        ),
        (
            SpatialDirectionWitnessRef::surface_normal("phase-eight-surface", 0.25, 0.5),
            true,
        ),
    ];
    let catalog_lookup_request_count = point_requests
        .iter()
        .filter(|(_, requires_catalog)| *requires_catalog)
        .count()
        + direction_requests
            .iter()
            .filter(|(_, requires_catalog)| *requires_catalog)
            .count();
    let point_results = point_requests
        .map(|(request, _)| resolve_spatial_point_witness_with_catalog(request, &catalog).is_ok());
    let direction_results = direction_requests.map(|(request, _)| {
        resolve_spatial_direction_witness_with_catalog(request, &catalog).is_ok()
    });

    let point_witness_request_count = point_results.len();
    let direction_witness_request_count = direction_results.len();
    let resolved_witness_count = point_results
        .iter()
        .chain(direction_results.iter())
        .filter(|resolved| **resolved)
        .count();
    let witness_resolution_request_count =
        point_witness_request_count + direction_witness_request_count;

    WorthSpatialPhaseEightPerformanceCounters {
        witness_resolution_request_count,
        point_witness_request_count,
        direction_witness_request_count,
        resolved_witness_count,
        denied_witness_count: witness_resolution_request_count - resolved_witness_count,
        catalog_lookup_request_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_eight_spatial_witness_counter_report_exposes_exact_resolution_breadth() {
        let counters = current_spatial_phase_eight_performance_counters();

        assert_eq!(counters.witness_resolution_request_count(), 8);
        assert_eq!(counters.point_witness_request_count(), 4);
        assert_eq!(counters.direction_witness_request_count(), 4);
        assert_eq!(counters.resolved_witness_count(), 4);
        assert_eq!(counters.denied_witness_count(), 4);
        assert_eq!(counters.catalog_lookup_request_count(), 2);
    }
}
