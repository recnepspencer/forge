use super::*;
use crate::lower_runtime_routing::{
    worth_query_lower_runtime_closeout_registry, WorthQueryLowerRuntimeCrossingClassification,
    WorthQueryLowerRuntimeSeamKey,
};

#[test]
fn support_matrix_rows_cover_crossings_and_closeout_registry() {
    let inventory = super::super::worth_query_lower_runtime_crossing_inventory();
    let closeout = worth_query_lower_runtime_closeout_registry();
    let support = worth_query_lower_runtime_support_matrix();

    assert_eq!(
        support.rows().len(),
        inventory.rows().len() + closeout.rows().len()
    );
    for crossing in inventory.rows() {
        let support_row = support
            .support_for(crossing.seam_key())
            .expect("support matrix must cover every crossing row");
        assert_eq!(support_row.capability_label(), crossing.capability_label());
        assert_eq!(
            support_row.authority_owner(),
            crossing.lower_runtime_owner()
        );
        assert_eq!(support_row.route_kind(), crossing.route_kind());
        assert_eq!(
            support_row.posture(),
            support_posture_for_classification(crossing.classification())
        );
        assert_eq!(
            support_row.detail(),
            WorthQueryLowerRuntimeSupportDetail::Crossing
        );
    }
    for row in closeout.rows() {
        let support_row = support
            .support_for(row.seam_key())
            .expect("support matrix must cover every closeout row");
        assert_eq!(support_row.capability_label(), row.capability_label());
        assert_eq!(support_row.authority_owner(), row.owner());
        assert_eq!(support_row.route_kind(), row.route_kind());
        assert_eq!(
            support_row.posture(),
            support_posture_for_closeout(row.posture())
        );
        assert_eq!(support_row.closeout_target(), Some(row.closeout_target()));
        assert_eq!(
            support_row.required_closeout(),
            Some(row.required_closeout())
        );
        assert_eq!(
            support_row.certification_row(),
            Some(row.certification_row())
        );
    }
}

#[test]
fn support_matrix_rejects_seam_key_collisions_between_crossings_and_closeout_rows() {
    let support = worth_query_lower_runtime_support_matrix();
    let mut seen = std::collections::BTreeSet::new();

    for row in support.rows() {
        assert!(
            seen.insert(row.seam_key().as_str().to_string()),
            "support row seam key `{}` must be unique across crossing and closeout rows",
            row.seam_key().as_str()
        );
    }
}

#[test]
fn seam_elimination_and_deferred_neighbors_share_one_support_lookup_surface() {
    let support = worth_query_lower_runtime_support_matrix();

    let eliminated = support
        .support_for(WorthQueryLowerRuntimeSeamKey::RuntimeIntentModule)
        .expect("eliminated seam should still be explainable through support");
    assert_eq!(
        eliminated.posture(),
        WorthQueryLowerRuntimeSupportPosture::SeamEliminated
    );

    let deferred = support
        .support_for(WorthQueryLowerRuntimeSeamKey::StoreBackedRouteParityNeighbor)
        .expect("deferred neighbor should be explainable through the same support surface");
    assert_eq!(
        deferred.posture(),
        WorthQueryLowerRuntimeSupportPosture::Deferred
    );
}

#[test]
fn adapter_and_reuse_rows_remain_admitted_support() {
    for classification in [
        WorthQueryLowerRuntimeCrossingClassification::CanonicalLowerRuntimeReuse,
        WorthQueryLowerRuntimeCrossingClassification::QueryBoundaryAdapter,
    ] {
        assert_eq!(
            support_posture_for_classification(classification),
            WorthQueryLowerRuntimeSupportPosture::Admitted
        );
    }
}
