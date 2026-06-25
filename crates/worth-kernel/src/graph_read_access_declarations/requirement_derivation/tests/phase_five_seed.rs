use crate::graph_read_access_declarations::current_worth_graph_read_requirement_derivation_closeout;

use super::common::{phase_two_closeout_from_seed, production_seed};

#[test]
fn requirement_derivation_carries_query_family_anchor_to_phase_five() {
    let phase_two = phase_two_closeout_from_seed(&production_seed());
    let phase_four = current_worth_graph_read_requirement_derivation_closeout(&phase_two)
        .expect("Phase 4 should preserve Query family anchor identity");

    for record in phase_four.requirement_records() {
        let source_record = phase_two
            .declaration_catalog()
            .records()
            .iter()
            .find(|candidate| {
                candidate.declaration_identity_digest() == record.source_catalog_record_digest()
            })
            .expect("Phase 4 record should point at a Phase 2 catalog record");
        assert_eq!(
            record.query_family_name(),
            source_record.query_family_anchor().family_name()
        );
        assert_eq!(
            record.query_family_digest_seed(),
            source_record.query_family_anchor().family_digest_seed()
        );
        assert_eq!(record.query_family_admission_boundary(), "kernel_only");
        assert_eq!(
            record.derivation_attempt().catalog_record_digest(),
            record.source_catalog_record_digest()
        );
        assert_eq!(
            record.derivation_attempt().query_family_anchor_digest(),
            record.query_family_digest_seed()
        );
        assert!(!record.derivation_attempt().has_query_read_family_artifact());
        assert!(!record.derivation_attempt().has_access_shape_artifact());
        assert!(!record.derivation_attempt().has_selectivity_shape_artifact());
        assert!(!record
            .derivation_attempt()
            .attempted_query_requirement_derivation());
        assert!(record
            .query_family_name()
            .starts_with("worth_graph_read_family_"));
    }

    assert!(phase_four
        .phase_five_seed()
        .requirement_records()
        .iter()
        .all(|record| !record.query_family_digest_seed().is_empty()));
}
