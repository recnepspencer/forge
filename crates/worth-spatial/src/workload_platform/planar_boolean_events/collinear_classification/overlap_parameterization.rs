use crate::workload_platform::planar_boolean_events::PlanarBooleanPredicateBoundPair;

use super::counters::PlanarBooleanCollinearRelationCounters;
use super::denial::{
    PlanarBooleanCollinearRelationDenial, PlanarBooleanCollinearRelationDenialKind,
};
use super::interval_basis::PlanarBooleanCollinearIntervalBasis;

pub(crate) struct PlanarBooleanCollinearOverlapParameterization {
    left_parameter_range: [f64; 2],
    right_parameter_range: [f64; 2],
}

impl PlanarBooleanCollinearOverlapParameterization {
    pub(crate) fn from_bound_pair(
        bound_pair: &PlanarBooleanPredicateBoundPair,
        counters: &mut PlanarBooleanCollinearRelationCounters,
    ) -> Result<Self, PlanarBooleanCollinearRelationDenial> {
        let basis = bound_pair.segment_basis();
        let a0 = basis.first_start_point_2d();
        let a1 = basis.first_end_point_2d();
        let b0 = basis.second_start_point_2d();
        let b1 = basis.second_end_point_2d();
        let axis =
            dominant_axis(a0, a1).ok_or_else(|| degenerate_collinearity(bound_pair, counters))?;
        let a_min = a0[axis].min(a1[axis]);
        let a_max = a0[axis].max(a1[axis]);
        let b_min = b0[axis].min(b1[axis]);
        let b_max = b0[axis].max(b1[axis]);
        let overlap_start = a_min.max(b_min);
        let overlap_end = a_max.min(b_max);
        Ok(Self {
            left_parameter_range: [
                parameter_on_axis(overlap_start, a0[axis], a1[axis])
                    .ok_or_else(|| degenerate_collinearity(bound_pair, counters))?,
                parameter_on_axis(overlap_end, a0[axis], a1[axis])
                    .ok_or_else(|| degenerate_collinearity(bound_pair, counters))?,
            ],
            right_parameter_range: [
                parameter_on_axis(overlap_start, b0[axis], b1[axis])
                    .ok_or_else(|| degenerate_collinearity(bound_pair, counters))?,
                parameter_on_axis(overlap_end, b0[axis], b1[axis])
                    .ok_or_else(|| degenerate_collinearity(bound_pair, counters))?,
            ],
        })
    }

    pub(crate) fn into_interval_basis(self) -> PlanarBooleanCollinearIntervalBasis {
        PlanarBooleanCollinearIntervalBasis::from_source_ranges(
            self.left_parameter_range,
            self.right_parameter_range,
        )
    }

    pub(crate) fn contains_one_segment(&self) -> bool {
        range_is_full_segment(self.left_parameter_range)
            || range_is_full_segment(self.right_parameter_range)
    }

    pub(crate) fn overlap_start_on_left(&self) -> f64 {
        self.left_parameter_range[0]
    }

    pub(crate) fn overlap_start_on_right(&self) -> f64 {
        self.right_parameter_range[0]
    }
}

fn dominant_axis(start: [f64; 2], end: [f64; 2]) -> Option<usize> {
    let dx = (end[0] - start[0]).abs();
    let dy = (end[1] - start[1]).abs();
    if dx == 0.0 && dy == 0.0 {
        None
    } else {
        Some(usize::from(dy > dx))
    }
}

fn parameter_on_axis(coordinate: f64, start: f64, end: f64) -> Option<f64> {
    let denominator = end - start;
    if denominator == 0.0 {
        return None;
    }
    Some((coordinate - start) / denominator)
}

fn degenerate_collinearity(
    bound_pair: &PlanarBooleanPredicateBoundPair,
    counters: &mut PlanarBooleanCollinearRelationCounters,
) -> PlanarBooleanCollinearRelationDenial {
    counters.unsupported_degenerate_collinear_relation();
    PlanarBooleanCollinearRelationDenial::new(
        PlanarBooleanCollinearRelationDenialKind::UnsupportedDegenerateCollinearity,
        bound_pair.predicate_binding_identity(),
        bound_pair.segment_pair_identity(),
        *counters,
        "collinear relation extraction does not admit zero-length segment parameterization",
    )
}

fn range_is_full_segment(range: [f64; 2]) -> bool {
    let ordered = ordered_range(range);
    ordered[0] == 0.0 && ordered[1] == 1.0
}

pub(crate) fn ordered_range(range: [f64; 2]) -> [f64; 2] {
    if range[0] <= range[1] {
        range
    } else {
        [range[1], range[0]]
    }
}
