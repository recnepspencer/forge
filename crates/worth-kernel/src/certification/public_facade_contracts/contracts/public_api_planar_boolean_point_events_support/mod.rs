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

#[allow(dead_code)]
pub(crate) fn binding_subject_with_relation_schedule(
    readiness_scope: &'static str,
    relations: &[SyntheticPointRelation],
) -> BindingSubject {
    let mut subject = predicate_binding_support::binding_subject(readiness_scope);
    apply_relation_schedule(readiness_scope, relations, &mut subject);
    subject
}

#[allow(dead_code)]
pub(crate) fn metaboss_binding_subject_with_relation_schedule(
    readiness_scope: &'static str,
    relations: &[SyntheticPointRelation],
) -> BindingSubject {
    let mut subject = predicate_binding_support::metaboss_binding_subject(readiness_scope);
    apply_relation_schedule(readiness_scope, relations, &mut subject);
    subject
}

#[allow(dead_code)]
fn apply_relation_schedule(
    readiness_scope: &'static str,
    relations: &[SyntheticPointRelation],
    subject: &mut BindingSubject,
) {
    subject.segment_receipts =
        certified_segment_receipts_for_relation_schedule(readiness_scope, &subject, relations);
    subject.predicate_consumption =
        point_event_predicate_consumption::predicate_consumption_receipt(
            readiness_scope,
            subject.segment_receipts.clone(),
        );
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

#[allow(dead_code)]
fn certified_segment_receipts_for_relation_schedule(
    readiness_scope: &'static str,
    subject: &BindingSubject,
    relations: &[SyntheticPointRelation],
) -> Vec<CertifiedSegmentSegment2DReceipt> {
    let work_items = subject.pair_worklist.work_items();
    assert!(
        work_items.len() >= relations.len(),
        "metaboss relation schedule must fit inside the catalog-derived worklist"
    );
    let first = &work_items[0];
    let frame = point_event_projection::certified_point_event_frame(
        readiness_scope,
        first.left().local_frame_identity(),
        first.left().precision_basis_identity(),
    );
    work_items
        .iter()
        .enumerate()
        .map(|(index, work_item)| {
            let relation = relations
                .get(index)
                .copied()
                .unwrap_or(SyntheticPointRelation::NearEndpointMiss);
            point_event_relation_fixture::segment_receipt_for_relation(
                readiness_scope,
                &frame,
                work_item,
                relation,
            )
        })
        .collect()
}
