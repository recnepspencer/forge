use worth_math::sign::TriSign;

use crate::planar_contracts::segment_segment_2d::CertifiedSegmentSegment2DClassification;
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanEventClassifierInput, PlanarBooleanPointEventExtractionCounters,
};

use super::coordinate_fact::PlanarBooleanPointEventCoordinateFact;
use super::denial::{
    PlanarBooleanPointEventExtractionDenial, PlanarBooleanPointEventExtractionDenialKind,
};
use super::event::PlanarBooleanPointEvent;
use super::event_kind::PlanarBooleanPointEventKind;
use super::segment_parameter::PlanarBooleanPointEventSegmentParameterFact;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum PointContactClassification {
    Emit(Box<PlanarBooleanPointEvent>),
    SkipNonPoint,
}

pub(crate) fn classify_point_contact(
    input: PlanarBooleanEventClassifierInput<'_>,
) -> Result<PointContactClassification, PlanarBooleanPointEventExtractionDenialKind> {
    match input.bound_pair().classification() {
        CertifiedSegmentSegment2DClassification::ProperCrossing => proper_crossing_event(input)
            .map(Box::new)
            .map(PointContactClassification::Emit),
        CertifiedSegmentSegment2DClassification::EndpointTouch => endpoint_touch_event(input),
        CertifiedSegmentSegment2DClassification::PolicyRequiredOrUncertain => {
            Err(PlanarBooleanPointEventExtractionDenialKind::AmbiguousPredicateRelation)
        }
        CertifiedSegmentSegment2DClassification::Disjoint
        | CertifiedSegmentSegment2DClassification::CollinearDisjoint
        | CertifiedSegmentSegment2DClassification::CollinearOverlap
        | CertifiedSegmentSegment2DClassification::Identical
        | CertifiedSegmentSegment2DClassification::ReverseIdentical => {
            Ok(PointContactClassification::SkipNonPoint)
        }
    }
}

pub(crate) fn denial_for_kind(
    kind: PlanarBooleanPointEventExtractionDenialKind,
    input: PlanarBooleanEventClassifierInput<'_>,
    counters: PlanarBooleanPointEventExtractionCounters,
) -> PlanarBooleanPointEventExtractionDenial {
    PlanarBooleanPointEventExtractionDenial::new(
        kind,
        input.predicate_binding_identity(),
        input.segment_pair_identity(),
        counters,
        format!(
            "point-event extraction could not certify {} for segment pair {}",
            input.predicate_bound_pair_identity(),
            input.segment_pair_identity()
        ),
    )
}

fn proper_crossing_event(
    input: PlanarBooleanEventClassifierInput<'_>,
) -> Result<PlanarBooleanPointEvent, PlanarBooleanPointEventExtractionDenialKind> {
    let bound_pair = input.bound_pair();
    let basis = bound_pair.segment_basis();
    let a0 = basis.first_start_point_2d();
    let a1 = basis.first_end_point_2d();
    let b0 = basis.second_start_point_2d();
    let b1 = basis.second_end_point_2d();
    let r = subtract(a1, a0);
    let s = subtract(b1, b0);
    let denominator = cross(r, s);
    if denominator == 0.0 {
        return Err(PlanarBooleanPointEventExtractionDenialKind::DegenerateSegmentParameterBasis);
    }
    let delta = subtract(b0, a0);
    let a_parameter = cross(delta, s) / denominator;
    let b_parameter = cross(delta, r) / denominator;
    let point = add(a0, scale(r, a_parameter));
    make_event(
        input,
        PlanarBooleanPointEventKind::ProperInteriorInteriorCrossing,
        point_event_relation(point, a_parameter, b_parameter),
    )
}

