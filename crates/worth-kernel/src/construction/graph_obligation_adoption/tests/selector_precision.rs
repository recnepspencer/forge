use std::collections::BTreeSet;

use super::super::primitive_construction_graph_obligation_selector_precision_matrix;
use crate::query_obligation_selection::selection_substrate::QuerySelectorPrecisionPosture;

#[test]
fn kernel_construction_selector_precision_matrix_proves_exact_birth_boundary() {
    let rows = primitive_construction_graph_obligation_selector_precision_matrix()
        .expect("selector precision matrix");

    assert_eq!(rows.len(), 6);
    assert!(rows.iter().all(|row| !row.descriptor_digest().is_empty()));
    assert_eq!(
        rows.iter()
            .map(|row| row.descriptor_digest())
            .collect::<BTreeSet<_>>()
            .len(),
        rows.len()
    );
    assert!(rows
        .iter()
        .all(|row| row.selected_count() == row.expected_selected_count()));
    for row in &rows {
        assert!(row.touch_lookup_key_count() > 0);
        assert!(row.operating_world_lookup_key_count() > 0);
        assert_eq!(
            row.attempted_bucket_lookup_count(),
            row.touch_lookup_key_count() * row.operating_world_lookup_key_count()
        );
        assert_eq!(row.registration_full_scan_count(), 0);
        assert!(row.matched_bucket_count() <= row.attempted_bucket_lookup_count());
        assert!(row.deduplicated_candidate_count() <= row.candidate_registration_count());
        assert_eq!(
            row.candidate_registration_count(),
            row.selected_count() + row.denied_row_count()
        );
        assert_eq!(row.residue_row_count(), 0);
        assert_eq!(
            row.precision_posture(),
            QuerySelectorPrecisionPosture::TouchedDescriptorBounded
        );
        assert!(!row.precision_report_digest().is_empty());
        assert!(!row.precision_counters_digest().is_empty());
    }
    assert_eq!(
        rows.iter()
            .map(|row| row.candidate_registration_count())
            .sum::<usize>(),
        rows.iter().map(|row| row.selected_count()).sum::<usize>()
            + rows.iter().map(|row| row.denied_row_count()).sum::<usize>()
    );
    assert_eq!(
        rows.iter()
            .map(|row| row.residue_row_count())
            .sum::<usize>(),
        0
    );
    assert_eq!(
        rows.iter()
            .filter(|row| row.selected_count() == 0)
            .map(|row| row.label())
            .collect::<Vec<_>>(),
        vec![
            "unrelated-collection",
            "wrong-mutation-family",
            "wrong-aspect-operation",
            "wrong-aspect-path",
            "read-not-mutation"
        ]
    );
}
