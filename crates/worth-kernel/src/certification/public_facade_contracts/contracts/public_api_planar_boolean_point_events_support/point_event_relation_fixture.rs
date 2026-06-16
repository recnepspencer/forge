use worth_spatial::facade::planar_boolean_events::PlanarBooleanSegmentPairWorkItem;
use worth_spatial::facade::planar_local_frame::PlanarLocalFrameCertificateReceipt;
use worth_spatial::facade::planar_segment_segment::{
    CertifiedProjectedSegment2D, CertifiedSegmentSegment2D, CertifiedSegmentSegment2DContracts,
    CertifiedSegmentSegment2DReceipt, SegmentContactPolicy,
};

use super::point_event_contract_handles::{predicate_handle, segment_handle};
use super::point_event_projection::project_synthetic_endpoint;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub(crate) enum SyntheticPointRelation {
    ProperCrossing,
    ProperCrossingReversed,
    OperandAEndpointOnOperandBInterior,
    OperandBEndpointOnOperandAInterior,
    SharedEndpoint,
    SharedEndpointWithDifferentFreeEndpoints,
    PolicyRequiredCollinearOverlap,
    NearEndpointMiss,
    CollinearDisjoint,
    CollinearEndpointTouch,
    CollinearPartialOverlap,
    CollinearContainmentOverlap,
    CollinearIdenticalSameDirection,
    CollinearIdenticalAntiParallel,
}

pub(crate) fn segment_receipt_for_relation(
    world: &'static str,
    frame: &PlanarLocalFrameCertificateReceipt,
    work_item: &PlanarBooleanSegmentPairWorkItem,
    relation: SyntheticPointRelation,
) -> CertifiedSegmentSegment2DReceipt {
    let left = projected_segment_from_relation_endpoints(world, frame, work_item, true, relation);
    let right = projected_segment_from_relation_endpoints(world, frame, work_item, false, relation);

    CertifiedSegmentSegment2D::classify(left, right)
        .within_topology_basis(super::point_event_projection::TOPOLOGY)
        .with_policy(contact_policy(relation))
        .compile(&CertifiedSegmentSegment2DContracts::new(
            segment_handle(world),
            predicate_handle(),
        ))
        .expect("point-event segment plan")
        .certify()
        .expect("point-event segment receipt")
}

fn projected_segment_from_relation_endpoints(
    world: &'static str,
    frame: &PlanarLocalFrameCertificateReceipt,
    work_item: &PlanarBooleanSegmentPairWorkItem,
    is_left_segment: bool,
    relation: SyntheticPointRelation,
) -> CertifiedProjectedSegment2D {
    let (left_points, right_points) = relation_points(relation);
    let points = if is_left_segment {
        left_points
    } else {
        right_points
    };
    let segment = if is_left_segment {
        work_item.left()
    } else {
        work_item.right()
    };
    CertifiedProjectedSegment2D::from_projected_endpoints(
        segment.canonical_segment_identity(),
        project_synthetic_endpoint(world, frame, segment, true, points[0]),
        project_synthetic_endpoint(world, frame, segment, false, points[1]),
    )
    .expect("point-event projected segment")
}

fn relation_points(relation: SyntheticPointRelation) -> ([[f64; 2]; 2], [[f64; 2]; 2]) {
    match relation {
        SyntheticPointRelation::ProperCrossing => {
            ([[0.0, 0.0], [1.0, 1.0]], [[0.0, 1.0], [1.0, 0.0]])
        }
        SyntheticPointRelation::ProperCrossingReversed => {
            ([[1.0, 1.0], [0.0, 0.0]], [[1.0, 0.0], [0.0, 1.0]])
        }
        SyntheticPointRelation::OperandAEndpointOnOperandBInterior => {
            ([[0.5, 0.0], [0.5, 1.0]], [[0.0, 0.0], [1.0, 0.0]])
        }
        SyntheticPointRelation::OperandBEndpointOnOperandAInterior => {
            ([[0.0, 0.0], [1.0, 0.0]], [[0.5, 0.0], [0.5, 1.0]])
        }
        SyntheticPointRelation::SharedEndpoint => {
            ([[0.0, 0.0], [1.0, 0.0]], [[0.0, 0.0], [0.0, 1.0]])
        }
        SyntheticPointRelation::SharedEndpointWithDifferentFreeEndpoints => {
            ([[0.0, 0.0], [2.0, 0.0]], [[0.0, 0.0], [0.0, 2.0]])
        }
        SyntheticPointRelation::PolicyRequiredCollinearOverlap => {
            ([[0.0, 0.0], [1.0, 0.0]], [[0.25, 0.0], [0.75, 0.0]])
        }
        SyntheticPointRelation::NearEndpointMiss => {
            ([[0.5, 0.001], [0.5, 1.0]], [[0.0, 0.0], [1.0, 0.0]])
        }
        SyntheticPointRelation::CollinearDisjoint => {
            ([[0.0, 0.0], [1.0, 0.0]], [[2.0, 0.0], [3.0, 0.0]])
        }
        SyntheticPointRelation::CollinearEndpointTouch => {
            ([[0.0, 0.0], [1.0, 0.0]], [[1.0, 0.0], [2.0, 0.0]])
        }
        SyntheticPointRelation::CollinearPartialOverlap => {
            ([[0.0, 0.0], [2.0, 0.0]], [[1.0, 0.0], [3.0, 0.0]])
        }
        SyntheticPointRelation::CollinearContainmentOverlap => {
            ([[0.0, 0.0], [3.0, 0.0]], [[1.0, 0.0], [2.0, 0.0]])
        }
        SyntheticPointRelation::CollinearIdenticalSameDirection => {
            ([[0.0, 0.0], [1.0, 0.0]], [[0.0, 0.0], [1.0, 0.0]])
        }
        SyntheticPointRelation::CollinearIdenticalAntiParallel => {
            ([[0.0, 0.0], [1.0, 0.0]], [[1.0, 0.0], [0.0, 0.0]])
        }
    }
}

fn contact_policy(relation: SyntheticPointRelation) -> SegmentContactPolicy {
    match relation {
        SyntheticPointRelation::PolicyRequiredCollinearOverlap => {
            SegmentContactPolicy::RequireImprintForCollinearOverlap
        }
        SyntheticPointRelation::ProperCrossing
        | SyntheticPointRelation::ProperCrossingReversed
        | SyntheticPointRelation::OperandAEndpointOnOperandBInterior
        | SyntheticPointRelation::OperandBEndpointOnOperandAInterior
        | SyntheticPointRelation::SharedEndpoint
        | SyntheticPointRelation::SharedEndpointWithDifferentFreeEndpoints
        | SyntheticPointRelation::NearEndpointMiss
        | SyntheticPointRelation::CollinearDisjoint
        | SyntheticPointRelation::CollinearEndpointTouch
        | SyntheticPointRelation::CollinearPartialOverlap
        | SyntheticPointRelation::CollinearContainmentOverlap
        | SyntheticPointRelation::CollinearIdenticalSameDirection
        | SyntheticPointRelation::CollinearIdenticalAntiParallel => {
            SegmentContactPolicy::CertifyContactsDenyImprintRequired
        }
    }
}
