mod contract_handles;
mod predicate_consumption;
mod projection;
mod relation_fixture;

use worth_spatial::facade::planar_segment_segment::CertifiedSegmentSegment2DReceipt;

use super::predicate_binding_support::{self, BindingSubject};

pub(crate) use relation_fixture::SyntheticCollinearRelation;

pub(crate) fn binding_subject_with_relation(
    readiness_scope: &'static str,
    relation: SyntheticCollinearRelation,
) -> BindingSubject {
    let mut subject = predicate_binding_support::binding_subject(readiness_scope);
    subject.segment_receipts =
        certified_segment_receipts_for_relation(readiness_scope, &subject, relation);
    subject.predicate_consumption = predicate_consumption::predicate_consumption_receipt(
        readiness_scope,
        subject.segment_receipts.clone(),
    );
    subject
}

fn certified_segment_receipts_for_relation(
    readiness_scope: &'static str,
    subject: &BindingSubject,
    relation: SyntheticCollinearRelation,
) -> Vec<CertifiedSegmentSegment2DReceipt> {
    let first = &subject.pair_worklist.work_items()[0];
    let frame = projection::certified_collinear_relation_frame(
        readiness_scope,
        first.left().local_frame_identity(),
        first.left().precision_basis_identity(),
    );
    subject
        .pair_worklist
        .work_items()
        .iter()
        .map(|work_item| {
            relation_fixture::segment_receipt_for_relation(
                readiness_scope,
                &frame,
                work_item,
                relation,
            )
        })
        .collect()
}
