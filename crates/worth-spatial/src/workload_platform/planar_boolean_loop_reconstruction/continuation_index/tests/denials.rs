use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    duplicate_first_fragment_for_continuation_slot, prepared_loop_continuation_subject,
    source_provenance_with_missing_fragment_membership, split_vertices_without_first_vertex,
    LoopFixtureEntryOrder,
};

use super::super::{
    PlanarBooleanFragmentContinuationDenialKind, PlanarBooleanFragmentContinuationIndex,
    PlanarBooleanFragmentContinuationIndexInput,
};

#[test]
fn fragment_continuation_index_rejects_missing_split_vertex_binding() {
    let prepared = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Canonical);
    let missing_vertices = split_vertices_without_first_vertex(&prepared.subject.vertices);

    let denial = PlanarBooleanFragmentContinuationIndex::admit(
        PlanarBooleanFragmentContinuationIndexInput::from_request_and_provenance(
            &prepared.request,
            &prepared.source_provenance,
            &missing_vertices,
            &prepared.subject.fragments,
            &prepared.subject.overlap_chains,
        ),
    )
    .expect_err("missing split vertex binding must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanFragmentContinuationDenialKind::MissingSplitVertexBinding
    );
}

#[test]
fn fragment_continuation_index_rejects_duplicate_continuation_slots() {
    let prepared = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Canonical);
    let duplicate_fragments =
        duplicate_first_fragment_for_continuation_slot(&prepared.subject.fragments);

    let denial = PlanarBooleanFragmentContinuationIndex::admit(
        PlanarBooleanFragmentContinuationIndexInput::from_request_and_provenance(
            &prepared.request,
            &prepared.source_provenance,
            &prepared.subject.vertices,
            &duplicate_fragments,
            &prepared.subject.overlap_chains,
        ),
    )
    .expect_err("duplicate continuation slots must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanFragmentContinuationDenialKind::DuplicateContinuationSlot
    );
}

#[test]
fn fragment_continuation_index_rejects_dangling_fragment_membership_before_policy() {
    let prepared = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Canonical);
    let poisoned_provenance =
        source_provenance_with_missing_fragment_membership(&prepared.source_provenance);

    let denial = PlanarBooleanFragmentContinuationIndex::admit(
        PlanarBooleanFragmentContinuationIndexInput::from_request_and_provenance(
            &prepared.request,
            &poisoned_provenance,
            &prepared.subject.vertices,
            &prepared.subject.fragments,
            &prepared.subject.overlap_chains,
        ),
    )
    .expect_err("dangling fragment membership must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanFragmentContinuationDenialKind::MissingFragmentMembership
    );
}
