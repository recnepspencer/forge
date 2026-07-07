mod point_event_contract_handles;
mod point_event_predicate_consumption;
mod point_event_projection;
mod point_event_relation_fixture;

use worth_spatial::facade::planar_segment_segment::CertifiedSegmentSegment2DReceipt;

use super::predicate_binding_support::{self, BindingSubject};

pub(crate) use point_event_relation_fixture::SyntheticPointRelation;

pub(crate) fn binding_subject_with_relation(
    readiness_scope: &'static str,
    relation: SyntheticPointRelation,
) -> BindingSubject {
    let mut subject = predicate_binding_support::binding_subject(readiness_scope);
    subject.segment_receipts =
        certified_segment_receipts_for_relation(readiness_scope, &subject, relation);
    subject.predicate_consumption =
        point_event_predicate_consumption::predicate_consumption_receipt(
            readiness_scope,
            subject.segment_receipts.clone(),
        );
    subject
}

pub(crate) fn binding_subject_with_reversed_relation_receipts(
    readiness_scope: &'static str,
    relation: SyntheticPointRelation,
) -> BindingSubject {
    let mut subject = binding_subject_with_relation(readiness_scope, relation);
    subject.segment_receipts.reverse();
    subject.predicate_consumption =
        point_event_predicate_consumption::predicate_consumption_receipt(
            readiness_scope,
            subject.segment_receipts.clone(),
        );
    subject
}

fn certified_segment_receipts_for_relation(
    readiness_scope: &'static str,
    subject: &BindingSubject,
    relation: SyntheticPointRelation,
) -> Vec<CertifiedSegmentSegment2DReceipt> {
    let first = &subject.pair_worklist.work_items()[0];
    let frame = point_event_projection::certified_point_event_frame(
        readiness_scope,
        first.left().local_frame_identity(),
        first.left().precision_basis_identity(),
    );
    subject
        .pair_worklist
        .work_items()
        .iter()
        .map(|work_item| {
            point_event_relation_fixture::segment_receipt_for_relation(
                readiness_scope,
                &frame,
                work_item,
                relation,
            )
        })
        .collect()
}

const _: () = {
    let _ = binding_subject_with_reversed_relation_receipts;
};