fn endpoint_touch_event(
    input: PlanarBooleanEventClassifierInput<'_>,
) -> Result<PointContactClassification, PlanarBooleanPointEventExtractionDenialKind> {
    let signs = input.bound_pair().segment_basis().orientation_signs();
    if signs[2] == TriSign::Zero {
        if let Ok(event) = endpoint_a_on_b(input, true) {
            return Ok(PointContactClassification::Emit(Box::new(event)));
        }
    }
    if signs[3] == TriSign::Zero {
        if let Ok(event) = endpoint_a_on_b(input, false) {
            return Ok(PointContactClassification::Emit(Box::new(event)));
        }
    }
    if signs[0] == TriSign::Zero {
        if let Ok(event) = endpoint_b_on_a(input, true) {
            return Ok(PointContactClassification::Emit(Box::new(event)));
        }
    }
    if signs[1] == TriSign::Zero {
        if let Ok(event) = endpoint_b_on_a(input, false) {
            return Ok(PointContactClassification::Emit(Box::new(event)));
        }
    }
    Err(PlanarBooleanPointEventExtractionDenialKind::MissingInteriorEndpointWitness)
}

fn endpoint_a_on_b(
    input: PlanarBooleanEventClassifierInput<'_>,
    is_start: bool,
) -> Result<PlanarBooleanPointEvent, PlanarBooleanPointEventExtractionDenialKind> {
    let basis = input.bound_pair().segment_basis();
    let endpoint = if is_start {
        basis.first_start_point_2d()
    } else {
        basis.first_end_point_2d()
    };
    let b0 = basis.second_start_point_2d();
    let b1 = basis.second_end_point_2d();
    let b_parameter = segment_parameter(endpoint, b0, b1)?;
    let a_parameter = if is_start { 0.0 } else { 1.0 };
    let a_endpoint_index = if is_start { 0 } else { 1 };
    if is_strict_interior_parameter(b_parameter) {
        return make_event(
            input,
            PlanarBooleanPointEventKind::OperandAEndpointOnOperandBInterior,
            point_event_relation(endpoint, a_parameter, b_parameter),
        );
    }
    make_shared_endpoint_event(
        input,
        endpoint,
        a_parameter,
        b_parameter,
        [
            a_endpoint_index,
            endpoint_index_from_parameter(b_parameter, 2, 3)?,
        ],
    )
}

fn endpoint_b_on_a(
    input: PlanarBooleanEventClassifierInput<'_>,
    is_start: bool,
) -> Result<PlanarBooleanPointEvent, PlanarBooleanPointEventExtractionDenialKind> {
    let basis = input.bound_pair().segment_basis();
    let endpoint = if is_start {
        basis.second_start_point_2d()
    } else {
        basis.second_end_point_2d()
    };
    let a0 = basis.first_start_point_2d();
    let a1 = basis.first_end_point_2d();
    let a_parameter = segment_parameter(endpoint, a0, a1)?;
    let b_parameter = if is_start { 0.0 } else { 1.0 };
    let b_endpoint_index = if is_start { 2 } else { 3 };
    if is_strict_interior_parameter(a_parameter) {
        return make_event(
            input,
            PlanarBooleanPointEventKind::OperandBEndpointOnOperandAInterior,
            point_event_relation(endpoint, a_parameter, b_parameter),
        );
    }
    make_shared_endpoint_event(
        input,
        endpoint,
        a_parameter,
        b_parameter,
        [
            endpoint_index_from_parameter(a_parameter, 0, 1)?,
            b_endpoint_index,
        ],
    )
}

fn make_event(
    input: PlanarBooleanEventClassifierInput<'_>,
    kind: PlanarBooleanPointEventKind,
    relation: PointEventRelation,
) -> Result<PlanarBooleanPointEvent, PlanarBooleanPointEventExtractionDenialKind> {
    make_point_event(input, kind, relation, None)
}

fn make_shared_endpoint_event(
    input: PlanarBooleanEventClassifierInput<'_>,
    point: [f64; 2],
    a_parameter: f64,
    b_parameter: f64,
    endpoint_indices: [usize; 2],
) -> Result<PlanarBooleanPointEvent, PlanarBooleanPointEventExtractionDenialKind> {
    make_point_event(
        input,
        PlanarBooleanPointEventKind::SharedEndpoint,
        point_event_relation(point, a_parameter, b_parameter),
        Some(endpoint_indices),
    )
}

