use super::bundles::canonical_row;
use super::fixtures;
use crate::harness::collection_matrix::{
    CollectionCertificationRow, CollectionHostileExpectation, CollectionPerturbationClass,
};

pub(super) fn canonical_rows() -> Vec<CollectionCertificationRow> {
    vec![
        canonical_row(
            "ordered-collection-parity",
            CollectionPerturbationClass::OrderedCollectionParity,
            CollectionHostileExpectation::EquivalentToControl,
            fixtures::ordered_collection_preflight(),
            fixtures::ordered_collection_preflight(),
            fixtures::replay_ordered_collection_preflight(),
        ),
        canonical_row(
            "cursor-advance-repeatability",
            CollectionPerturbationClass::CursorRepeatability,
            CollectionHostileExpectation::EquivalentToControl,
            fixtures::ordered_collection_preflight(),
            fixtures::replay_ordered_collection_preflight(),
            fixtures::ordered_collection_preflight(),
        ),
        canonical_row(
            "bounded-traversal-parity",
            CollectionPerturbationClass::TraversalBoundParity,
            CollectionHostileExpectation::EquivalentToControl,
            fixtures::ordered_collection_preflight(),
            fixtures::ordered_collection_preflight(),
            fixtures::replay_ordered_collection_preflight(),
        ),
        canonical_row(
            "aggregate-rollup-parity",
            CollectionPerturbationClass::AggregateRollupParity,
            CollectionHostileExpectation::EquivalentToControl,
            fixtures::aggregate_rollup_collection_preflight(),
            fixtures::aggregate_rollup_collection_preflight(),
            fixtures::replay_aggregate_rollup_collection_preflight(),
        ),
        canonical_row(
            "derived-field-parity",
            CollectionPerturbationClass::DerivedFieldParity,
            CollectionHostileExpectation::EquivalentToControl,
            fixtures::derived_field_collection_preflight(),
            fixtures::derived_field_collection_preflight(),
            fixtures::replay_derived_field_collection_preflight(),
        ),
        canonical_row(
            "ordering-difference",
            CollectionPerturbationClass::OrderingDifference,
            CollectionHostileExpectation::DistinctFromControl,
            fixtures::ordered_collection_preflight(),
            fixtures::descending_collection_preflight(),
            fixtures::ordered_collection_preflight(),
        ),
        canonical_row(
            "cdc-shaped-result-parity",
            CollectionPerturbationClass::CdcResultDifference,
            CollectionHostileExpectation::DistinctFromControl,
            fixtures::ordered_collection_preflight(),
            fixtures::cdc_collection_preflight(),
            fixtures::ordered_collection_preflight(),
        ),
    ]
}
