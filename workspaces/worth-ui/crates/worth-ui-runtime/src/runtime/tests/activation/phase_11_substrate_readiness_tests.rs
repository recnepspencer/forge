#[test]
fn portal_activation_retains_typed_identity_and_receipt_owned_geometry() {
    let (_runtime, roots, receipt, committed_evidence) =
        super::production_catalog_activation_test_support::runtime_with_portal_catalog();
    let portal = committed_evidence
        .portal_anchors()
        .first()
        .expect("canonical committed evidence retains the portal row");
    assert_eq!(portal.identity().target().raw(), 44);
    assert_eq!(
        portal.identity().coordinate_space(),
        crate::evidence::UiMeasurementCoordinateSpace::PortalLayer
    );
    assert!(roots
        .iter()
        .any(|root| *root == receipt.identity().graph_node_identity()));
    assert_eq!(portal.receipt_identity(), receipt.identity());

    assert!(matches!(
        receipt.geometry_evidence().bounds(),
        crate::runtime::UiAllocationGeometryKnowledge::NotKnownAtAllocation
    ));
    let bounds = receipt
        .geometry_evidence()
        .portal_anchor_observation()
        .expect("portal commit retains its anchor observation separately")
        .observed_bounds();
    assert_eq!(
        (bounds.x(), bounds.y(), bounds.width(), bounds.height()),
        (1.0, 2.0, 3.0, 4.0)
    );
    assert_eq!(
        receipt.inspection_receipt().geometry(),
        receipt.geometry_evidence()
    );
}

#[test]
fn unknown_geometry_relationships_are_explicit_not_empty_folklore() {
    let (_runtime, _roots, receipt, _) =
        super::production_catalog_activation_test_support::runtime_with_portal_catalog();
    assert!(matches!(
        receipt.geometry_evidence().parent_edges(),
        crate::runtime::UiAllocationGeometryKnowledge::NotKnownAtAllocation
    ));
    assert!(matches!(
        receipt.geometry_evidence().sibling_edges(),
        crate::runtime::UiAllocationGeometryKnowledge::NotKnownAtAllocation
    ));
    assert!(matches!(
        receipt.geometry_evidence().spacing_relationship_ids(),
        crate::runtime::UiAllocationGeometryKnowledge::NotKnownAtAllocation
    ));
    assert!(matches!(
        receipt.geometry_evidence().baseline_relationships(),
        crate::runtime::UiAllocationGeometryKnowledge::NotKnownAtAllocation
    ));
}

#[test]
fn initial_activation_does_not_invent_scroll_or_viewport_evidence() {
    let (_runtime, _, _, _, _, _, scroll_aggregate, _) =
        super::production_catalog_activation_test_support::runtime_with_scroll_catalog();
    let (_runtime, _, _, aggregate) =
        super::production_catalog_activation_test_support::runtime_with_portal_catalog();
    assert!(scroll_aggregate.viewport().is_none());
    assert!(scroll_aggregate.scroll_owned().is_empty());
    assert!(scroll_aggregate.portal_anchors().is_empty());
    assert!(aggregate.viewport().is_none());
    assert!(aggregate.scroll_owned().is_empty());
    assert_eq!(aggregate.portal_anchors().len(), 1);
}
