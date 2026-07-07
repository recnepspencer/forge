#[path = "../public_api_planar_boolean_collinear_relations_support/mod.rs"]
mod collinear_relation_support;
#[path = "../public_api_planar_boolean_event_ledger_support.rs"]
mod event_ledger_support;
#[path = "../public_api_planar_boolean_point_events_support/mod.rs"]
mod point_event_support;
#[path = "../public_api_planar_boolean_event_predicate_binding_support.rs"]
mod predicate_binding_support;
#[path = "../public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;

mod expected_shape;
mod ledger_shape_assertions;
mod replay_assertions;
mod subject;
mod workload_handoff_assertions;

pub(crate) use ledger_shape_assertions::assert_event_ledger_shape;
pub(crate) use replay_assertions::assert_replay_preserves_event_ledger_identity;
pub(crate) use subject::MetabossEventExtractionSubject;
pub(crate) use workload_handoff_assertions::{
    assert_public_contract_rejects_synthetic_event_ledger_rows,
    assert_split_handoff_requires_event_ledger_receipt,
};

const _: () = {
    let _ = assert_event_ledger_shape;
    let _ = assert_replay_preserves_event_ledger_identity;
    let _ = assert_public_contract_rejects_synthetic_event_ledger_rows;
    let _ = assert_split_handoff_requires_event_ledger_receipt;
    let _ = MetabossEventExtractionSubject::certify_from_pair;
    let _ = MetabossEventExtractionSubject::certify_event_carrier;
    let _ = MetabossEventExtractionSubject::policy_stop;
    let _ = MetabossEventExtractionSubject::expected;
    let _ = event_ledger_support::ledger_for_point_relation;
};
