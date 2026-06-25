use crate::graph_read_access_declarations::current_worth_graph_read_requirement_derivation_closeout;
use crate::graph_read_access_inventory::same_semantics_different_provenance_milestone_seven_seeds_for_tests;

use super::common::{only_requirement_record, phase_two_closeout_from_seed, production_seed};

#[test]
fn requirement_rows_preserve_seed_vocabulary_basis_as_trace_not_authority() {
    let phase_two = phase_two_closeout_from_seed(&production_seed());
    let phase_four = current_worth_graph_read_requirement_derivation_closeout(&phase_two)
        .expect("Phase 4 should preserve source trace for every catalog record");

    for record in phase_four.requirement_records() {
        let trace = record.requirement_source_trace();
        assert_eq!(
            trace.catalog_record_digest(),
            record.source_catalog_record_digest()
        );
        assert_eq!(
            trace.catalog_key_digest(),
            record.source_catalog_key_digest()
        );
        assert!(!trace.seed_requirement_evidence_digest().is_empty());
        assert!(!trace.source_row_identities().is_empty());
        assert!(!trace.trace_digest().is_empty());
        for identity in trace.source_row_identities() {
            assert!(trace.trace_digest() != identity.source_path());
            assert!(!identity.source_path().is_empty());
            assert!(!identity.current_caller().is_empty());
        }
        assert!(record
            .derivation_outcome()
            .query_requirement_set_evidence()
            .is_none());
    }
}

#[test]
fn source_trace_identity_changes_with_provenance_not_catalog_semantics() {
    let (left, right) = same_semantics_different_provenance_milestone_seven_seeds_for_tests();
    let left = current_worth_graph_read_requirement_derivation_closeout(
        &phase_two_closeout_from_seed(&left),
    )
    .expect("left provenance fixture should derive Phase 4");
    let right = current_worth_graph_read_requirement_derivation_closeout(
        &phase_two_closeout_from_seed(&right),
    )
    .expect("right provenance fixture should derive Phase 4");

    let left_record = only_requirement_record(&left);
    let right_record = only_requirement_record(&right);
    assert_eq!(
        left_record.source_catalog_key_digest(),
        right_record.source_catalog_key_digest()
    );
    assert_ne!(
        left_record
            .requirement_source_trace()
            .source_row_identities(),
        right_record
            .requirement_source_trace()
            .source_row_identities()
    );
    assert_ne!(
        left_record.requirement_source_trace().trace_digest(),
        right_record.requirement_source_trace().trace_digest()
    );
}
