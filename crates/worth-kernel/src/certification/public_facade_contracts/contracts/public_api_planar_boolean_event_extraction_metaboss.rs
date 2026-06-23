#[path = "public_api_planar_boolean_collinear_relations_support/mod.rs"]
#[allow(dead_code)]
mod collinear_relation_support;
#[path = "public_api_planar_boolean_event_ledger_support.rs"]
#[allow(dead_code)]
mod event_ledger_support;
#[path = "public_api_planar_boolean_event_extraction_metaboss_support/mod.rs"]
mod metaboss_support;
#[path = "public_api_planar_boolean_point_events_support/mod.rs"]
#[allow(dead_code)]
mod point_event_support;
#[path = "public_api_planar_boolean_event_predicate_binding_support.rs"]
#[allow(dead_code)]
mod predicate_binding_support;
#[path = "public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;

use metaboss_support::{
    assert_event_ledger_shape, assert_public_contract_rejects_synthetic_event_ledger_rows,
    assert_replay_preserves_event_ledger_identity,
    assert_split_handoff_requires_event_ledger_receipt, MetabossEventExtractionSubject,
};

#[test]
fn planar_boolean_event_extraction_metaboss_ledger_is_complete_canonical_and_unforgeable() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject =
            MetabossEventExtractionSubject::certify("phase7.2 metaboss event extraction closeout");
        assert_event_ledger_shape(&subject);
        assert_split_handoff_requires_event_ledger_receipt(&subject);
    });
}

#[test]
fn event_ledger_replay_and_orientation_variation_preserve_canonical_identity() {
    reduced_pair_support::run_with_large_stack(|| {
        let first =
            MetabossEventExtractionSubject::certify("phase7.2 metaboss event extraction replay");
        let second =
            MetabossEventExtractionSubject::certify("phase7.2 metaboss event extraction replay");
        assert_replay_preserves_event_ledger_identity(&first, &second);
    });
}

#[test]
fn event_ledger_public_contract_rejects_synthetic_rows_and_raw_pair_fixtures() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject = MetabossEventExtractionSubject::certify(
            "phase7.2 metaboss event extraction anti theatre",
        );
        assert_public_contract_rejects_synthetic_event_ledger_rows(&subject);
    });
}

#[test]
fn edge_split_consumption_requires_event_ledger_receipt_not_raw_events() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject = MetabossEventExtractionSubject::certify(
            "phase7.2 metaboss event extraction split handoff",
        );
        assert_split_handoff_requires_event_ledger_receipt(&subject);
    });
}
