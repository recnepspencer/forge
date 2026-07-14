use super::bundles::{
    rejection_row, unstable_cursor_shape_hostile, unsupported_aggregate_family_hostile,
    unsupported_cdc_result_family_hostile, unsupported_ordering_family_hostile,
    unsupported_traversal_bound_hostile,
};
use super::fixtures;
use crate::harness::collection_matrix::{CollectionPerturbationClass, CollectionRejectionRow};

pub(super) fn rejection_rows() -> Vec<CollectionRejectionRow> {
    vec![
        rejection_row(
            "unsupported-ordering-family",
            CollectionPerturbationClass::CollectionRejection,
            &fixtures::ordered_collection_preflight(),
            unsupported_ordering_family_hostile(),
        ),
        rejection_row(
            "unstable-cursor-shape",
            CollectionPerturbationClass::CollectionRejection,
            &fixtures::ordered_collection_preflight(),
            unstable_cursor_shape_hostile(),
        ),
        rejection_row(
            "unsupported-cdc-result-family",
            CollectionPerturbationClass::CollectionRejection,
            &fixtures::ordered_collection_preflight(),
            unsupported_cdc_result_family_hostile(),
        ),
        rejection_row(
            "unsupported-traversal-bound",
            CollectionPerturbationClass::CollectionRejection,
            &fixtures::ordered_collection_preflight(),
            unsupported_traversal_bound_hostile(),
        ),
        rejection_row(
            "unsupported-aggregate-family",
            CollectionPerturbationClass::CollectionRejection,
            &fixtures::ordered_collection_preflight(),
            unsupported_aggregate_family_hostile(),
        ),
    ]
}
