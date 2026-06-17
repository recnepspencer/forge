use worth_spatial::facade::planar_boolean_events::PlanarBooleanSegmentPairWorkItem;
use worth_spatial::facade::planar_local_frame::PlanarLocalFrameCertificateReceipt;
use worth_spatial::facade::planar_segment_segment::{
    CertifiedProjectedSegment2D, CertifiedSegmentSegment2D, CertifiedSegmentSegment2DContracts,
    CertifiedSegmentSegment2DReceipt, SegmentContactPolicy,
};

use super::contract_handles::{predicate_handle, segment_handle};
use super::projection::{project_synthetic_endpoint, TOPOLOGY};

#[derive(Clone, Copy)]
pub(crate) enum SyntheticCollinearRelation {
    Disjoint,
    EndpointTouch,
    EndpointTouchWithFirstReversed,
    EndpointTouchWithSecondReversed,
    PartialOverlap,
    DiagonalPartialOverlapWithSecondReversed,
    ContainmentOverlap,
    IdenticalSameDirection,
    IdenticalAntiParallel,
}

pub(crate) fn segment_receipt_for_relation(
    world: &'static str,
    frame: &PlanarLocalFrameCertificateReceipt,
    work_item: &PlanarBooleanSegmentPairWorkItem,
    relation: SyntheticCollinearRelation,
) -> CertifiedSegmentSegment2DReceipt {
    let left = projected_segment_from_relation_endpoints(world, frame, work_item, true, relation);
    let right = projected_segment_from_relation_endpoints(world, frame, work_item, false, relation);

    CertifiedSegmentSegment2D::classify(left, right)
        .within_topology_basis(TOPOLOGY)
        .with_policy(SegmentContactPolicy::CertifyContactsDenyImprintRequired)
        .compile(&CertifiedSegmentSegment2DContracts::new(
            segment_handle(world),
            predicate_handle(),
        ))
        .expect("collinear-relation segment plan")
        .certify()
        .expect("collinear-relation segment receipt")
}

fn projected_segment_from_relation_endpoints(
    world: &'static str,
    frame: &PlanarLocalFrameCertificateReceipt,
    work_item: &PlanarBooleanSegmentPairWorkItem,
    is_left_segment: bool,
    relation: SyntheticCollinearRelation,
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
    .expect("collinear-relation projected segment")
}

fn relation_points(relation: SyntheticCollinearRelation) -> ([[f64; 2]; 2], [[f64; 2]; 2]) {
    match relation {
        SyntheticCollinearRelation::Disjoint => {
            ([[0.0, 0.0], [1.0, 0.0]], [[2.0, 0.0], [3.0, 0.0]])
        }
        SyntheticCollinearRelation::EndpointTouch => {
            ([[0.0, 0.0], [1.0, 0.0]], [[1.0, 0.0], [2.0, 0.0]])
        }
        SyntheticCollinearRelation::EndpointTouchWithFirstReversed => {
            ([[1.0, 0.0], [0.0, 0.0]], [[1.0, 0.0], [2.0, 0.0]])
        }
        SyntheticCollinearRelation::EndpointTouchWithSecondReversed => {
            ([[0.0, 0.0], [1.0, 0.0]], [[2.0, 0.0], [1.0, 0.0]])
        }
        SyntheticCollinearRelation::PartialOverlap => {
            ([[0.0, 0.0], [2.0, 0.0]], [[1.0, 0.0], [3.0, 0.0]])
        }
        SyntheticCollinearRelation::DiagonalPartialOverlapWithSecondReversed => {
            ([[0.0, 0.0], [4.0, 4.0]], [[6.0, 6.0], [2.0, 2.0]])
        }
        SyntheticCollinearRelation::ContainmentOverlap => {
            ([[0.0, 0.0], [3.0, 0.0]], [[1.0, 0.0], [2.0, 0.0]])
        }
        SyntheticCollinearRelation::IdenticalSameDirection => {
            ([[0.0, 0.0], [1.0, 0.0]], [[0.0, 0.0], [1.0, 0.0]])
        }
        SyntheticCollinearRelation::IdenticalAntiParallel => {
            ([[0.0, 0.0], [1.0, 0.0]], [[1.0, 0.0], [0.0, 0.0]])
        }
    }
}