fn make_point_event(
    input: PlanarBooleanEventClassifierInput<'_>,
    kind: PlanarBooleanPointEventKind,
    relation: PointEventRelation,
    shared_endpoint_indices: Option<[usize; 2]>,
) -> Result<PlanarBooleanPointEvent, PlanarBooleanPointEventExtractionDenialKind> {
    let point = relation.point;
    if !point[0].is_finite() || !point[1].is_finite() {
        return Err(PlanarBooleanPointEventExtractionDenialKind::NonFinitePointEventCoordinate);
    }
    let bound_pair = input.bound_pair();
    let coordinate_fact = PlanarBooleanPointEventCoordinateFact::new(
        point,
        bound_pair.local_frame_identity(),
        bound_pair.precision_basis_identity(),
    );
    let operand_a_parameter = PlanarBooleanPointEventSegmentParameterFact::new(
        bound_pair.left_segment_identity(),
        bound_pair.left_carrier_identity(),
        relation.a_parameter,
    );
    let operand_b_parameter = PlanarBooleanPointEventSegmentParameterFact::new(
        bound_pair.right_segment_identity(),
        bound_pair.right_carrier_identity(),
        relation.b_parameter,
    );
    Ok(match shared_endpoint_indices {
        Some(endpoint_indices) => PlanarBooleanPointEvent::new_shared_endpoint(
            coordinate_fact,
            operand_a_parameter,
            operand_b_parameter,
            bound_pair,
            endpoint_indices,
        ),
        None => PlanarBooleanPointEvent::new(
            kind,
            coordinate_fact,
            operand_a_parameter,
            operand_b_parameter,
            bound_pair,
        ),
    })
}

fn segment_parameter(
    point: [f64; 2],
    start: [f64; 2],
    end: [f64; 2],
) -> Result<f64, PlanarBooleanPointEventExtractionDenialKind> {
    let direction = subtract(end, start);
    let denominator = dot(direction, direction);
    if denominator == 0.0 {
        return Err(PlanarBooleanPointEventExtractionDenialKind::DegenerateSegmentParameterBasis);
    }
    Ok(dot(subtract(point, start), direction) / denominator)
}

fn is_strict_interior_parameter(parameter: f64) -> bool {
    parameter > 0.0 && parameter < 1.0
}

fn endpoint_index_from_parameter(
    parameter: f64,
    start_index: usize,
    end_index: usize,
) -> Result<usize, PlanarBooleanPointEventExtractionDenialKind> {
    if parameter == 0.0 {
        Ok(start_index)
    } else if parameter == 1.0 {
        Ok(end_index)
    } else {
        Err(PlanarBooleanPointEventExtractionDenialKind::MissingInteriorEndpointWitness)
    }
}

struct PointEventRelation {
    point: [f64; 2],
    a_parameter: f64,
    b_parameter: f64,
}

fn point_event_relation(point: [f64; 2], a_parameter: f64, b_parameter: f64) -> PointEventRelation {
    PointEventRelation {
        point,
        a_parameter,
        b_parameter,
    }
}

fn subtract(left: [f64; 2], right: [f64; 2]) -> [f64; 2] {
    [left[0] - right[0], left[1] - right[1]]
}

fn add(left: [f64; 2], right: [f64; 2]) -> [f64; 2] {
    [left[0] + right[0], left[1] + right[1]]
}

fn scale(vector: [f64; 2], scalar: f64) -> [f64; 2] {
    [vector[0] * scalar, vector[1] * scalar]
}

fn cross(left: [f64; 2], right: [f64; 2]) -> f64 {
    left[0] * right[1] - left[1] * right[0]
}

fn dot(left: [f64; 2], right: [f64; 2]) -> f64 {
    left[0] * right[0] + left[1] * right[1]
}
