pub(crate) fn candidate_rows_have_provenance(
    rows: &[worth_spatial::facade::planar_boolean_events::PlanarBooleanSegmentCandidateRowReceipt],
) -> bool {
    rows.iter().all(|row| {
        !row.candidate_identity().is_empty()
            && !row.left_source_face_identity().is_empty()
            && !row.left_source_loop_identity().is_empty()
            && !row.left_source_edge_identity().is_empty()
            && !row.right_source_face_identity().is_empty()
            && !row.right_source_loop_identity().is_empty()
            && !row.right_source_edge_identity().is_empty()
            && !row.local_frame_identity().is_empty()
            && !row.precision_basis_identity().is_empty()
    })
}
